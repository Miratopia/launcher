import { defineStore } from 'pinia'
import { useAccountsCommand } from '../composables/useAccountsCommand'
import { useLauncherStore } from './launcherStore'

export interface Account {
  username: string
  uuid: string
  type: 'microsoft' | 'offline'
}

export interface MicrosoftAuthCode {
  code: string
  url: string
}

let _cancelMicrosoftAuth: (() => void) | null = null

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

    async refreshLauncherData() {
      const launcher = useLauncherStore()
      await launcher.fetchModpacks()
      if (launcher.selectedPack) {
        await launcher.loadModpackSettings()
      }
    },

    async addOfflineAccount(profileName: string) {
      const { addAccount } = useAccountsCommand()
      try {
        this.addingAccount = true
        await addAccount('offline', profileName)
        await this.fetchAccounts()
        await this.fetchActiveAccount()
        await this.refreshLauncherData()
      } finally {
        this.addingAccount = false
      }
    },

    cancelMicrosoftAuth() {
      if (_cancelMicrosoftAuth) {
        _cancelMicrosoftAuth()
        _cancelMicrosoftAuth = null
      }
      this.addingAccount = false
      this.microsoftAuthCode = null
    },

    async addMicrosoftAccount(): Promise<void> {
      const { addAccount } = useAccountsCommand()
      try {
        this.addingAccount = true
        this.microsoftAuthCode = null
        _cancelMicrosoftAuth = null
        await addAccount('microsoft', null, ({ code, url, cancel }) => {
          _cancelMicrosoftAuth = cancel
          this.microsoftAuthCode = { code, url }
        })
        this.microsoftAuthCode = null
        _cancelMicrosoftAuth = null
        await this.fetchAccounts()
        await this.fetchActiveAccount()
        await this.refreshLauncherData()
      } finally {
        this.addingAccount = false
        this.microsoftAuthCode = null
        _cancelMicrosoftAuth = null
      }
    },

    async removeAccount(profileName: string) {
      const { delAccount } = useAccountsCommand()
      try {
        await delAccount(profileName)
      } finally {
        await this.fetchAccounts()
        await this.fetchActiveAccount()
        await this.refreshLauncherData()
      }
    },

    async switchActive(profileName: string) {
      const { switchActiveAccount } = useAccountsCommand()
      await switchActiveAccount(profileName)
      await this.fetchActiveAccount()
      await this.refreshLauncherData()
    },

    async init() {
      await Promise.all([this.fetchAccounts(), this.fetchActiveAccount()])
    },
  },
})
