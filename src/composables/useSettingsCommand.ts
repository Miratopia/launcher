import { invoke } from "@tauri-apps/api/core"

export function useSettingsCommand() {
  async function displaySettings(modpackName: string) {
    try {
      const settings = await invoke('display_settings', { modpack_name: modpackName })
      return settings
    } catch (error) {
      console.error('Failed to display settings:', error)
      throw error
    }
  }

  async function updateSettings(modpackName: string, newSettings: any) {
    try {
      await invoke('update_settings', { modpack_name: modpackName, new_settings: newSettings })
    } catch (error) {
      console.error('Failed to update settings:', error)
      throw error
    }
  }

  return {
    displaySettings,
    updateSettings,
  }
}
