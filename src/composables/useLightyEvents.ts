import { onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useConsoleStore } from '../stores/consoleStore'

export function useLightyEvents() {
  // const downloadStore = useDownloadStore();
  // const launchStore = useLaunchStore();
  const consoleStore = useConsoleStore()
  // const errorStore = useErrorStore();
  let unlistenFns: UnlistenFn[] = []

  async function setupListeners() {
    try {
      const unlistenDownload = await listen<any>(
        'lighty://download-progress',
        (event) => {
          console.log('Download progress event received:', event.payload);
          // downloadStore.updateProgress(event.payload);
        }
      )

      const unlistenStatus = await listen<any>(
        'lighty://launch-status',
        (event) => {
          console.log('Launch status event received:', event.payload);
          // launchStore.updateStatus(event.payload);

          // Reset download si lancement réussi
          if (event.payload.status === 'running') {
            // downloadStore.complete(event.payload.instance_name);
          }
        }
      )

      // Console Output
      const unlistenConsole = await listen<any[]>(
        'lighty://console-output',
        (event) => {
          for (const log of event.payload) {
            console.log('Console log received:', log);
            consoleStore.addLog(log.pid, {
              pid: log.pid,
              stream: log.stream as 'stdout' | 'stderr',
              line: log.line,
              timestamp: log.timestamp,
            });
          }
        }
      );

      // Errors
      const unlistenError = await listen<any>(
        'lighty://error',
        (event) => {
          console.error('Error event received:', event.payload);
          // errorStore.setError(event.payload);
        }
      )

      unlistenFns = [
        unlistenDownload,
        unlistenStatus,
        unlistenConsole,
        unlistenError,
      ]
      console.log('Listeners set up successfully')
    } catch (error) {
      console.error('Failed to listen to download progress:', error)
    }
  }

  function cleanup() {
    unlistenFns.forEach(fn => fn())
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
