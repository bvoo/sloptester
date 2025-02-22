<script setup lang="ts">
import { Card, CardContent } from '@/components/ui/card'

defineProps<{
  label: string
  x: number
  y: number
}>()

function clampStickPosition(x: number, y: number): [number, number] {
  const magnitude = Math.sqrt(x * x + y * y);
  if (magnitude > 1) {
    return [x / magnitude, y / magnitude];
  }
  return [x, y];
}
</script>

<template>
  <Card>
    <CardContent class="flex flex-col items-center pt-6">
      <div class="font-medium mb-2.5">{{ label }}</div>
      <div class="relative w-[150px] h-[150px] my-2.5">
        <div class="absolute inset-0 border-2 border-border rounded-full"></div>
        <div 
          class="absolute w-5 h-5 bg-accent/30 rounded-full transform -translate-x-1/2 -translate-y-1/2 transition-all duration-100"
          :style="{ 
            left: `${(x + 1) * 50}%`, 
            top: `${(-y + 1) * 50}%` 
          }"
        ></div>
        <div 
          class="absolute w-5 h-5 bg-accent rounded-full transform -translate-x-1/2 -translate-y-1/2 transition-all duration-100"
          :style="{ 
            left: `${(clampStickPosition(x, y)[0] + 1) * 50}%`, 
            top: `${(-clampStickPosition(x, y)[1] + 1) * 50}%` 
          }"
        ></div>
      </div>
      <div class="font-mono text-center mt-2.5 space-y-1">
        <div>Raw - X: {{ x?.toFixed(2) || '0.00' }} Y: {{ y?.toFixed(2) || '0.00' }}</div>
        <div class="text-muted-foreground text-sm">
          Normalized - X: {{ clampStickPosition(x, y)[0].toFixed(2) }} Y: {{ clampStickPosition(x, y)[1].toFixed(2) }}
        </div>
      </div>
    </CardContent>
  </Card>
</template>
