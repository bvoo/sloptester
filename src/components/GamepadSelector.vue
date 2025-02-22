<script setup lang="ts">
import { computed } from 'vue'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'

interface GamepadInfo {
  id: number
  name: string
  controller_type: string
  vendor_id?: number
  product_id?: number
}

const props = defineProps<{
  gamepads: GamepadInfo[]
  selectedGamepad: string
}>()

const emit = defineEmits<{
  (e: 'update:selectedGamepad', value: string): void
  (e: 'change'): void
}>()

function createGamepadValue(gamepad: GamepadInfo): string {
  return `${gamepad.id}:${gamepad.controller_type}`
}

function parseGamepadValue(value: string): { id: number, controller_type: string } | null {
  const [id, type] = value.split(':')
  return id && type ? { id: parseInt(id), controller_type: type } : null
}

function handleChange(value: string) {
  console.log('Selected gamepad value:', value)
  emit('update:selectedGamepad', value)
  emit('change')
}

const selectedDevice = computed(() => {
  const parsed = parseGamepadValue(props.selectedGamepad)
  if (!parsed) return null
  return props.gamepads.find(g => g.id === parsed.id && g.controller_type === parsed.controller_type)
})
</script>

<template>
  <div class="my-5 flex flex-col gap-4">
    <div class="flex items-center gap-4">
      <Select :model-value="selectedGamepad" @update:model-value="handleChange">
        <SelectTrigger class="w-[280px]">
          <SelectValue placeholder="Select a gamepad" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem v-for="gamepad in gamepads" 
            :key="createGamepadValue(gamepad)"
            :value="createGamepadValue(gamepad)"
          >
            {{ gamepad.name }} [{{ gamepad.controller_type.toUpperCase() }}]
          </SelectItem>
        </SelectContent>
      </Select>

      <div v-if="selectedDevice" class="text-sm text-muted-foreground">
        <template v-if="selectedDevice.vendor_id && selectedDevice.product_id">
          VID: {{ selectedDevice.vendor_id.toString(16).padStart(4, '0') }},
          PID: {{ selectedDevice.product_id.toString(16).padStart(4, '0') }}
        </template>
      </div>
    </div>
  </div>
</template>
