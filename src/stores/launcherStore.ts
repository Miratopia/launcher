import { defineStore } from 'pinia'
import { useModpacksCommand } from '../composables/useModpacksCommand'
import { useSettingsCommand } from '../composables/useSettingsCommand'
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
    settingsTab: 'launcher' as 'launcher' | 'modpack' | 'accounts',

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

    memoryOptions(): number[] {
      return [2, 4, 6, 8, 10, 12, 16]
    },
  },

  actions: {
    selectPack(id: string) {
      this.selectedPack = id
    },

    openSettings(tab?: 'launcher' | 'modpack' | 'accounts') {
      if (tab) this.settingsTab = tab
      this.showSettings = true
      if (this.settingsTab === 'modpack' && this.selectedPack) {
        this.loadModpackSettings()
      }
    },

    closeSettings() {
      this.showSettings = false
    },

    setMemory(value: number) {
      this.memory = value
      this.memoryInput = String(value).replace('.', ',')
    },

    decreaseMemory() {
      const opts = this.memoryOptions
      const idx = opts.indexOf(this.memory)
      if (idx > 0) {
        this.setMemory(opts[idx - 1])
      } else if (idx === -1 && this.memory > opts[0]) {
        const lower = opts.filter((v) => v < this.memory)
        if (lower.length) this.setMemory(lower[lower.length - 1])
      }
    },

    increaseMemory() {
      const opts = this.memoryOptions
      const idx = opts.indexOf(this.memory)
      if (idx !== -1 && idx < opts.length - 1) {
        this.setMemory(opts[idx + 1])
      } else if (idx === -1 && this.memory < opts[opts.length - 1]) {
        const higher = opts.filter((v) => v > this.memory)
        if (higher.length) this.setMemory(higher[0])
      }
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
        }
      } catch (error) {
        consola.error('Failed to fetch modpacks:', error)
      } finally {
        this.modpacksLoading = false
      }
    },

    async loadModpackSettings() {
      if (!this.selectedPack) return
      const { displayModpackSettings } = useSettingsCommand()
      try {
        this.modpackSettingsLoading = true
        this.modpackSettings = await displayModpackSettings(this.selectedPack)
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

    async saveModpackSettings(settings: Settings) {
      if (!this.selectedPack) return
      const { updateModpackSettings } = useSettingsCommand()
      try {
        this.modpackSettings = await updateModpackSettings(this.selectedPack, settings)
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
