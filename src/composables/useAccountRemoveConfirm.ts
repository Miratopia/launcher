import { ref } from 'vue'
import { useAccountsStore } from '../stores/accountsStore'

const show = ref(false)
const pendingAccount = ref<string | null>(null)

export function useAccountRemoveConfirm() {
  const store = useAccountsStore()

  function requestRemove(profileName: string) {
    pendingAccount.value = profileName
    show.value = true
  }

  async function confirm() {
    if (pendingAccount.value) {
      await store.removeAccount(pendingAccount.value)
    }
    show.value = false
    pendingAccount.value = null
  }

  function cancel() {
    show.value = false
    pendingAccount.value = null
  }

  return { show, pendingAccount, requestRemove, confirm, cancel }
}
