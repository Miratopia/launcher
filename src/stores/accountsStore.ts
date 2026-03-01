import { defineStore } from 'pinia'
import { useAccountsCommand } from '../composables/useAccountsCommand'

export interface Account {
  username: string
  uuid: string
  type: 'microsoft' | 'offline'
}

export interface MicrosoftAuthCode {
  code: string
  url: string
}

export const useAccountsStore = defineStore('accounts', {
  state: () => ({
    accounts: [] as string[],
    activeAccount: null as Account | null,
    loading: false,
    microsoftAuthCode: null as MicrosoftAuthCode | null,
    addingAccount: false,
  }),

  getters: {
    hasAccounts: (state) => state.accounts.length > 0,

    isActive: (state) => {
      return (profileName: string): boolean =>
        state.activeAccount?.username === profileName
    },
  },

  actions: {
    async fetchAccounts() {
      const { listAccounts } = useAccountsCommand()
      try {
        this.loading = true
        const result = await listAccounts()
        this.accounts = (result as string[]) || []
      } catch {
        this.accounts = []
      } finally {
        this.loading = false
      }
    },

    async fetchActiveAccount() {
      const { displayActiveAccount } = useAccountsCommand()
      try {
        const result = await displayActiveAccount()
        this.activeAccount = result as Account | null
      } catch {
        this.activeAccount = null
      }
    },

    async addOfflineAccount(profileName: string) {
      const { addAccount } = useAccountsCommand()
      try {
        this.addingAccount = true
        await addAccount('offline', profileName)
        await this.fetchAccounts()
        await this.fetchActiveAccount()
      } finally {
        this.addingAccount = false
      }
    },

    async addMicrosoftAccount(): Promise<void> {
      const { addAccount } = useAccountsCommand()
      try {
        this.addingAccount = true
        this.microsoftAuthCode = null
        await addAccount('microsoft', null, ({ code, url, cancel }) => {
          console.log('code', code)
          this.microsoftAuthCode = { code, url }
        })
        this.microsoftAuthCode = null
        await this.fetchAccounts()
        await this.fetchActiveAccount()
      } finally {
        this.addingAccount = false
        this.microsoftAuthCode = null
      }
    },

    async removeAccount(profileName: string) {
      const { delAccount } = useAccountsCommand()
      await delAccount(profileName)
      await this.fetchAccounts()
      if (this.activeAccount?.username === profileName) {
        await this.fetchActiveAccount()
      }
    },

    async switchActive(profileName: string) {
      const { switchActiveAccount } = useAccountsCommand()
      await switchActiveAccount(profileName)
      await this.fetchActiveAccount()
    },

    async init() {
      await Promise.all([this.fetchAccounts(), this.fetchActiveAccount()])
    },
  },
})
