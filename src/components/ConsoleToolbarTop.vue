<script setup lang="ts">
import { Terminal, Trash2, ArrowLeft, Upload, ExternalLink, Copy, Loader2 } from 'lucide-vue-next'

defineProps<{
  logCount: number
  batchCount: number
  visibleStart: number
  visibleEnd: number
  logUrl: string | null
  uploading: boolean
}>()

const emit = defineEmits<{
  clear: []
  upload: []
}>()
</script>

<template>
  <div
    class="flex items-center justify-between px-4 py-2 border-b border-white/5 shrink-0"
    style="background-color: rgba(0, 0, 0, 0.5)"
  >
    <div class="flex items-center gap-3">
      <NuxtLink to="/launcher" class="p-2 hover:bg-white/10 rounded-lg transition-colors text-white/40 hover:text-white">
        <ArrowLeft :size="16" />
      </NuxtLink>
      <Terminal :size="18" class="text-amber-400" />
      <span class="text-white/70 text-sm font-medium">Console</span>
      <span class="text-white/30 text-xs">
        {{ logCount }} lignes
        <template v-if="batchCount > 1">
          &middot; Page {{ visibleStart + 1 }}&ndash;{{ visibleEnd }} / {{ batchCount }}
        </template>
      </span>

      <a
        v-if="logUrl"
        :href="logUrl"
        target="_blank"
        class="flex items-center gap-1.5 px-2 py-1 bg-amber-500/10 border border-amber-500/20 rounded-lg text-amber-400 text-xs hover:bg-amber-500/20 transition-colors"
      >
        <ExternalLink :size="12" />
        <span class="max-w-[160px] truncate">{{ logUrl }}</span>
      </a>
    </div>

    <div class="flex items-center gap-1">
      <button
        class="p-2 rounded-lg transition-colors"
        :class="uploading
          ? 'text-white/20 cursor-wait'
          : 'hover:bg-white/10 text-white/40 hover:text-amber-400 hover:cursor-pointer'"
        :disabled="uploading || logCount === 0"
        title="Uploader sur mclo.gs"
        @click="emit('upload')"
      >
        <Loader2 v-if="uploading" :size="16" class="animate-spin" />
        <Upload v-else :size="16" />
      </button>
      <button
        class="p-2 hover:bg-white/10 rounded-lg transition-colors text-white/40 hover:text-red-400"
        title="Effacer les logs"
        @click="emit('clear')"
      >
        <Trash2 :size="16" />
      </button>
    </div>
  </div>
</template>
