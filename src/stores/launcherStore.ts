import { defineStore } from 'pinia'

export const useLauncherStore = defineStore('launcher', {
  state: () => ({
    memory: 8,
    memoryInput: '8',
    launching: false,
    selectedPack: 'smp',
    showSettings: false,
    settingsTab: 'launcher' as 'launcher' | 'modpack' | 'accounts',

    fullscreen: false,
    resWidth: '1920',
    resHeight: '1080',
    autoUpdate: true,
    showConsole: false,
    closeLauncher: true,

    modpacks: [
      { id: 'smp', name: 'Miratopia SMP', version: '1.20.4', mods: 42 },
      { id: 's1', name: 'Miratopia Saison 1', version: '1.19.2', mods: 38 },
    ] as Modpack[],

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

    async launchGame() {
      this.launching = true
      // TODO: invoke Tauri command
      setTimeout(() => {
        this.launching = false
      }, 3000)
    },
  },
})

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
