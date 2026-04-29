import { defineStore } from 'pinia'
import { useModpacksCommand } from '~/composables/useModpacksCommand'
import { useSettingsCommand } from '~/composables/useSettingsCommand'
import { useAccountsStore } from './accountsStore'
import { useLaunchStore } from './launchStore'
import { useDownloadStore } from './downloadStore'
import { LaunchStatus } from '~/types/lighty-events'
import type { Settings } from '~/types/settings'
import type { ModpackInfo } from '~/types/modpack'
import consola from 'consola'

/** Mo → Go avec une décimale (évite Math.round(8.5) === 9 et conserve les entrées type 8,5 Go). */
function memoryMbToDisplayGb(mb: number): number {
  return Math.round((mb / 1024) * 10) / 10
}

export interface Modpack {
  id: string
  name: string
  version: string
  mods: number
  info: ModpackInfo | null
}

export interface Announcement {
  date: string
  title: string
  type: 'event' | 'update'
}

export type OptionalModsDialogMode = 'manage' | 'first-launch' | 'view-all'

export interface OptionalModsDialogState {
  show: boolean
  modpackId: string
  mode: OptionalModsDialogMode
}

export const useLauncherStore = defineStore('launcher', {
  state: () => ({
    memory: 8,
    memoryInput: '8',
    launching: false,
    selectedPack: '',
    showSettings: false,
    settingsTab: 'launcher' as string,

    fullscreen: false,
    resWidth: '1920',
    resHeight: '1080',
    autoUpdate: true,
    showConsole: false,
    closeLauncher: true,

    modpacks: [] as Modpack[],
    modpacksLoading: false,

    modpackSettings: null as Settings | null,
    /** Modpack dont les données dans `modpackSettings` correspondent (après dernier load/save réussi). */
    loadedModpackSettingsId: '' as string,
    modpackSettingsLoading: false,

    optionalModsDialog: {
      show: false,
      modpackId: '',
      mode: 'manage',
    } as OptionalModsDialogState,

    announcements: [
      { date: '30 avril 2026', title: 'Ouverture du serveur MiraLabs !', type: 'update' },
    ] as Announcement[],
  }),

  getters: {
    selectedModpack(): Modpack | undefined {
      return this.modpacks.find((p) => p.id === this.selectedPack)
    },

    activeSettingsModpackId(): string | null {
      if (this.settingsTab.startsWith('modpack:')) {
        return this.settingsTab.slice('modpack:'.length)
      }
      return null
    },

    settingsTitle(): string {
      if (this.settingsTab === 'launcher') return 'Launcher'
      if (this.settingsTab === 'accounts') return 'Comptes'
      const modpack = this.modpacks.find((p) => p.id === this.activeSettingsModpackId)
      return modpack?.name ?? ''
    },

    isGameActive(): boolean {
      const launchStore = useLaunchStore()
      const downloadStore = useDownloadStore()

      if (this.launching) return true
      if (!this.selectedPack) return false

      const packProgress = downloadStore.getStatusByInstance(this.selectedPack)
        ?? downloadStore.getStatusByInstance('')
      if (packProgress) return true

      const status = launchStore.currentStatus.get(this.selectedPack)
      if (status && ![LaunchStatus.Exited, LaunchStatus.Failed].includes(status.status)) return true

      return launchStore.isRunning(this.selectedPack)
    },

    memoryMin(): number {
      return 4
    },
    memoryMax(): number {
      return 16
    },
    memoryStep(): number {
      return 0.5
    },
  },

  actions: {
    async getSettingsMergeBase(forPackId: string): Promise<Settings> {
      if (this.loadedModpackSettingsId === forPackId && this.modpackSettings) {
        return this.modpackSettings
      }
      const { displayModpackSettings } = useSettingsCommand()
      return await displayModpackSettings(forPackId)
    },

    /**
     * Calcule la liste des chemins de fichiers optionnels activés par défaut
     * (i.e. déclarés `optional: true` ET `default: true` dans le modpack.json).
     * Retourne `undefined` si le modpack n'est pas (encore) chargé.
     */
    defaultOptionalPaths(modpackId: string): string[] | undefined {
      const info = this.modpacks.find((p) => p.id === modpackId)?.info
      if (!info) return undefined
      return info.files
        .filter((f) => f.optional && f.default)
        .map((f) => f.path)
    },

    /** Liste des fichiers optionnels disponibles pour un modpack (peu importe leur état). */
    optionalFilesFor(modpackId: string) {
      const info = this.modpacks.find((p) => p.id === modpackId)?.info
      if (!info) return []
      return info.files.filter((f) => f.optional)
    },

    /**
     * Initialise `optionalFiles` pour TOUS les modpacks chargés dans le store.
     *
     * - Pour chaque modpack, lit l'état actuel via `display_modpack_settings`.
     * - Si `optionalFiles` est déjà défini (array, même vide), ne touche à rien.
     * - Sinon, persiste la liste par défaut (chemins des fichiers `optional: true && default: true`).
     * - Ne touche jamais à `optionalFilesPrompted` afin que le dialogue de premier
     *   lancement reste déclenché à la première utilisation.
     *
     * Cette méthode est appelée automatiquement à la fin de `fetchModpacks()`.
     *
     * Note : côté backend, `Option::None` peut sérialiser en `null` ou être absent
     * selon la version du store ; on traite donc `null` ET `undefined` comme
     * "non initialisé" via `!= null`.
     */
    async ensureAllOptionalFilesInitialized() {
      if (this.modpacks.length === 0) return

      const { displayModpackSettings, updateModpackSettings } = useSettingsCommand()

      for (const pack of this.modpacks) {
        try {
          const current = await displayModpackSettings(pack.id)
          if (current.optionalFiles != null) continue

          const defaults = this.defaultOptionalPaths(pack.id)
          if (defaults === undefined) continue

          const updated = await updateModpackSettings(pack.id, {
            ...current,
            optionalFiles: defaults,
          })

          if (this.loadedModpackSettingsId === pack.id) {
            this.modpackSettings = updated
          }
        } catch (error) {
          consola.error(`Failed to initialize optionalFiles for "${pack.id}":`, error)
        }
      }
    },

    async selectPack(id: string) {
      this.selectedPack = id
      await this.loadModpackSettings(id)
    },

    openSettings(tab?: string) {
      if (tab) this.settingsTab = tab
      this.showSettings = true
      if (this.activeSettingsModpackId) {
        this.loadModpackSettings(this.activeSettingsModpackId)
      }
    },

    closeSettings() {
      const accounts = useAccountsStore()
      if (accounts.addingAccount) {
        accounts.cancelMicrosoftAuth()
      }
      this.showSettings = false
    },

    setMemory(value: number) {
      this.memory = value
      this.memoryInput = String(value).replace('.', ',')
    },

    decreaseMemory() {
      const val = Math.round((this.memory - this.memoryStep) * 10) / 10
      if (val >= this.memoryMin) this.setMemory(val)
    },

    increaseMemory() {
      const val = Math.round((this.memory + this.memoryStep) * 10) / 10
      if (val <= this.memoryMax) this.setMemory(val)
    },

    updateMemoryInput(input: string) {
      if (/^[0-9]*[,.]?[0-9]*$/.test(input)) {
        this.memoryInput = input
        const normalized = input.replace(',', '.')
        const val = parseFloat(normalized)
        if (!isNaN(val) && val >= 0.5 && val <= 32) {
          this.memory = val
        }
      }
    },

    blurMemoryInput() {
      if (this.memoryInput === '' || parseFloat(this.memoryInput.replace(',', '.')) < 0.5) {
        this.setMemory(2)
      } else {
        this.memoryInput = String(this.memory).replace('.', ',')
      }
    },

    async fetchModpacks() {
      const { listModpacks } = useModpacksCommand()
      try {
        this.modpacksLoading = true
        const result = await listModpacks()
        if (result.length > 0) {
          this.modpacks = result.map((info): Modpack => ({
            id: info.id,
            name: info.name,
            version: info.minecraft.version,
            mods: info.files.filter((f) => f.path.includes('mods/')).length,
            info,
          }))
          if (!this.selectedPack || !this.modpacks.find((p) => p.id === this.selectedPack)) {
            const defaultPack = this.modpacks.find((p) => p.info?.default)
            this.selectedPack = (defaultPack ?? this.modpacks[0]).id
          }
          await this.ensureAllOptionalFilesInitialized()
        } else {
          this.modpacks = []
          this.selectedPack = ''
        }
      } catch (error) {
        consola.error('Failed to fetch modpacks:', error)
        this.modpacks = []
        this.selectedPack = ''
      } finally {
        this.modpacksLoading = false
      }
    },

    async loadModpackSettings(modpackId?: string) {
      const id = modpackId ?? this.selectedPack
      if (!id) return
      const { displayModpackSettings } = useSettingsCommand()
      try {
        this.modpackSettingsLoading = true
        this.modpackSettings = await displayModpackSettings(id)
        this.loadedModpackSettingsId = id

        if (id === this.selectedPack) {
          if (this.modpackSettings?.maxMemory) {
            this.setMemory(memoryMbToDisplayGb(this.modpackSettings.maxMemory))
          } else {
            const recommended = this.modpacks.find((p) => p.id === id)?.info?.minecraft.recommendedMemory
            if (recommended) {
              this.setMemory(memoryMbToDisplayGb(recommended))
            }
          }
          if (this.modpackSettings?.fullScreen !== undefined) {
            this.fullscreen = this.modpackSettings.fullScreen
          }
          if (this.modpackSettings?.windowWidth) {
            this.resWidth = String(this.modpackSettings.windowWidth)
          }
          if (this.modpackSettings?.windowHeight) {
            this.resHeight = String(this.modpackSettings.windowHeight)
          }
        }
      } catch (error) {
        consola.error('Failed to load modpack settings:', error)
      } finally {
        this.modpackSettingsLoading = false
      }
    },

    openOptionalModsManager(modpackId?: string) {
      const id = modpackId ?? this.selectedPack
      if (!id) return
      this.optionalModsDialog = { show: true, modpackId: id, mode: 'manage' }
    },

    openAllModsViewer(modpackId: string) {
      this.optionalModsDialog = { show: true, modpackId, mode: 'view-all' }
    },

    closeOptionalModsDialog() {
      this.optionalModsDialog = { ...this.optionalModsDialog, show: false }
    },

    async saveModpackSettings(settings: Settings, modpackId?: string) {
      const id = modpackId ?? this.selectedPack
      if (!id) return
      const { updateModpackSettings } = useSettingsCommand()
      try {
        this.modpackSettings = await updateModpackSettings(id, settings)
        this.loadedModpackSettingsId = id
        if (id === this.selectedPack && this.modpackSettings?.maxMemory) {
          this.setMemory(memoryMbToDisplayGb(this.modpackSettings.maxMemory))
        }
      } catch (error) {
        consola.error('Failed to save modpack settings:', error)
        throw error
      }
    },

    async launchGame() {
      if (!this.selectedPack) return

      if (this.loadedModpackSettingsId !== this.selectedPack || !this.modpackSettings) {
        await this.loadModpackSettings(this.selectedPack)
      }

      const hasOptional = this.optionalFilesFor(this.selectedPack).length > 0
      const prompted = !!this.modpackSettings?.optionalFilesPrompted
      if (hasOptional && !prompted) {
        this.optionalModsDialog = {
          show: true,
          modpackId: this.selectedPack,
          mode: 'first-launch',
        }
        return
      }

      const { startModpack } = useModpacksCommand()
      try {
        this.launching = true
        await startModpack(this.selectedPack)
      } catch (error) {
        consola.error('Failed to launch game:', error)
        this.launching = false
      }
    },

    async init() {
      await this.fetchModpacks()
      if (this.selectedPack) {
        await this.loadModpackSettings()
      }
    },
  },
})
