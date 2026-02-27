import { invoke } from "@tauri-apps/api/core"

export function useAccountsCommand() {
  async function getAccount(profileName: string) {
    try {
      const accounts = await invoke('get_account', { profileName })
      return accounts
    } catch (error) {
      console.error('Failed to get accounts:', error)
      throw error
    }
  }

  async function addAccount(accountType: string, profileName?: string | null) {
    try {
      await invoke('add_account', { accountType, profileName })
    } catch (error) {
      console.error('Failed to add account:', error)
      throw error
    }
  }

  async function delAccount(profileName: string) {
    try {
      await invoke('del_account', { profileName })
    } catch (error) {
      console.error('Failed to delete account:', error)
      throw error
    }
  }

  async function listAccounts() {
    try {
      const accounts = await invoke('list_accounts')
      return accounts
    } catch (error) {
      console.error('Failed to list accounts:', error)
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
