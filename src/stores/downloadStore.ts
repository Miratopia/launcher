import { defineStore } from 'pinia'
import { DownloadProgressPayload } from '../types/lighty-events'

export const useDownloadStore = defineStore('download', {
  state: () => ({
    currentStatus: new Map<string, DownloadProgressPayload>(),
  }),

  getters: {
    getStatusByInstance: (state) => {
      return (instance_name: string): DownloadProgressPayload | null => state.currentStatus.get(instance_name) || null
    },
  },

  actions: {
    updateProgress(payload: DownloadProgressPayload): void {
      console.log('Updating download progress with payload:', payload)
      this.currentStatus.set(payload.instance_name, payload)
    },

    complete(instance_name: string): void {
      this.currentStatus.delete(instance_name)
    }
  },
})
