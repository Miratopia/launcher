import { invoke } from '@tauri-apps/api/core'

export interface UseUtilsCommand {
  osTotalMemoryInfo: () => Promise<number>,
}

export function useUtilsCommand(): UseUtilsCommand {

  function osTotalMemoryInfo(): Promise<number> {
    return invoke<number>('os_total_memory_info')
  }

  return {
    osTotalMemoryInfo,
  }
}
