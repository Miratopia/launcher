<script setup lang="ts">
import { computed } from 'vue'
import { Play, Square } from 'lucide-vue-next'
import { useLauncherStore } from '../stores/launcherStore'
import { useDownloadStore } from '../stores/downloadStore'
import { useLaunchStore } from '../stores/launchStore'
import { useModpacksCommand } from '../composables/useModpacksCommand'
import { LaunchStatus } from '../types/lighty-events'

const store = useLauncherStore()
const downloadStore = useDownloadStore()
const launchStore = useLaunchStore()
const { stopModpack } = useModpacksCommand()

const downloadProgress = computed(() => {
  if (!store.selectedPack) return null
  return downloadStore.getStatusByInstance(store.selectedPack)
})

const isRunning = computed(() => {
  if (!store.selectedPack) return false
  return launchStore.isRunning(store.selectedPack)
})

const launchStatus = computed(() => {
  if (!store.selectedPack) return null
  return launchStore.currentStatus.get(store.selectedPack) ?? null
})

const statusText = computed(() => {
  if (downloadProgress.value) {
    const pct = Math.round(downloadProgress.value.percentage)
    return `${downloadProgress.value.message} (${pct}%)`
  }
  if (launchStatus.value) {
    switch (launchStatus.value.status) {
      case LaunchStatus.Installing: return 'Installation en cours...'
      case LaunchStatus.Downloading: return 'Téléchargement...'
      case LaunchStatus.Launched: return 'Lancement...'
      case LaunchStatus.Running: return 'En cours d\'exécution'
      case LaunchStatus.Exited: return 'Terminé'
      case LaunchStatus.Failed: return 'Échec du lancement'
      default: return ''
    }
  }
  if (store.launching) return 'Vérification des fichiers...'
  return 'Prêt à jouer'
})

const isBusy = computed(() =>
  store.launching || !!downloadProgress.value ||
  (!!launchStatus.value && ![LaunchStatus.Exited, LaunchStatus.Failed].includes(launchStatus.value.status))
)

async function handleStop() {
  if (!launchStatus.value) return
  await stopModpack(launchStatus.value.instance_name)
}
</script>

<template>
  <div class="p-6">
    <div
      class="backdrop-blur-sm rounded-2xl border border-white/5 p-4 flex flex-col gap-3"
      style="background-color: rgba(255, 255, 255, 0.02)"
    >
      <div class="flex items-center gap-4">
        <div class="flex-1">
          <p class="text-white/90 font-medium">
            {{ store.selectedModpack?.name || 'Aucun modpack' }}
          </p>
          <p class="text-white/30 text-sm">
            {{ statusText }}
          </p>
        </div>

        <button
          v-if="isRunning"
          class="px-6 py-3 bg-red-500/20 hover:bg-red-500/30 border border-red-500/30 rounded-xl font-semibold text-red-400 flex items-center gap-2 transition-all"
          @click="handleStop"
        >
          <Square :size="20" fill="currentColor" />
          Arrêter
        </button>
        <button
          v-else
          :disabled="isBusy || !store.selectedPack"
          :class="isBusy ? 'btn-play-loading' : 'btn-play'"
          @click="store.launchGame()"
        >
          <template v-if="isBusy">
            <div class="w-5 h-5 border-2 border-white/20 border-t-white/60 rounded-full animate-spin" />
            Lancement...
          </template>
          <template v-else>
            <Play :size="20" fill="currentColor" />
            Jouer
          </template>
        </button>
      </div>

      <!-- Download progress bar -->
      <div v-if="downloadProgress" class="w-full">
        <div class="h-1.5 bg-white/5 rounded-full overflow-hidden">
          <div
            class="h-full bg-gradient-to-r from-amber-500 to-orange-500 rounded-full transition-all duration-300"
            :style="{ width: `${downloadProgress.percentage}%` }"
          />
        </div>
      </div>
    </div>
  </div>
</template>
