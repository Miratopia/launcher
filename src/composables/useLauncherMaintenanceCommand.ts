import { invoke } from '@tauri-apps/api/core'
import consola from 'consola'

export function useLauncherMaintenanceCommand() {
  async function clearCache(): Promise<void> {
    try {
      await invoke('clear_cache')
    } catch (error) {
      consola.error('Failed to clear cache:', error)
      throw error
    }
  }

  async function resetAllSettings(): Promise<void> {
    try {
      await invoke('reset_all_settings')
    } catch (error) {
      consola.error('Failed to reset settings:', error)
      throw error
    }
  }

  async function clearAllAccounts(): Promise<void> {
    try {
      await invoke('clear_all_accounts')
    } catch (error) {
      consola.error('Failed to clear accounts:', error)
      throw error
    }
  }

  async function deleteAllModpacks(): Promise<void> {
    try {
      await invoke('delete_all_modpacks')
    } catch (error) {
      consola.error('Failed to delete modpacks:', error)
      throw error
    }
  }

  return {
    clearCache,
    resetAllSettings,
    clearAllAccounts,
    deleteAllModpacks,
  }
}
