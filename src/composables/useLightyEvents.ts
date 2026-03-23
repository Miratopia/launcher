import { onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useConsoleStore } from '../stores/consoleStore'
import { useDownloadStore } from '../stores/downloadStore'
import { useLaunchStore } from '../stores/launchStore'
import { useLauncherStore } from '../stores/launcherStore'
import { useErrorStore } from '../stores/errorStore'
import { ConsoleLinePayload, DownloadProgressPayload, ErrorPayload, LaunchStatus, LaunchStatusPayload, LightyEvent, StdStream } from '../types/lighty-events'
import consola from 'consola'

function nowSecs(): number {
  return Math.floor(Date.now() / 1000)
}

function toLogEntry(line: string, instanceName: string = '', pid: number = 0, stream: StdStream = StdStream.Stdout): ConsoleLinePayload {
  return { instance_name: instanceName, pid, stream, line, timestamp: nowSecs() }
}

export function useLightyEvents() {
  const downloadStore = useDownloadStore()
  const launchStore = useLaunchStore()
  const launcherStore = useLauncherStore()
  const consoleStore = useConsoleStore()
  const errorStore = useErrorStore()

  let unlistenFns: UnlistenFn[] = []
  let lastDownloadPercentage = -1

  async function setupListeners() {
    try {
      const unlistenDownload = await listen<DownloadProgressPayload>(
        LightyEvent.DownloadProgress,
        (event) => {
          downloadStore.updateProgress(event.payload)

          const p = event.payload
          if (p.percentage !== lastDownloadPercentage) {
            lastDownloadPercentage = p.percentage
            consoleStore.addLog(toLogEntry(
              `[Download] ${p.message} (${p.percentage}%)`,
              p.instance_name,
            ))
          }
        }
      )

      const unlistenStatus = await listen<LaunchStatusPayload>(
        LightyEvent.LaunchStatus,
        (event) => {
          consola.info('Launch status event received:', event.payload);
          launchStore.updateStatus(event.payload)

          const p = event.payload
          consoleStore.addLog(toLogEntry(
            `[Status] ${p.phase}`,
            p.instance_name,
            p.pid,
          ))

          if (p.status === LaunchStatus.Running) {
            downloadStore.complete(p.instance_name)
          }

          if (p.status === LaunchStatus.Exited || p.status === LaunchStatus.Failed) {
            launcherStore.launching = false
          }
        }
      )

      const unlistenConsole = await listen<ConsoleLinePayload[]>(
        LightyEvent.ConsoleOutput,
        (event) => {
          for (const log of event.payload) {
            consoleStore.addLog(log)
          }
        }
      )

      const unlistenError = await listen<ErrorPayload>(
        LightyEvent.Error,
        (event) => {
          errorStore.setError(event.payload)

          const p = event.payload
          consoleStore.addLog(toLogEntry(
            `[Error:${p.category}] ${p.message}${p.details ? ` — ${p.details}` : ''}`,
            '',
            0,
            StdStream.Stderr,
          ))
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
