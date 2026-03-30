<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { ArrowDown } from 'lucide-vue-next'
import { useConsoleStore } from '~/stores/consoleStore'
import { useLauncherStore } from '~/stores/launcherStore'
import { useAccountsStore } from '~/stores/accountsStore'
import type { ConsoleTab } from '~/stores/consoleStore'
import type ConsoleContent from '~/components/Console/ConsoleContent.vue'

definePageMeta({ layout: 'empty' })

const consoleStore = useConsoleStore()
const launcherStore = useLauncherStore()
const accountsStore = useAccountsStore()
const { activeTab, currentLogs } = storeToRefs(consoleStore)

const content = ref<InstanceType<typeof ConsoleContent> | null>(null)
const logUrl = ref<string | null>(null)
const uploading = ref(false)
const uploadCooldown = ref(false)

const isRefreshing = computed(() =>
  consoleStore.latestLogsLoading || consoleStore.launcherLogsLoading,
)

async function loadTabLogs(tab: ConsoleTab) {
  if (tab === 'latest') {
    const modpackId = launcherStore.selectedModpack?.id
    if (modpackId) await consoleStore.loadLatestLogs(modpackId)
  } else if (tab === 'launcher') {
    await consoleStore.loadLauncherLogs()
  }
}

function onTabChange(tab: ConsoleTab) {
  consoleStore.setActiveTab(tab)
  logUrl.value = null
  loadTabLogs(tab)
}

async function uploadLogs() {
  if (!currentLogs.value.length || uploading.value || uploadCooldown.value) return
  uploading.value = true
  try {
    const raw = currentLogs.value.map(l => l.line).join('\n')
    const packName = launcherStore.selectedModpack?.name ?? 'Unknown'
    const username = accountsStore.activeAccount?.username ?? 'Unknown'
    const tabLabel = activeTab.value.charAt(0).toUpperCase() + activeTab.value.slice(1)
    const body = new URLSearchParams()
    body.set('content', raw)
    body.set('source', `Miratopia ${packName} ${tabLabel} Logs (${username})`)
    const res = await fetch('https://api.mclo.gs/1/log', { method: 'POST', body })
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
  content.value.visibleStart = Math.max(0, content.value.batches.length - 5)
  content.value.isAtBottom = true
  content.value.scrollToBottom('smooth')
}

onMounted(async () => {
  if (!launcherStore.modpacks.length) {
    await launcherStore.fetchModpacks()
  }
  const savedPack = localStorage.getItem('selectedPack')
  if (savedPack && launcherStore.modpacks.some(p => p.id === savedPack)) {
    launcherStore.selectedPack = savedPack
  }
  loadTabLogs(activeTab.value)
})
</script>

<template>
  <div class="relative z-10 flex flex-col h-screen">
    <ConsoleToolbarTop
      :active-tab="activeTab"
      :log-count="currentLogs.length"
      :batch-count="content?.batches?.length ?? 0"
      :visible-start="content?.visibleStart ?? 0"
      :visible-end="content?.visibleEnd ?? 0"
      :log-url="logUrl"
      :uploading="uploading || uploadCooldown"
      :refreshing="isRefreshing"
      @clear="consoleStore.clearAllLogs()"
      @upload="uploadLogs"
      @refresh="loadTabLogs(activeTab)"
      @tab-change="onTabChange"
    />

    <ConsoleContent :key="activeTab" ref="content" :logs="currentLogs" />

    <Transition
      enter-active-class="transition duration-200 ease-out"
      enter-from-class="opacity-0 translate-y-4"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="transition duration-150 ease-in"
      leave-from-class="opacity-100 translate-y-0"
      leave-to-class="opacity-0 translate-y-4"
    >
      <button
        v-if="content && !content.isAtBottom && currentLogs.length > 0"
        class="absolute bottom-6 right-6 flex items-center gap-2 px-4 py-2 bg-amber-500/20 hover:bg-amber-500/30 border border-amber-500/30 rounded-lg text-amber-400 text-sm font-medium transition-all shadow-lg"
        @click="jumpToLatest"
      >
        <ArrowDown :size="14" />
        Derniers logs
      </button>
    </Transition>
  </div>
</template>
