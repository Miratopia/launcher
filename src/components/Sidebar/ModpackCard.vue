<script setup lang="ts">
import { Monitor } from 'lucide-vue-next'
import type { Modpack } from '../../stores/launcherStore'

const props = defineProps<{
  pack: Modpack
  selected: boolean
}>()

defineEmits<{
  select: [id: string]
}>()

console.log('pack', props.pack);
</script>

<template>
  <div
    :class="[
      'flex items-center gap-3 p-3 rounded-xl cursor-pointer transition-all duration-300',
      selected
        ? 'bg-gradient-to-r from-amber-500/15 to-transparent border border-amber-500/30 shadow-lg shadow-amber-500/5'
        : 'bg-white/5 border border-transparent hover:bg-white/10',
    ]"
    @click="$emit('select', props.pack.id)"
  >
    <div
      :class="[
        'w-10 h-10 rounded-lg flex items-center justify-center transition-all',
        selected
          ? 'bg-gradient-to-br from-amber-400 to-orange-500 shadow-lg shadow-amber-500/30'
          : 'bg-white/10',
      ]"
    >
      <Monitor :size="18" :class="selected ? 'text-black' : 'text-white/50'" />
    </div>
    <div class="flex-1 min-w-0">
      <p class="text-sm font-medium text-white/90 truncate">{{ pack.name }}</p>
      <p class="text-xs text-white/30">{{ pack.version }} &bull; {{ pack.mods }} mods</p>
    </div>
    <div v-if="selected" class="status-dot-selected" />
  </div>
</template>
