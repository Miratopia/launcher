import { defineStore } from 'pinia'
import { ConsoleLinePayload } from '../types/lighty-events'

export const useConsoleStore = defineStore('console', {
  state: () => ({
    allLogs: [] as ConsoleLinePayload[],
  }),

  getters: {
    getAllLogs: (state) => state.allLogs,
  },

  actions: {
    addLog(log: ConsoleLinePayload): void {
      this.allLogs.push(log)
    },

    clearAllLogs(): void {
      this.allLogs = []
    },
  },
})
