import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { StdStream } from '~/types/lighty-events'
import type { ConsoleLinePayload } from '~/types/lighty-events'

export type ConsoleTab = 'live' | 'latest' | 'launcher'

function parseRawLog(raw: string): ConsoleLinePayload[] {
  return raw.split('\n').map((line) => ({
    instance_name: '',
    pid: 0,
    stream: StdStream.Stdout,
    line,
    timestamp: 0,
  }))
}

export const useConsoleStore = defineStore('console', {
  state: () => ({
    activeTab: 'live' as ConsoleTab,
    allLogs: [] as ConsoleLinePayload[],
    latestLogs: [] as ConsoleLinePayload[],
    launcherLogs: [] as ConsoleLinePayload[],
    latestLogsLoading: false,
    launcherLogsLoading: false,
    latestLogsError: null as string | null,
    launcherLogsError: null as string | null,
  }),

  getters: {
    currentLogs(state): ConsoleLinePayload[] {
      switch (state.activeTab) {
        case 'latest': return state.latestLogs
        case 'launcher': return state.launcherLogs
        default: return state.allLogs
      }
    },
  },

  actions: {
    addLog(log: ConsoleLinePayload): void {
      this.allLogs.push(log)
    },

    clearAllLogs(): void {
      this.allLogs = []
    },

    setActiveTab(tab: ConsoleTab): void {
      this.activeTab = tab
    },

    async loadLatestLogs(modpackId: string): Promise<void> {
      this.latestLogsLoading = true
      this.latestLogsError = null
      try {
        const raw = await invoke<string>('read_log_file', { logType: 'latest', modpackId })
        this.latestLogs = parseRawLog(raw)
      } catch (e) {
        this.latestLogsError = String(e)
        this.latestLogs = []
      } finally {
        this.latestLogsLoading = false
      }
    },

    async loadLauncherLogs(): Promise<void> {
      this.launcherLogsLoading = true
      this.launcherLogsError = null
      try {
        const raw = await invoke<string>('read_log_file', { logType: 'launcher' })
        this.launcherLogs = parseRawLog(raw)
      } catch (e) {
        this.launcherLogsError = String(e)
        this.launcherLogs = []
      } finally {
        this.launcherLogsLoading = false
      }
    },
  },
})
