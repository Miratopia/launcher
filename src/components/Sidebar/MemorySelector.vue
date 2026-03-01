<script setup lang="ts">
import { Cpu, Minus, Plus } from 'lucide-vue-next'
import { useLauncherStore } from '../../stores/launcherStore'

const store = useLauncherStore()
</script>

<template>
  <div class="p-4 border-t border-white/5">
    <div class="flex items-center gap-3 bg-white/5 rounded-xl p-3 border border-white/5">
      <Cpu :size="16" class="text-amber-400/50" />
      <span class="text-xs text-white/40">RAM</span>
      <div class="flex-1 flex items-center justify-end gap-2">
        <button
          :disabled="store.memory <= store.memoryOptions[0]"
          class="w-7 h-7 flex items-center justify-center rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-white/60 hover:text-white disabled:opacity-30 disabled:cursor-not-allowed transition-all"
          @click="store.decreaseMemory()"
        >
          <Minus :size="14" />
        </button>
        <div class="relative w-16">
          <input
            type="text"
            :value="store.memoryInput"
            class="w-full text-center text-sm font-medium text-white bg-transparent border border-transparent hover:border-white/10 focus:border-amber-500/50 rounded-lg py-1 outline-none transition-all"
            @input="store.updateMemoryInput(($event.target as HTMLInputElement).value)"
            @blur="store.blurMemoryInput()"
          />
          <span class="absolute right-1 top-1/2 -translate-y-1/2 text-xs text-white/40 pointer-events-none">
            Go
          </span>
        </div>
        <button
          :disabled="store.memory >= store.memoryOptions[store.memoryOptions.length - 1]"
          class="w-7 h-7 flex items-center justify-center rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-white/60 hover:text-white disabled:opacity-30 disabled:cursor-not-allowed transition-all"
          @click="store.increaseMemory()"
        >
          <Plus :size="14" />
        </button>
      </div>
    </div>
  </div>
</template>
