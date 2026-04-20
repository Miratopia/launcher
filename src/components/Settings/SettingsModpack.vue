<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { Cpu, Monitor, ToggleLeft, ToggleRight, ChevronRight, ChevronDown, Loader2, Minus, Plus } from 'lucide-vue-next'
import { useLauncherStore } from '~/stores/launcherStore'
import { useSettingsCommand } from '~/composables/useSettingsCommand'
import type { Settings, JavaDistributionListItem } from '~/types/settings'

const props = defineProps<{
  modpackId: string
}>()

const store = useLauncherStore()
const { listJavaDistributions } = useSettingsCommand()

const javaDistributions: JavaDistributionListItem[] = listJavaDistributions()

/** RAM du modpack ouvert dans les réglages : si ce n’est pas le modpack sélectionné dans la barre, on n’utilise pas `store.memory` (réservé au panneau latéral). */
const ramIsSharedWithSidebar = computed(() => props.modpackId === store.selectedPack)

const otherGb = ref(8)
const otherMemoryInput = ref('8')

function mbToDisplayGb(mb: number): number {
  return Math.round((mb / 1024) * 10) / 10
}

function syncOtherRamFromLoadedSettings() {
  const s = store.modpackSettings
  if (!s) return
  if (s.maxMemory) {
    const gb = mbToDisplayGb(s.maxMemory)
    otherGb.value = gb
    otherMemoryInput.value = String(gb).replace('.', ',')
    return
  }
  const recommended = store.modpacks.find((p) => p.id === props.modpackId)?.info?.minecraft.recommendedMemory
  if (recommended) {
    const gb = mbToDisplayGb(recommended)
    otherGb.value = gb
    otherMemoryInput.value = String(gb).replace('.', ',')
  }
}

watch(
  () => [store.modpackSettingsLoading, store.loadedModpackSettingsId, store.modpackSettings] as const,
  () => {
    if (store.modpackSettingsLoading) return
    if (store.loadedModpackSettingsId !== props.modpackId) return
    if (ramIsSharedWithSidebar.value) return
    syncOtherRamFromLoadedSettings()
  },
)

onMounted(() => {
  store.loadModpackSettings(props.modpackId)
})

async function saveSettings(partial: Partial<Settings>) {
  const current = await store.getSettingsMergeBase(props.modpackId)
  await store.saveModpackSettings({ ...current, ...partial }, props.modpackId)
}

async function handleJavaChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value
  if (value) {
    await saveSettings({ javaDistribution: value as Settings['javaDistribution'] })
  }
}

async function handleMemoryChange() {
  const gb = ramIsSharedWithSidebar.value ? store.memory : otherGb.value
  await saveSettings({
    maxMemory: gb * 1024,
  })
}

function decreaseOtherRam() {
  const val = Math.round((otherGb.value - store.memoryStep) * 10) / 10
  if (val >= store.memoryMin) {
    otherGb.value = val
    otherMemoryInput.value = String(val).replace('.', ',')
  }
}

function increaseOtherRam() {
  const val = Math.round((otherGb.value + store.memoryStep) * 10) / 10
  if (val <= store.memoryMax) {
    otherGb.value = val
    otherMemoryInput.value = String(val).replace('.', ',')
  }
}

function updateOtherMemoryInput(input: string) {
  if (/^[0-9]*[,.]?[0-9]*$/.test(input)) {
    otherMemoryInput.value = input
    const normalized = input.replace(',', '.')
    const val = parseFloat(normalized)
    if (!isNaN(val) && val >= 0.5 && val <= 32) {
      otherGb.value = val
    }
  }
}

function blurOtherMemoryInput() {
  if (otherMemoryInput.value === '' || parseFloat(otherMemoryInput.value.replace(',', '.')) < 0.5) {
    otherGb.value = 2
    otherMemoryInput.value = '2'
  } else {
    otherMemoryInput.value = String(otherGb.value).replace('.', ',')
  }
}

async function onDecreaseRam() {
  if (ramIsSharedWithSidebar.value) {
    store.decreaseMemory()
  } else {
    decreaseOtherRam()
  }
  await handleMemoryChange()
}

async function onIncreaseRam() {
  if (ramIsSharedWithSidebar.value) {
    store.increaseMemory()
  } else {
    increaseOtherRam()
  }
  await handleMemoryChange()
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
          <!-- Même source que la barre latérale lorsque ce modpack est sélectionné -->
          <div v-if="ramIsSharedWithSidebar" class="flex items-center gap-3 mt-3">
            <button
              type="button"
              :disabled="store.memory <= store.memoryMin || store.isGameActive"
              class="w-7 h-7 shrink-0 flex items-center justify-center rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-white/60 hover:text-white disabled:opacity-30 disabled:cursor-not-allowed transition-all"
              @click="onDecreaseRam"
            >
              <Minus :size="14" />
            </button>
            <input
              type="text"
              :value="store.memoryInput"
              :disabled="store.isGameActive"
              class="w-20 input-field disabled:opacity-30 disabled:cursor-not-allowed"
              @input="store.updateMemoryInput(($event.target as HTMLInputElement).value)"
              @blur="store.blurMemoryInput(); handleMemoryChange()"
            />
            <span class="text-white/40 text-sm">Go</span>
            <button
              type="button"
              :disabled="store.memory >= store.memoryMax || store.isGameActive"
              class="w-7 h-7 shrink-0 flex items-center justify-center rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-white/60 hover:text-white disabled:opacity-30 disabled:cursor-not-allowed transition-all"
              @click="onIncreaseRam"
            >
              <Plus :size="14" />
            </button>
            <div class="flex-1 mx-2 h-2 bg-black/30 rounded-full overflow-hidden">
              <div
                class="h-full bg-gradient-to-r from-amber-500 to-orange-500 rounded-full transition-all"
                :style="{ width: `${Math.min((store.memory / 16) * 100, 100)}%` }"
              />
            </div>
            <span class="text-xs text-white/30">max 16 Go</span>
          </div>
          <!-- Modpack différent du sélectionné : état local, sans toucher à la RAM affichée dans la sidebar -->
          <div v-else class="flex items-center gap-3 mt-3">
            <button
              type="button"
              :disabled="otherGb <= store.memoryMin || store.isGameActive"
              class="w-7 h-7 shrink-0 flex items-center justify-center rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-white/60 hover:text-white disabled:opacity-30 disabled:cursor-not-allowed transition-all"
              @click="onDecreaseRam"
            >
              <Minus :size="14" />
            </button>
            <input
              type="text"
              :value="otherMemoryInput"
              :disabled="store.isGameActive"
              class="w-20 input-field disabled:opacity-30 disabled:cursor-not-allowed"
              @input="updateOtherMemoryInput(($event.target as HTMLInputElement).value)"
              @blur="blurOtherMemoryInput(); handleMemoryChange()"
            />
            <span class="text-white/40 text-sm">Go</span>
            <button
              type="button"
              :disabled="otherGb >= store.memoryMax || store.isGameActive"
              class="w-7 h-7 shrink-0 flex items-center justify-center rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-white/60 hover:text-white disabled:opacity-30 disabled:cursor-not-allowed transition-all"
              @click="onIncreaseRam"
            >
              <Plus :size="14" />
            </button>
            <div class="flex-1 mx-2 h-2 bg-black/30 rounded-full overflow-hidden">
              <div
                class="h-full bg-gradient-to-r from-amber-500 to-orange-500 rounded-full transition-all"
                :style="{ width: `${Math.min((otherGb / 16) * 100, 100)}%` }"
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
