import { invoke } from "@tauri-apps/api/core"
import { listen, UnlistenFn } from "@tauri-apps/api/event"
import consola from "consola"

export function useAccountsCommand() {
  async function getAccount(profileName: string) {
    try {
      const accounts = await invoke('get_account', { profileName })
      return accounts
    } catch (error) {
      consola.error('Failed to get accounts:', error)
      throw error
    }
  }

  async function addAccount(
    accountType: string,
    profileName?: string | null,
    cb?: ({ code, url, cancel }: { code: string, url: string, cancel: () => void }) => void,
  ) {
    let result = null
    let unlistenAuthMicrosoftCode: UnlistenFn | null = null
    try {
      unlistenAuthMicrosoftCode = await listen<any>('lighty://auth-microsoft-code', (event) => {
        const { code, url } = event.payload
        consola.log(`Auth code: ${code}, URL: ${url}`)
        if (cb) {
          cb({
            code,
            url,
            cancel: () => {
              if (unlistenAuthMicrosoftCode) {
                unlistenAuthMicrosoftCode()
              }
            },
          })
        }
      })
      result = await invoke('add_account', { accountType, profileName })

    } catch (error) {
      consola.error('Failed to add account:', error)
      throw error
    } finally {
      if (unlistenAuthMicrosoftCode) {
        await unlistenAuthMicrosoftCode()
      }
    }

    return result
  }

  async function delAccount(profileName: string) {
    try {
      await invoke('del_account', { profileName })
    } catch (error) {
      consola.error('Failed to delete account:', error)
      throw error
    }
  }

  async function listAccounts() {
    try {
      const accounts = await invoke('list_accounts')
      return accounts
    } catch (error) {
      consola.error('Failed to list accounts:', error)
      throw error
    }
  }

  return {
    getAccount,
    addAccount,
    delAccount,
    listAccounts,
  }
}
