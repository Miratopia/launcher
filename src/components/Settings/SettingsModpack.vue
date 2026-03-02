<script setup lang="ts">
import { onMounted } from 'vue'
import { Cpu, Monitor, ToggleLeft, ToggleRight, ChevronRight, ChevronDown, Loader2 } from 'lucide-vue-next'
import { useLauncherStore } from '../../stores/launcherStore'
import { useSettingsCommand } from '../../composables/useSettingsCommand'
import type { Settings, JavaDistributionListItem } from '../../types/settings'

const props = defineProps<{
  modpackId: string
}>()

const store = useLauncherStore()
const { listJavaDistributions } = useSettingsCommand()

const javaDistributions: JavaDistributionListItem[] = listJavaDistributions()

onMounted(() => {
  store.loadModpackSettings(props.modpackId)
})

async function saveSettings(partial: Partial<Settings>) {
  const current = store.modpackSettings ?? {}
  await store.saveModpackSettings({ ...current, ...partial }, props.modpackId)
}

async function handleJavaChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value
  if (value) {
    await saveSettings({ javaDistribution: value as Settings['javaDistribution'] })
  }
}

async function handleMemoryChange() {
  await saveSettings({
    maxMemory: store.memory * 1024,
  })
}

// async function saveDisplaySettings() {
//   await saveSettings({
//     fullScreen: store.fullscreen,
//     windowWidth: parseInt(store.resWidth) || 1920,
//     windowHeight: parseInt(store.resHeight) || 1080,
//   })
// }

// function toggleFullscreen() {
//   store.fullscreen = !store.fullscreen
//   saveDisplaySettings()
// }
</script>

<template>
  <div class="space-y-4">
    <div v-if="store.modpackSettingsLoading" class="flex items-center justify-center py-12">
      <Loader2 :size="24" class="text-amber-400 animate-spin" />
    </div>

    <template v-else>
      <!-- TODO: Résolution — pas encore pris en charge côté backend -->
      <SettingsSettingRow
        :icon="Monitor"
        title="Résolution de l'écran"
        description="Résolution appliquée au lancement"
      >
        <template #content>
          <div class="flex items-center gap-3 mt-3 opacity-40 pointer-events-none">
            <input
              v-model="store.resWidth"
              type="text"
              class="w-20 input-field"
              disabled
            />
            <span class="text-white/30">&times;</span>
            <input
              v-model="store.resHeight"
              type="text"
              class="w-20 input-field"
              disabled
            />
            <button
              :class="[
                'flex items-center gap-2 px-3 py-2 rounded-lg border transition-all',
                'bg-black/30 border-white/10 text-white/60',
              ]"
              disabled
            >
              <component :is="store.fullscreen ? ToggleRight : ToggleLeft" :size="18" />
              <span class="text-sm">Plein écran</span>
            </button>
          </div>
        </template>
      </SettingsSettingRow>

      <!-- RAM -->
      <SettingsSettingRow
        :icon="Cpu"
        title="Mémoire allouée"
        description="RAM dédiée à Minecraft"
      >
        <template #content>
          <div class="flex items-center gap-3 mt-3">
            <input
              type="text"
              :value="store.memoryInput"
              :disabled="store.isGameActive"
              class="w-20 input-field disabled:opacity-30 disabled:cursor-not-allowed"
              @input="store.updateMemoryInput(($event.target as HTMLInputElement).value)"
              @blur="store.blurMemoryInput(); handleMemoryChange()"
            />
            <span class="text-white/40 text-sm">Go</span>
            <div class="flex-1 mx-2 h-2 bg-black/30 rounded-full overflow-hidden">
              <div
                class="h-full bg-gradient-to-r from-amber-500 to-orange-500 rounded-full transition-all"
                :style="{ width: `${Math.min((store.memory / 16) * 100, 100)}%` }"
              />
            </div>
            <span class="text-xs text-white/30">max 16 Go</span>
          </div>
        </template>
      </SettingsSettingRow>

      <!-- Version Java -->
      <SettingsSettingRow
        :icon="ChevronRight"
        title="Distribution Java"
        description="Runtime Java utilisé pour le lancement"
      >
        <template #action>
          <div class="relative">
            <select
              :disabled="store.isGameActive"
              class="appearance-none px-3 py-2 pr-8 bg-black/30 border border-white/10 rounded-lg text-sm text-white/80 cursor-pointer focus:border-amber-500/50 outline-none transition-all disabled:opacity-30 disabled:cursor-not-allowed"
              :value="store.modpackSettings?.javaDistribution ?? ''"
              @change="handleJavaChange"
            >
              <option value="" disabled>Choisir</option>
              <option
                v-for="dist in javaDistributions"
                :key="dist.value"
                :value="dist.value"
              >
                {{ dist.label }}
              </option>
            </select>
            <ChevronDown
              :size="14"
              class="absolute right-2 top-1/2 -translate-y-1/2 text-white/40 pointer-events-none"
            />
          </div>
        </template>
      </SettingsSettingRow>
    </template>
  </div>
</template>
