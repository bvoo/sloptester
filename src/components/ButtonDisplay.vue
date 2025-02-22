<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import DPad from './DPad.vue'

defineProps<{
  buttons: boolean[]
}>()

const buttonLabels = [
  'A', 'B', 'X', 'Y',
  'LB', 'RB', 'Back', 'Start',
  'Guide', 'L3', 'R3'
]
</script>

<template>
  <div class="flex gap-4 items-stretch">
    <Card class="flex-1">
      <CardHeader>
        <CardTitle>Buttons</CardTitle>
      </CardHeader>
      <CardContent>
        <div class="grid grid-cols-4 gap-2.5">
          <Button
            v-for="(pressed, index) in buttons.slice(0, 11)"
            :key="index"
            :variant="pressed ? 'default' : 'outline'"
            class="h-14"
          >
            {{ buttonLabels[index] }}
          </Button>
        </div>
      </CardContent>
    </Card>

    <Card class="w-[240px] flex flex-col">
      <CardHeader>
        <CardTitle>D-Pad</CardTitle>
      </CardHeader>
      <CardContent class="flex-1 flex items-center justify-center">
        <DPad 
          :up="buttons[11]"
          :down="buttons[12]"
          :left="buttons[13]"
          :right="buttons[14]"
        />
      </CardContent>
    </Card>
  </div>
</template>
