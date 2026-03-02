<script setup lang="ts">
import { Cpu, Minus, Plus } from 'lucide-vue-next'
import { useLauncherStore } from '../../stores/launcherStore'

const store = useLauncherStore()

async function saveMemory() {
  const current = store.modpackSettings ?? {}
  await store.saveModpackSettings({ ...current, maxMemory: store.memory * 1024 })
}

async function onDecrease() {
  store.decreaseMemory()
  await saveMemory()
}

async function onIncrease() {
  store.increaseMemory()
  await saveMemory()
}

async function onBlur() {
  store.blurMemoryInput()
  await saveMemory()
}
</script>

<template>
  <div class="p-4 border-t border-white/5">
    <div class="flex items-center gap-3 bg-white/5 rounded-xl p-3 border border-white/5">
      <Cpu :size="16" class="text-amber-400/50" />
      <span class="text-xs text-white/40">RAM</span>
      <div class="flex-1 flex items-center justify-end gap-2">
        <button
          :disabled="store.memory <= store.memoryMin || store.isGameActive"
          class="w-7 h-7 flex items-center justify-center rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-white/60 hover:text-white disabled:opacity-30 disabled:cursor-not-allowed transition-all"
          @click="onDecrease"
        >
          <Minus :size="14" />
        </button>
        <div class="relative w-16">
          <input
            type="text"
            :value="store.memoryInput"
            :disabled="store.isGameActive"
            class="w-full text-center text-sm font-medium text-white bg-transparent border border-transparent hover:border-white/10 focus:border-amber-500/50 rounded-lg py-1 outline-none transition-all disabled:opacity-30 disabled:cursor-not-allowed"
            @input="store.updateMemoryInput(($event.target as HTMLInputElement).value)"
            @blur="onBlur"
          />
          <span class="absolute right-1 top-1/2 -translate-y-1/2 text-xs text-white/40 pointer-events-none">
            Go
          </span>
        </div>
        <button
          :disabled="store.memory >= store.memoryMax || store.isGameActive"
          class="w-7 h-7 flex items-center justify-center rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-white/60 hover:text-white disabled:opacity-30 disabled:cursor-not-allowed transition-all"
          @click="onIncrease"
        >
          <Plus :size="14" />
        </button>
      </div>
    </div>
  </div>
</template>
