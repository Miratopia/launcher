<script setup lang="ts">
import { ref, computed } from 'vue'
import { FolderOpen, Trash2, RotateCcw, UserX, PackageX } from 'lucide-vue-next'
import { useModpacksCommand } from '../../composables/useModpacksCommand'
import { useLauncherMaintenanceCommand } from '../../composables/useLauncherMaintenanceCommand'
import { useLauncherStore } from '../../stores/launcherStore'
import { useAccountsStore } from '../../stores/accountsStore'

const { openLauncherFolder, openModpacksFolder } = useModpacksCommand()
const { clearCache, resetAllSettings, clearAllAccounts, deleteAllModpacks } = useLauncherMaintenanceCommand()
const launcherStore = useLauncherStore()
const accountsStore = useAccountsStore()

type ConfirmAction = 'cache' | 'settings' | 'accounts' | 'modpacks' | null
const confirmAction = ref<ConfirmAction>(null)
const actionLoading = ref(false)

const confirmConfig = computed(() => {
  switch (confirmAction.value) {
    case 'cache':
      return {
        title: 'Vider le cache',
        message: 'La position et la taille de la fenêtre seront réinitialisées. Le launcher va redémarrer automatiquement.',
        confirmLabel: 'Vider et redémarrer',
        variant: 'warning' as const,
      }
    case 'settings':
      return {
        title: 'Réinitialiser les paramètres',
        message: 'Tous les paramètres du launcher et les paramètres personnalisés de vos modpacks seront supprimés et remis aux valeurs par défaut.',
        confirmLabel: 'Réinitialiser',
        variant: 'danger' as const,
      }
    case 'accounts':
      return {
        title: 'Supprimer tous les comptes',
        message: 'Tous les comptes connectés seront déconnectés et supprimés du launcher. Vous devrez vous reconnecter.',
        confirmLabel: 'Supprimer les comptes',
        variant: 'danger' as const,
      }
    case 'modpacks':
      return {
        title: 'Supprimer tous les modpacks',
        message: 'Tous les fichiers des modpacks téléchargés seront supprimés du disque. Ils seront retéléchargés au prochain lancement.',
        confirmLabel: 'Supprimer les modpacks',
        variant: 'danger' as const,
      }
    default:
      return { title: '', message: '', confirmLabel: '', variant: 'warning' as const }
  }
})

async function handleConfirm() {
  actionLoading.value = true
  try {
    switch (confirmAction.value) {
      case 'cache':
        await clearCache()
        break
      case 'settings':
        await resetAllSettings()
        await launcherStore.loadModpackSettings()
        break
      case 'accounts':
        await clearAllAccounts()
        await accountsStore.fetchAccounts()
        await accountsStore.fetchActiveAccount()
        break
      case 'modpacks':
        await deleteAllModpacks()
        break
    }
  } finally {
    actionLoading.value = false
    confirmAction.value = null
  }
}
</script>

<template>
  <div class="space-y-4">
    <!-- Dossier launcher -->
    <SettingsSettingRow
      :icon="FolderOpen"
      title="Dossier du launcher"
      description="Ouvrir le dossier d'installation du launcher"
    >
      <template #action>
        <button
          :disabled="launcherStore.isGameActive"
          class="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-sm text-white/80 hover:text-white transition-all disabled:opacity-30 disabled:cursor-not-allowed disabled:hover:bg-white/5 disabled:hover:text-white/80"
          @click="openLauncherFolder"
        >
          Ouvrir
        </button>
      </template>
    </SettingsSettingRow>

    <!-- Dossier modpacks -->
    <SettingsSettingRow
      :icon="FolderOpen"
      title="Dossier des modpacks"
      description="Ouvrir le dossier de données des modpacks"
    >
      <template #action>
        <button
          :disabled="launcherStore.isGameActive"
          class="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-sm text-white/80 hover:text-white transition-all disabled:opacity-30 disabled:cursor-not-allowed disabled:hover:bg-white/5 disabled:hover:text-white/80"
          @click="openModpacksFolder"
        >
          Ouvrir
        </button>
      </template>
    </SettingsSettingRow>

    <!-- Séparateur -->
    <div class="border-t border-white/5" />

    <!-- Vider le cache -->
    <SettingsSettingRow
      :icon="Trash2"
      title="Vider le cache"
      description="Réinitialise la position et la taille de la fenêtre"
    >
      <template #action>
        <button
          :disabled="launcherStore.isGameActive"
          class="px-4 py-2 bg-amber-500/10 hover:bg-amber-500/20 border border-amber-500/20 rounded-lg text-sm text-amber-400 hover:text-amber-300 transition-all disabled:opacity-30 disabled:cursor-not-allowed disabled:hover:bg-amber-500/10 disabled:hover:text-amber-400"
          @click="confirmAction = 'cache'"
        >
          Vider
        </button>
      </template>
    </SettingsSettingRow>

    <!-- Réinitialiser les paramètres -->
    <SettingsSettingRow
      :icon="RotateCcw"
      title="Réinitialiser les paramètres"
      description="Remet tous les paramètres du launcher et des modpacks par défaut"
    >
      <template #action>
        <button
          :disabled="launcherStore.isGameActive"
          class="px-4 py-2 bg-red-500/10 hover:bg-red-500/20 border border-red-500/20 rounded-lg text-sm text-red-400 hover:text-red-300 transition-all disabled:opacity-30 disabled:cursor-not-allowed disabled:hover:bg-red-500/10 disabled:hover:text-red-400"
          @click="confirmAction = 'settings'"
        >
          Réinitialiser
        </button>
      </template>
    </SettingsSettingRow>

    <!-- Supprimer tous les comptes -->
    <SettingsSettingRow
      :icon="UserX"
      title="Supprimer tous les comptes"
      description="Déconnecte et supprime tous les comptes enregistrés"
    >
      <template #action>
        <button
          :disabled="launcherStore.isGameActive"
          class="px-4 py-2 bg-red-500/10 hover:bg-red-500/20 border border-red-500/20 rounded-lg text-sm text-red-400 hover:text-red-300 transition-all disabled:opacity-30 disabled:cursor-not-allowed disabled:hover:bg-red-500/10 disabled:hover:text-red-400"
          @click="confirmAction = 'accounts'"
        >
          Supprimer
        </button>
      </template>
    </SettingsSettingRow>

    <!-- Supprimer tous les modpacks -->
    <SettingsSettingRow
      :icon="PackageX"
      title="Supprimer tous les modpacks"
      description="Supprime tous les fichiers de modpacks téléchargés"
    >
      <template #action>
        <button
          :disabled="launcherStore.isGameActive"
          class="px-4 py-2 bg-red-500/10 hover:bg-red-500/20 border border-red-500/20 rounded-lg text-sm text-red-400 hover:text-red-300 transition-all disabled:opacity-30 disabled:cursor-not-allowed disabled:hover:bg-red-500/10 disabled:hover:text-red-400"
          @click="confirmAction = 'modpacks'"
        >
          Supprimer
        </button>
      </template>
    </SettingsSettingRow>

    <!-- Modale de confirmation -->
    <SettingsConfirmModal
      :show="confirmAction !== null"
      :title="confirmConfig.title"
      :message="confirmConfig.message"
      :confirm-label="confirmConfig.confirmLabel"
      :variant="confirmConfig.variant"
      @confirm="handleConfirm"
      @cancel="confirmAction = null"
    />
  </div>
</template>
