<script setup lang="ts">
import { ref } from 'vue'
import { storeToRefs } from 'pinia'
import { ArrowDown } from 'lucide-vue-next'
import { useConsoleStore } from '../stores/consoleStore'
import { useLauncherStore } from '../stores/launcherStore'
import { useAccountsStore } from '../stores/accountsStore'
import type ConsoleContent from '../components/ConsoleContent.vue'

definePageMeta({ layout: 'empty' })

const consoleStore = useConsoleStore()
const launcherStore = useLauncherStore()
const accountsStore = useAccountsStore()
const { allLogs } = storeToRefs(consoleStore)

const content = ref<InstanceType<typeof ConsoleContent> | null>(null)

const logUrl = ref<string | null>(null)
const uploading = ref(false)
const uploadCooldown = ref(false)

async function uploadLogs() {
  if (!allLogs.value.length || uploading.value || uploadCooldown.value) return
  uploading.value = true
  try {
    const raw = allLogs.value.map(l => l.line).join('\n')
    const packName = launcherStore.selectedModpack?.name ?? 'Unknown'
    const username = accountsStore.activeAccount?.username ?? 'Unknown'
    const body = new URLSearchParams()
    body.set('content', raw)
    body.set('source', `Miratopia ${packName} Logs (${username})`)
    const res = await fetch('https://api.mclo.gs/1/log', {
      method: 'POST',
      body,
    })
    const data = await res.json()
    if (data.success) {
      logUrl.value = data.url
      await navigator.clipboard.writeText(data.url)
    }
  } finally {
    uploading.value = false
    uploadCooldown.value = true
    setTimeout(() => { uploadCooldown.value = false }, 10_000)
  }
}

function jumpToLatest() {
  if (!content.value) return
  content.value.visibleStart = content.value.batches.length - 5 > 0
    ? content.value.batches.length - 5
    : 0
  content.value.isAtBottom = true
  content.value.scrollToBottom('smooth')
}
</script>

<template>
  <div class="relative z-10 flex flex-col h-screen">
    <ConsoleToolbarTop
      :log-count="allLogs.length"
      :batch-count="content?.batches?.length ?? 0"
      :visible-start="content?.visibleStart ?? 0"
      :visible-end="content?.visibleEnd ?? 0"
      :log-url="logUrl"
      :uploading="uploading || uploadCooldown"
      @clear="consoleStore.clearAllLogs()"
      @upload="uploadLogs"
    />

    <ConsoleContent ref="content" :logs="allLogs" />

    <Transition
      enter-active-class="transition duration-200 ease-out"
      enter-from-class="opacity-0 translate-y-4"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="transition duration-150 ease-in"
      leave-from-class="opacity-100 translate-y-0"
      leave-to-class="opacity-0 translate-y-4"
    >
      <button
        v-if="content && !content.isAtBottom && allLogs.length > 0"
        class="absolute bottom-6 right-6 flex items-center gap-2 px-4 py-2 bg-amber-500/20 hover:bg-amber-500/30 border border-amber-500/30 rounded-lg text-amber-400 text-sm font-medium transition-all shadow-lg"
        @click="jumpToLatest"
      >
        <ArrowDown :size="14" />
        Derniers logs
      </button>
    </Transition>
  </div>
</template>
