import { invoke } from "@tauri-apps/api/core"

export function useModpacksCommand() {
  async function listModpacks(profileName: string): Promise<any> {
    try {
      const modpacks = await invoke('list_modpacks', { profileName })
      return modpacks
    } catch (error) {
      console.error('Failed to list modpacks:', error)
      throw error
    }
  }

  async function startModpack(modpackName: string, profileName: string) {
    try {
      await invoke('start_modpack', {
        modpackName,
        profileName,
      })
    } catch (error) {
      console.error('Failed to start modpack:', error)
      throw error
    }
  }

  async function stopModpack(instanceId: string) {
    try {
      await invoke('stop_modpack', { instanceId })
    } catch (error) {
      console.error('Failed to stop modpack:', error)
      throw error
    }
  }

  return {
    listModpacks,
    startModpack,
    stopModpack,
  }
}
