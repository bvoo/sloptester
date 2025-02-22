<script setup lang="ts">
import { Card, CardContent } from '@/components/ui/card'

defineProps<{
  label: string
  value: number
}>()
</script>

<template>
  <Card>
    <CardContent class="pt-6">
      <div class="font-medium mb-1.5">{{ label }}</div>
      <div class="font-mono mb-1.5 text-muted-foreground">{{ value?.toFixed(2) || '0.00' }}</div>
      <div class="h-5 bg-secondary rounded-full overflow-hidden">
        <div 
          class="h-full bg-accent transition-all duration-100" 
          :style="{ width: `${value * 100}%` }"
        ></div>
      </div>
    </CardContent>
  </Card>
</template>