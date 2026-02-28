import { onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useConsoleStore } from '../stores/consoleStore'
import { useDownloadStore } from '../stores/downloadStore'
import { useLaunchStore } from '../stores/launchStore'
import { useErrorStore } from '../stores/errorStore'
import { ConsoleLinePayload, DownloadProgressPayload, ErrorPayload, LaunchStatus, LaunchStatusPayload, LightyEvent } from '../types/lighty-events'
import consola from 'consola'

export function useLightyEvents() {
  const downloadStore = useDownloadStore()
  const launchStore = useLaunchStore()
  const consoleStore = useConsoleStore()
  const errorStore = useErrorStore()

  let unlistenFns: UnlistenFn[] = []

  async function setupListeners() {
    try {
      const unlistenDownload = await listen<DownloadProgressPayload>(
        LightyEvent.DownloadProgress,
        (event) => {
          downloadStore.updateProgress(event.payload)
        }
      )

      const unlistenStatus = await listen<LaunchStatusPayload>(
        LightyEvent.LaunchStatus,
        (event) => {
          consola.info('Launch status event received:', event.payload);
          launchStore.updateStatus(event.payload)

          if (event.payload.status === LaunchStatus.Running) {
            downloadStore.complete(event.payload.instance_name)
          }
        }
      )

      const unlistenConsole = await listen<ConsoleLinePayload[]>(
        LightyEvent.ConsoleOutput,
        (event) => {
          for (const log of event.payload) {
            consoleStore.addLog(log.pid, log)
          }
        }
      )

      const unlistenError = await listen<ErrorPayload>(
        LightyEvent.Error,
        (event) => {
          errorStore.setError(event.payload)
        }
      )

      unlistenFns = [
        unlistenDownload,
        unlistenStatus,
        unlistenConsole,
        unlistenError,
      ]
    } catch (error) {
      console.error('Failed to listen to download progress:', error)
      throw error
    }
  }

  async function cleanup(): Promise<void> {
    for (const unlisten of unlistenFns) {
      await unlisten()
    }
    unlistenFns = []
  }

  onMounted(() => {
    setupListeners()
  })

  onUnmounted(() => {
    cleanup()
  })

  return {
    setupListeners,
    cleanup,
  }
}
