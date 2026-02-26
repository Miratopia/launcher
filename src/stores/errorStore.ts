import { defineStore } from 'pinia'
import { ErrorPayload } from '../types/lighty-events'

export const useErrorStore = defineStore('error', {
  state: () => ({
    errors: new Map<string, ErrorPayload[]>(),
  }),

  getters: {
    getErrorsByCategory: (state) => {
      return (category: string): ErrorPayload[] => {
        return state.errors.get(category) || []
      }
    },
  },

  actions: {
    setError(payload: ErrorPayload): void {
      const categoryErrors = this.errors.get(payload.category) || []
      categoryErrors.push(payload)
      this.errors.set(payload.category, categoryErrors)
    },
  },
})
