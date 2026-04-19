import { ref } from 'vue'
import { useAccountsCommand } from '~/composables/useAccountsCommand'
import { useAccountsStore } from '~/stores/accountsStore'
import type { Account } from '~/stores/accountsStore'

const show = ref(false)
const pendingAccount = ref<string | null>(null)
const pendingAccountType = ref<'microsoft' | 'offline'>('microsoft')

export function useAccountRemoveConfirm() {
  const store = useAccountsStore()

  async function requestRemove(
    profileName: string,
    opts?: { type?: 'microsoft' | 'offline' },
  ) {
    let accountType = opts?.type
    if (!accountType) {
      const { displayAccount } = useAccountsCommand()
      try {
        const partial = (await displayAccount(profileName)) as Account | null
        if (partial?.type === 'offline' || partial?.type === 'microsoft') {
          accountType = partial.type
        }
      } catch {
        accountType = 'microsoft'
      }
    }
    pendingAccount.value = profileName
    pendingAccountType.value = accountType ?? 'microsoft'
    show.value = true
  }

  async function confirm() {
    if (pendingAccount.value) {
      await store.removeAccount(pendingAccount.value)
    }
    show.value = false
    pendingAccount.value = null
    pendingAccountType.value = 'microsoft'
  }

  function cancel() {
    show.value = false
    pendingAccount.value = null
    pendingAccountType.value = 'microsoft'
  }

  return { show, pendingAccount, pendingAccountType, requestRemove, confirm, cancel }
}
