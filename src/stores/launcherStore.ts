import { defineStore } from 'pinia'
import { useModpacksCommand } from '../composables/useModpacksCommand'
import { useSettingsCommand } from '../composables/useSettingsCommand'
import { useAccountsStore } from './accountsStore'
import { useLaunchStore } from './launchStore'
import { useDownloadStore } from './downloadStore'
import { LaunchStatus } from '../types/lighty-events'
import type { Settings } from '../types/settings'
import consola from 'consola'

export interface Modpack {
  id: string
  name: string
  version: string
  mods: number
}

export interface Announcement {
  date: string
  title: string
  type: 'event' | 'update'
}

export const useLauncherStore = defineStore('launcher', {
  state: () => ({
    memory: 8,
    memoryInput: '8',
    launching: false,
    selectedPack: '',
    showSettings: false,
    settingsTab: 'launcher' as string,

    wallpaper: '',

    fullscreen: false,
    resWidth: '1920',
    resHeight: '1080',
    autoUpdate: true,
    showConsole: false,
    closeLauncher: true,

    modpacks: [] as Modpack[],
    modpacksLoading: false,

    modpackSettings: null as Settings | null,
    modpackSettingsLoading: false,

    announcements: [
      { date: '27 juin 2025', title: "Nouvel événement : Attaquons tous ensemble l'Enderdragon !", type: 'event' },
      { date: '22 juin 2025', title: 'Reset de la map Survie ce vendredi !', type: 'update' },
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
      return 1
    },
  },

  actions: {
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
        if (Array.isArray(result) && result.length > 0) {
          this.modpacks = result.map((name: string) => ({
            id: name,
            name,
            version: '',
            mods: 0,
          }))
          if (!this.selectedPack || !this.modpacks.find((p) => p.id === this.selectedPack)) {
            this.selectedPack = this.modpacks[0].id
          }
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
        if (this.modpackSettings?.maxMemory) {
          this.setMemory(Math.round(this.modpackSettings.maxMemory / 1024))
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
      } catch (error) {
        consola.error('Failed to load modpack settings:', error)
      } finally {
        this.modpackSettingsLoading = false
      }
    },

    async saveModpackSettings(settings: Settings, modpackId?: string) {
      const id = modpackId ?? this.selectedPack
      if (!id) return
      const { updateModpackSettings } = useSettingsCommand()
      try {
        this.modpackSettings = await updateModpackSettings(id, settings)
      } catch (error) {
        consola.error('Failed to save modpack settings:', error)
        throw error
      }
    },

    async launchGame() {
      if (!this.selectedPack) return
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
