import { defineStore } from 'pinia'
import { LaunchStatus, LaunchStatusPayload } from '../types/lighty-events'

export const useLaunchStore = defineStore('launch', {
  state: () => ({
    currentStatus: new Map<string, LaunchStatusPayload>(),
  }),

  getters: {
    isRunning: (state) => {
      return (instanceName: string): boolean => {
        const status = state.currentStatus.get(instanceName)
        return status?.status === LaunchStatus.Running
      }
    },
  },

  actions: {
    updateStatus(payload: LaunchStatusPayload): void {
      this.currentStatus.set(payload.instance_name, payload)
    },
  },
})
