<script setup lang="ts">
import type { Component } from 'vue'

defineProps<{
  icon: Component
  title: string
  description: string
}>()
</script>

<template>
  <div class="surface-card-hover rounded-xl p-4">
    <div class="flex items-start gap-4">
      <div class="icon-box">
        <component :is="icon" :size="20" class="text-amber-400" />
      </div>
      <div class="flex-1">
        <h3 class="text-sm font-medium text-white">{{ title }}</h3>
        <p class="text-xs text-white/40 mt-0.5">{{ description }}</p>
        <slot name="content" />
      </div>
      <slot name="action" />
    </div>
  </div>
</template>
