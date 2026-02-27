import { invoke } from "@tauri-apps/api/core"

export function useModpacksCommand() {
  async function startModpack(modpackName: string, profileName: string, javaDistribution: string) {
    try {
      await invoke('start_modpack', {
        modpackName,
        profileName,
        javaDistribution,
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
    startModpack,
    stopModpack,
  }
}
