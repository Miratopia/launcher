<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from 'vue'
import { Terminal } from 'lucide-vue-next'
import { StdStream } from '~/types/lighty-events'
import type { ConsoleLinePayload } from '~/types/lighty-events'

const BATCH_CHAR_LIMIT = 50_000
const VISIBLE_WINDOW = 5
const LINE_HEIGHT_PX = 18

const props = defineProps<{
  logs: ConsoleLinePayload[]
}>()

const batches = computed<ConsoleLinePayload[][]>(() => {
  const logs = props.logs
  if (!logs.length) return []

  const result: ConsoleLinePayload[][] = []
  let batch: ConsoleLinePayload[] = []
  let chars = 0

  for (const log of logs) {
    if (chars + log.line.length > BATCH_CHAR_LIMIT && batch.length) {
      result.push(batch)
      batch = []
      chars = 0
    }
    batch.push(log)
    chars += log.line.length
  }

  if (batch.length) result.push(batch)
  return result
})

const visibleStart = ref(0)
const visibleEnd = computed(() => Math.min(visibleStart.value + VISIBLE_WINDOW, batches.value.length))
const lastBatchStart = computed(() => Math.max(0, batches.value.length - VISIBLE_WINDOW))

function placeholderHeight(from: number, to: number): number {
  let h = 0
  for (let i = from; i < to; i++) h += (batches.value[i]?.length ?? 50) * LINE_HEIGHT_PX
  return h
}

const scrollContainer = ref<HTMLElement | null>(null)
const isAtBottom = ref(true)
const topSentinel = ref<HTMLElement | null>(null)
const bottomSentinel = ref<HTMLElement | null>(null)
let observer: IntersectionObserver | null = null

function onScroll() {
  if (!scrollContainer.value) return
  const { scrollTop, scrollHeight, clientHeight } = scrollContainer.value
  isAtBottom.value = scrollHeight - scrollTop - clientHeight < 50
}

function scrollToBottom(behavior: ScrollBehavior = 'instant') {
  nextTick(() => scrollContainer.value?.scrollTo({ top: scrollContainer.value.scrollHeight, behavior }))
}

function setupObserver() {
  if (!scrollContainer.value) return

  observer = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (!entry.isIntersecting) continue
        if (entry.target === topSentinel.value && visibleStart.value > 0)
          visibleStart.value = Math.max(0, visibleStart.value - 2)
        if (entry.target === bottomSentinel.value && visibleStart.value < lastBatchStart.value)
          visibleStart.value = Math.min(lastBatchStart.value, visibleStart.value + 2)
      }
    },
    { root: scrollContainer.value, rootMargin: '200px' },
  )

  if (topSentinel.value) observer.observe(topSentinel.value)
  if (bottomSentinel.value) observer.observe(bottomSentinel.value)
}

watch(() => props.logs.length, () => {
  if (!isAtBottom.value) return
  visibleStart.value = lastBatchStart.value
  scrollToBottom()
})

function formatTime(ts: number): string {
  const d = new Date(ts > 1e12 ? ts : ts * 1000)
  return `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}:${String(d.getSeconds()).padStart(2, '0')}.${String(d.getMilliseconds()).padStart(3, '0')}`
}

onMounted(() => {
  setupObserver()
  if (batches.value.length) {
    visibleStart.value = lastBatchStart.value
    scrollToBottom()
  }
})

onUnmounted(() => observer?.disconnect())

defineExpose({
  isAtBottom,
  scrollToBottom,
  visibleStart,
  visibleEnd,
  batches,
})
</script>

<template>
  <div
    ref="scrollContainer"
    class="flex-1 overflow-y-auto font-mono text-xs leading-relaxed"
    style="background-color: rgba(0, 0, 0, 0.6)"
    @scroll="onScroll"
  >
    <div v-if="visibleStart > 0" :style="{ height: placeholderHeight(0, visibleStart) + 'px' }" />
    <div ref="topSentinel" class="h-px" />

    <div v-for="i in (visibleEnd - visibleStart)" :key="visibleStart + i - 1">
      <div
        v-for="(log, j) in batches[visibleStart + i - 1]"
        :key="`${log.pid}-${log.timestamp}-${j}`"
        class="flex px-4 py-px hover:bg-white/[0.03]"
      >
        <span class="text-white/20 select-none shrink-0 w-24">{{ formatTime(log.timestamp) }}</span>
        <span
          class="whitespace-pre-wrap break-all"
          :class="log.stream === StdStream.Stderr ? 'text-red-400' : 'text-gray-300'"
        >{{ log.line }}</span>
      </div>
    </div>

    <div ref="bottomSentinel" class="h-px" />
    <div v-if="visibleEnd < batches.length" :style="{ height: placeholderHeight(visibleEnd, batches.length) + 'px' }" />

    <div v-if="!logs.length" class="flex flex-col items-center justify-center h-full text-white/20 gap-3">
      <Terminal :size="48" />
      <span class="text-sm">En attente de logs...</span>
    </div>
  </div>
</template>
