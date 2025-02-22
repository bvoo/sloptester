<script setup lang="ts">
import { ArrowUp, ArrowDown, ArrowLeft, ArrowRight } from 'lucide-vue-next'

defineProps<{
  up: boolean
  down: boolean
  left: boolean
  right: boolean
}>()
</script>

<template>
  <div class="grid grid-cols-[1fr_48px_1fr] grid-rows-[48px_48px_48px] w-[144px] gap-0">
    <div />
    <div 
      class="transition-colors rounded-t-md ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 flex items-center justify-center z-10"
      :class="up ? 'bg-accent text-primary-foreground hover:bg-primary/90 active:bg-accent' : 'bg-secondary hover:bg-secondary/80 active:bg-secondary/70'"
    >
      <ArrowUp class="w-5 h-5" />
    </div>
    <div />
    <div 
      class="h-full transition-colors rounded-l ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 flex items-center justify-center z-10"
      :class="left ? 'bg-accent text-primary-foreground hover:bg-primary/90 active:bg-accent' : 'bg-secondary hover:bg-secondary/80 active:bg-secondary/70'"
    >
      <ArrowLeft class="w-5 h-5" />
    </div>
    <div 
      class="h-full transition-colors bg-secondary hover:bg-secondary/80 active:bg-secondary/70 z-0"
    />
    <div 
      class="h-full transition-colors rounded-r ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 flex items-center justify-center z-10"
      :class="right ? 'bg-accent text-primary-foreground hover:bg-primary/90 active:bg-primary/70' : 'bg-secondary hover:bg-secondary/80 active:bg-secondary/70'"
    >
      <ArrowRight class="w-5 h-5" />
    </div>
    <div />
    <div 
      class="transition-colors rounded-b-md ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 flex items-center justify-center z-10"
      :class="down ? 'bg-accent text-primary-foreground hover:bg-primary/90 active:bg-primary/70' : 'bg-secondary hover:bg-secondary/80 active:bg-secondary/70'"
    >
      <ArrowDown class="w-5 h-5" />
    </div>
    <div />
  </div>
</template>