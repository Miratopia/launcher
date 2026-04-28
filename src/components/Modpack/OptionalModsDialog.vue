<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { Puzzle, List, X, Loader2, Play, Save, Search } from 'lucide-vue-next'
import { useLauncherStore } from '~/stores/launcherStore'
import { useModpacksCommand } from '~/composables/useModpacksCommand'
import type { FileModpackInfo } from '~/types/modpack'
import consola from 'consola'

const store = useLauncherStore()
const { startModpack } = useModpacksCommand()

const dialog = computed(() => store.optionalModsDialog)
const show = computed(() => dialog.value.show)
const modpackId = computed(() => dialog.value.modpackId)
const mode = computed(() => dialog.value.mode)

const modpack = computed(() =>
  store.modpacks.find((p) => p.id === modpackId.value),
)

const isViewAll = computed(() => mode.value === 'view-all')

const allMods = computed<FileModpackInfo[]>(() => modpack.value?.info?.files ?? [])

const optionalMods = computed<FileModpackInfo[]>(() =>
  allMods.value.filter((f) => f.optional),
)

const sourceMods = computed(() => isViewAll.value ? allMods.value : optionalMods.value)

const draftPaths = ref<string[]>([])
const saving = ref(false)
const search = ref('')

const filteredMods = computed(() => {
  const q = search.value.trim().toLowerCase()
  const list = q
    ? sourceMods.value.filter((mod) => {
        const name = (mod.name?.trim() || mod.path).toLowerCase()
        const desc = (mod.description ?? '').toLowerCase()
        return name.includes(q) || desc.includes(q)
      })
    : [...sourceMods.value]

  return list.sort((a, b) => Number(a.optional ?? false) - Number(b.optional ?? false))
})

const settingsAreFresh = computed(() =>
  store.modpackSettings !== null
  && store.loadedModpackSettingsId === modpackId.value
  && !store.modpackSettingsLoading,
)

const isReady = computed(() => isViewAll.value || settingsAreFresh.value)

const title = computed(() => {
  if (mode.value === 'view-all') return 'Tous les mods'
  if (mode.value === 'first-launch') return 'Mods optionnels'
  return 'Gérer les mods optionnels'
})

const subtitle = computed(() => {
  if (mode.value === 'view-all') return 'Liste complète des mods de ce modpack.'
  if (mode.value === 'first-launch') return 'Choisissez les mods optionnels à installer pour ce modpack. Vous pourrez modifier votre sélection à tout moment depuis les paramètres.'
  return 'Activez ou désactivez les mods optionnels. Les changements seront appliqués au prochain lancement.'
})

watch(
  [show, settingsAreFresh, modpackId],
  ([isOpen, isFresh, id]) => {
    if (!isOpen || !id) return
    search.value = ''
    if (isViewAll.value) return
    if (isFresh) {
      draftPaths.value = [...(store.modpackSettings?.optionalFiles ?? [])]
    } else {
      store.loadModpackSettings(id)
    }
  },
  { immediate: true },
)

function close() {
  store.closeOptionalModsDialog()
}

async function persist(partial: Partial<{ optionalFiles: string[]; optionalFilesPrompted: boolean }>) {
  const current = await store.getSettingsMergeBase(modpackId.value)
  await store.saveModpackSettings({ ...current, ...partial }, modpackId.value)
}

async function onSaveManage() {
  if (saving.value) return
  saving.value = true
  try {
    await persist({ optionalFiles: [...draftPaths.value] })
    close()
  } catch (error) {
    consola.error('Failed to save optional mods selection:', error)
  } finally {
    saving.value = false
  }
}

async function onConfirmAndPlay() {
  if (saving.value) return
  saving.value = true
  try {
    await persist({
      optionalFiles: [...draftPaths.value],
      optionalFilesPrompted: true,
    })
    close()
    await launch()
  } catch (error) {
    consola.error('Failed to save and launch:', error)
  } finally {
    saving.value = false
  }
}

async function onLater() {
  if (saving.value) return
  saving.value = true
  try {
    await persist({ optionalFilesPrompted: true })
    close()
    await launch()
  } catch (error) {
    consola.error('Failed to defer optional mods selection:', error)
  } finally {
    saving.value = false
  }
}

async function launch() {
  try {
    store.launching = true
    await startModpack(modpackId.value)
  } catch (error) {
    consola.error('Failed to launch game:', error)
    store.launching = false
  }
}
</script>

