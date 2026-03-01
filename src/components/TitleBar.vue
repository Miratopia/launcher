<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { Minus, Maximize2, Square, X } from 'lucide-vue-next'
import { getCurrentWindow } from '@tauri-apps/api/window'

const appWindow = getCurrentWindow()
const isMaximized = ref(false)

async function syncMaximizedState() {
  isMaximized.value = await appWindow.isMaximized()
}

async function minimize() {
  await appWindow.minimize()
}

async function toggleMaximize() {
  await appWindow.toggleMaximize()
  await syncMaximizedState()
}

async function close() {
  await appWindow.close()
}

onMounted(async () => {
  await syncMaximizedState()

  const unlisten = await appWindow.onResized(async () => {
    await syncMaximizedState()
  })

  onUnmounted(() => {
    unlisten()
  })
})
</script>

<template>
  <div
    class="relative z-10 flex items-center justify-between px-4 py-2 backdrop-blur-sm border-b border-white/5"
    style="background-color: rgba(0, 0, 0, 0.5)"
    data-tauri-drag-region
  >
    <div class="flex items-center gap-3" data-tauri-drag-region>
      <img src="~/assets/images/logo.svg" alt="Miratopia" class="w-8 h-8 rounded-lg" />
      <span class="text-white/50 text-sm font-medium" data-tauri-drag-region>Miratopia Launcher</span>
      <span class="text-white/20 text-xs" data-tauri-drag-region>v0.1.10</span>
    </div>
    <div class="flex items-center gap-1">
      <button class="btn-window" @click="minimize">
        <Minus :size="14" />
      </button>
      <button class="btn-window" @click="toggleMaximize">
        <Square v-if="isMaximized" :size="12" />
        <Maximize2 v-else :size="14" />
      </button>
      <button
        class="p-2 hover:bg-red-500/80 rounded-lg transition-colors text-white/40 hover:text-white"
        @click="close"
      >
        <X :size="14" />
      </button>
    </div>
  </div>
</template>
