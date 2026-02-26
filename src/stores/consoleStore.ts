import { defineStore } from 'pinia'
import { ConsoleLinePayload } from '../types/lighty-events'

export const useConsoleStore = defineStore('console', {
  state: () => ({
    logs: new Map<number, ConsoleLinePayload[]>(),
  }),

  getters: {
    getLogsByPid: (state) => {
      return (pid: number): ConsoleLinePayload[] => state.logs.get(pid) || []
    },

    getAllLogs: (state) => {
      const allLogs: ConsoleLinePayload[] = []

      state.logs.forEach((logs) => {
        allLogs.push(...logs)
      })

      return allLogs
    },
  },

  actions: {
    addLog(pid: number, log: ConsoleLinePayload): void {
      if (!this.logs.has(pid)) {
        this.logs.set(pid, [])
      }

      this.logs.get(pid)!.push(log)
    },

    clearLogs(pid: number): void {
      this.logs.delete(pid)
    },

    clearAllLogs(): void {
      this.logs.clear()
    },
  },
})
