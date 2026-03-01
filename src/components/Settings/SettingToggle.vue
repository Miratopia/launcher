<script setup lang="ts">
import type { Component } from 'vue'

defineProps<{
  icon: Component
  title: string
  description: string
  modelValue: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
}>()
</script>

<template>
  <div class="surface-card-hover rounded-xl p-4">
    <div class="flex items-center gap-4">
      <div class="icon-box">
        <component :is="icon" :size="20" class="text-amber-400" />
      </div>
      <div class="flex-1">
        <h3 class="text-sm font-medium text-white">{{ title }}</h3>
        <p class="text-xs text-white/40 mt-0.5">{{ description }}</p>
      </div>
      <button
        :class="modelValue ? 'toggle-track-on' : 'toggle-track-off'"
        @click="emit('update:modelValue', !modelValue)"
      >
        <div :class="['toggle-thumb', modelValue ? 'translate-x-5' : 'translate-x-0']" />
      </button>
    </div>
  </div>
</template>
