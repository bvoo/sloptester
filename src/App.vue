<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import GamepadSelector from '@/components/GamepadSelector.vue'
import ButtonDisplay from '@/components/ButtonDisplay.vue'
import AnalogStick from '@/components/AnalogStick.vue'
import TriggerDisplay from '@/components/TriggerDisplay.vue'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'

interface GamepadInfo {
  id: number
  name: string
  controller_type: string
}

interface GamepadState {
  buttons: boolean[]
  axes: number[]
}

const gamepads = ref<GamepadInfo[]>([])
const selectedGamepad = ref('')
const gamepadState = ref<GamepadState>({
  buttons: [],
  axes: []
})

function parseGamepadValue(value: string): { id: number, controller_type: string } | null {
  const [id, type] = value.split(':')
  return id && type ? { id: parseInt(id), controller_type: type } : null
}

async function updateGamepads() {
  try {
    gamepads.value = await invoke('get_gamepads')
  } catch (error) {
    console.error('Failed to get gamepads:', error)
  }
}

async function updateGamepadState() {
  if (!selectedGamepad.value) return
  
  try {
    const parsed = parseGamepadValue(selectedGamepad.value)
    if (!parsed) {
      console.error('Invalid gamepad value format')
      return
    }

    const currentGamepad = gamepads.value.find(g => g.id === parsed.id && g.controller_type === parsed.controller_type)
    
    if (!currentGamepad) {
      console.error('Selected gamepad not found')
      return
    }

    console.log(`Requesting state for ${currentGamepad.name} (${currentGamepad.controller_type}) with ID ${parsed.id}`)
    
    const state = await invoke<GamepadState>('get_gamepad_state', { 
      id: parsed.id,
      controller_type: parsed.controller_type 
    })
    if (state) {
      gamepadState.value = state
    } else {
      console.warn('No state received for gamepad:', currentGamepad)
    }
  } catch (error) {
    console.error('Failed to get gamepad state:', error)
  }
}

function handleGamepadChange() {
  console.log('Gamepad selection changed:', selectedGamepad.value)
  updateGamepadState()
}

let frameId: number | undefined
let pollInterval: number | undefined

onMounted(async () => {
  // Initial gamepad scan
  await updateGamepads()
  
  // Poll for new gamepads every 2 seconds
  pollInterval = window.setInterval(updateGamepads, 2000)
  
  // Update gamepad state on each animation frame
  function animate() {
    if (selectedGamepad.value) {
      updateGamepadState()
    }
    frameId = requestAnimationFrame(animate)
  }
  animate()
})

onUnmounted(() => {
  if (frameId !== undefined) {
    cancelAnimationFrame(frameId)
  }
  if (pollInterval !== undefined) {
    clearInterval(pollInterval)
  }
})
</script>

<template>
  <div class="min-h-screen bg-background text-foreground">
    <div class="max-w-3xl mx-auto p-5">
      <h1 class="text-center text-2xl font-bold mb-6">sloptester</h1>
      
      <GamepadSelector
        v-model:selectedGamepad="selectedGamepad"
        :gamepads="gamepads"
        @change="handleGamepadChange"
      />

      <div v-if="selectedGamepad" class="gamepad-display space-y-5">
        <ButtonDisplay :buttons="gamepadState.buttons" />

        <Card>
          <CardHeader>
            <CardTitle>Controls</CardTitle>
          </CardHeader>
          <CardContent>
            <div class="grid grid-cols-2 gap-5 mb-5">
              <AnalogStick
                label="Left Stick"
                :x="gamepadState.axes[0] || 0"
                :y="gamepadState.axes[1] || 0"
              />
              <AnalogStick
                label="Right Stick"
                :x="gamepadState.axes[2] || 0"
                :y="gamepadState.axes[3] || 0"
              />
            </div>

            <div class="grid grid-cols-2 gap-5">
              <TriggerDisplay
                label="Left Trigger"
                :value="gamepadState.axes[4] || 0"
              />
              <TriggerDisplay
                label="Right Trigger"
                :value="gamepadState.axes[5] || 0"
              />
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  </div>
</template>

<style>
@import "tailwindcss";
</style>