<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition duration-200 ease-out"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition duration-150 ease-in"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="show"
        class="fixed inset-0 z-[110] flex items-center justify-center p-8 surface-overlay"
        @click.self="close"
      >
        <Transition
          enter-active-class="transition duration-200 ease-out"
          enter-from-class="opacity-0 scale-95"
          enter-to-class="opacity-100 scale-100"
          leave-active-class="transition duration-150 ease-in"
          leave-from-class="opacity-100 scale-100"
          leave-to-class="opacity-0 scale-95"
        >
          <div
            v-if="show"
            class="w-full max-w-2xl surface-modal flex flex-col"
            style="height: 85vh"
          >
            <div class="px-5 py-4 border-b border-white/5 flex items-center justify-between shrink-0">
              <div class="flex items-center gap-3 min-w-0">
                <div class="icon-box">
                  <component :is="isViewAll ? List : Puzzle" :size="18" class="text-amber-400" />
                </div>
                <div class="min-w-0">
                  <h2 class="text-base font-semibold text-white truncate">{{ title }}</h2>
                  <p v-if="modpack" class="text-xs text-white/40 truncate">
                    {{ modpack.name }}
                  </p>
                </div>
              </div>
              <button
                class="btn-window"
                aria-label="Fermer"
                @click="close"
              >
                <X :size="18" />
              </button>
            </div>

            <div class="px-5 py-4 border-b border-white/5 shrink-0 space-y-3">
              <p class="text-sm text-white/50 leading-relaxed">{{ subtitle }}</p>
              <div class="relative">
                <Search :size="14" class="absolute left-3 top-1/2 -translate-y-1/2 text-white/30 pointer-events-none" />
                <input
                  v-model="search"
                  type="text"
                  placeholder="Rechercher un mod…"
                  class="w-full pl-8 pr-8 py-2 text-sm bg-white/5 border border-white/10 rounded-lg text-white placeholder-white/30 outline-none focus:border-amber-500/50 transition-colors"
                />
                <button
                  v-if="search"
                  type="button"
                  class="absolute right-2.5 top-1/2 -translate-y-1/2 text-white/30 hover:text-white/70 transition-colors"
                  aria-label="Effacer la recherche"
                  @click="search = ''"
                >
                  <X :size="14" />
                </button>
              </div>
            </div>

            <div class="flex-1 overflow-y-auto p-5">
              <div
                v-if="!isReady"
                class="flex items-center justify-center py-12"
              >
                <Loader2 :size="24" class="text-amber-400 animate-spin" />
              </div>

              <ModpackModList
                v-else
                :mods="filteredMods"
                :selected-paths="draftPaths"
                :disabled="saving"
                :updatable="!isViewAll"
                :empty-message="search ? 'Aucun mod ne correspond à votre recherche.' : (isViewAll ? 'Ce modpack ne contient aucun mod.' : 'Aucun mod optionnel pour ce modpack.')"
                @update:selected-paths="draftPaths = $event"
              />
            </div>

            <div class="px-5 py-4 border-t border-white/5 flex items-center justify-end gap-3 shrink-0">
              <template v-if="mode === 'view-all'">
                <button
                  class="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-sm text-white/70 hover:text-white transition-all"
                  @click="close"
                >
                  Fermer
                </button>
              </template>

              <template v-else-if="mode === 'first-launch'">
                <button
                  class="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-sm text-white/70 hover:text-white transition-all disabled:opacity-30 disabled:cursor-not-allowed"
                  :disabled="saving || !settingsAreFresh"
                  @click="onLater"
                >
                  Plus tard
                </button>
                <button
                  class="px-4 py-2 rounded-lg text-sm font-medium bg-amber-500/20 hover:bg-amber-500/30 border border-amber-500/30 text-amber-400 hover:text-amber-300 transition-all disabled:opacity-30 disabled:cursor-not-allowed flex items-center gap-2"
                  :disabled="saving || !settingsAreFresh"
                  @click="onConfirmAndPlay"
                >
                  <Loader2 v-if="saving" :size="14" class="animate-spin" />
                  <Play v-else :size="14" fill="currentColor" />
                  Confirmer et jouer
                </button>
              </template>

              <template v-else>
                <button
                  class="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-sm text-white/70 hover:text-white transition-all disabled:opacity-30 disabled:cursor-not-allowed"
                  :disabled="saving"
                  @click="close"
                >
                  Annuler
                </button>
                <button
                  class="px-4 py-2 rounded-lg text-sm font-medium bg-amber-500/20 hover:bg-amber-500/30 border border-amber-500/30 text-amber-400 hover:text-amber-300 transition-all disabled:opacity-30 disabled:cursor-not-allowed flex items-center gap-2"
                  :disabled="saving || !settingsAreFresh"
                  @click="onSaveManage"
                >
                  <Loader2 v-if="saving" :size="14" class="animate-spin" />
                  <Save v-else :size="14" />
                  Enregistrer
                </button>
              </template>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>
