<script setup lang="ts">
import { computed } from 'vue'
import { useAccountRemoveConfirm } from '~/composables/useAccountRemoveConfirm'

const removeConfirm = useAccountRemoveConfirm()

const removeAccountModal = computed(() => {
  const name = removeConfirm.pendingAccount.value ?? ''
  const offline = removeConfirm.pendingAccountType.value === 'offline'
  if (offline) {
    return {
      title: 'Retirer le profil',
      message: `Voulez-vous vraiment retirer le profil local ${name} ? Vous pourrez en créer un autre avec le même nom.`,
      confirmLabel: 'Retirer',
    }
  }
  return {
    title: 'Déconnecter le compte',
    message: `Voulez-vous vraiment déconnecter le compte ${name} ? Vous pourrez vous reconnecter à tout moment.`,
    confirmLabel: 'Déconnecter',
  }
})
</script>

<template>
  <div
    class="w-full min-h-screen flex flex-col overflow-hidden relative launcher-bg"
  >
    <!-- Gradient overlay -->
    <div class="absolute inset-0 bg-gradient-to-br from-amber-950/20 via-transparent to-yellow-950/10 pointer-events-none" />

    <TitleBar />

    <slot />

    <SettingsModal />

    <ModpackOptionalModsDialog />

    <SettingsConfirmModal
      :show="removeConfirm.show.value"
      :title="removeAccountModal.title"
      :message="removeAccountModal.message"
      :confirm-label="removeAccountModal.confirmLabel"
      variant="danger"
      @confirm="removeConfirm.confirm"
      @cancel="removeConfirm.cancel"
    />
  </div>
</template>
