import { invoke } from "@tauri-apps/api/core"

export function useModpacksCommand() {
  async function listModpacks(): Promise<any> {
    try {
      return await invoke('list_modpacks')
    } catch (error) {
      console.error('Failed to list modpacks:', error)
      // throw error
    }
    return []
  }

  async function startModpack(modpackName: string) {
    try {
      await invoke('start_modpack', {
        modpackName,
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

  async function openModpacksFolder() {
    try {
      await invoke('open_modpacks_folder')
    } catch (error) {
      console.error('Failed to open modpacks folder:', error)
      throw error
    }
  }

  return {
    listModpacks,
    startModpack,
    stopModpack,
    openModpacksFolder,
  }
}
