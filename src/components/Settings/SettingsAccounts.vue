<script setup lang="ts">
import { computed, ref } from 'vue'
import { Trash2, CheckCircle, Loader2, Globe, Wifi, XCircle, Copy } from 'lucide-vue-next'
import { openUrl } from '@tauri-apps/plugin-opener'
import { useAccountsStore } from '~/stores/accountsStore'
import { useLauncherStore } from '~/stores/launcherStore'
import { useAccountRemoveConfirm } from '~/composables/useAccountRemoveConfirm'

const store = useAccountsStore()
const launcherStore = useLauncherStore()
const offlineNameInput = ref('')
const showOfflineForm = ref(false)

const { requestRemove } = useAccountRemoveConfirm()

const codeCopied = ref(false)

async function copyCode() {
  if (!store.microsoftAuthCode) return
  await navigator.clipboard.writeText(store.microsoftAuthCode.code)
  codeCopied.value = true
  setTimeout(() => { codeCopied.value = false }, 2000)
}

const activeAccountSubtitle = computed(() => {
  if (!store.activeAccount) return ''
  return store.activeAccount.type === 'offline'
    ? 'Profil hors-ligne • Actif'
    : 'Compte Microsoft • Actif'
})

async function onRequestRemove(profileName: string) {
  await requestRemove(profileName)
}

async function handleAddMicrosoft() {
  await store.addMicrosoftAccount()
}

async function handleAddOffline() {
  const name = offlineNameInput.value.trim()
  if (!name) return
  await store.addOfflineAccount(name)
  offlineNameInput.value = ''
  showOfflineForm.value = false
}

async function handleSwitch(profileName: string) {
  await store.switchActive(profileName)
}
</script>

<template>
  <div class="space-y-4">
    <!-- Compte actif -->
    <div
      v-if="store.activeAccount"
      class="bg-gradient-to-r from-amber-500/10 to-transparent rounded-xl p-4 border border-amber-500/20"
    >
      <div class="flex items-center gap-4">
        <img
          :src="`https://mc-heads.net/avatar/${store.activeAccount.username}/48`"
          alt="Skin"
          class="w-12 h-12 rounded-xl"
          style="image-rendering: pixelated"
        />
        <div class="flex-1">
          <h3 class="text-sm font-medium text-white">{{ store.activeAccount.username }}</h3>
          <p class="text-xs text-amber-400/80 mt-0.5">
            {{ activeAccountSubtitle }}
          </p>
        </div>
        <div class="w-3 h-3 bg-emerald-400 rounded-full shadow-lg shadow-emerald-400/50" />
      </div>
    </div>

    <!-- Liste des autres comptes -->
    <div
      v-for="account in store.accounts.filter((a: string) => a !== store.activeAccount?.username)"
      :key="account"
      class="bg-white/5 rounded-xl p-4 border border-white/5"
    >
      <div class="flex items-center gap-4">
        <img
          :src="`https://mc-heads.net/avatar/${account}/48`"
          alt="Skin"
          class="w-12 h-12 rounded-xl"
          style="image-rendering: pixelated"
        />
        <div class="flex-1">
          <h3 class="text-sm font-medium text-white">{{ account }}</h3>
          <p class="text-xs text-white/40 mt-0.5">Inactif</p>
        </div>
        <div class="flex items-center gap-2">
          <button
            :disabled="launcherStore.isGameActive"
            class="p-2 rounded-lg bg-white/5 hover:bg-amber-500/20 border border-white/10 hover:border-amber-500/30 text-white/60 hover:text-amber-400 transition-all disabled:opacity-30 disabled:cursor-not-allowed disabled:hover:bg-white/5 disabled:hover:text-white/60"
            title="Activer ce compte"
            @click="handleSwitch(account)"
          >
            <CheckCircle :size="16" />
          </button>
          <button
            :disabled="launcherStore.isGameActive"
            class="p-2 rounded-lg bg-white/5 hover:bg-red-500/10 border border-white/10 hover:border-red-500/30 text-white/60 hover:text-red-400 transition-all disabled:opacity-30 disabled:cursor-not-allowed disabled:hover:bg-white/5 disabled:hover:text-white/60"
            title="Supprimer ce compte"
            @click="onRequestRemove(account)"
          >
            <Trash2 :size="16" />
          </button>
        </div>
      </div>
    </div>

    <!-- Microsoft auth code display -->
    <div
      v-if="store.microsoftAuthCode"
      class="bg-blue-500/10 rounded-xl p-4 border border-blue-500/20"
    >
      <p class="text-sm text-white/80 mb-2">
        Connectez-vous sur
        <button class="text-blue-400 underline" @click="openUrl(store.microsoftAuthCode!.url)">
          {{ store.microsoftAuthCode.url }}
        </button>
      </p>
      <div class="flex items-center justify-center gap-3 py-2">
        <button
          class="flex items-center gap-2 px-4 py-2 rounded-lg bg-transparent border border-blue-400/40 hover:border-blue-400/70 text-blue-300 font-mono font-bold tracking-widest text-lg transition-all"
          @click="copyCode"
        >
          <Copy :size="16" class="shrink-0" />
          {{ store.microsoftAuthCode.code }}
        </button>
        <Transition
          enter-active-class="transition-all duration-200"
          enter-from-class="opacity-0 translate-x-1"
          enter-to-class="opacity-100 translate-x-0"
          leave-active-class="transition-all duration-200"
          leave-from-class="opacity-100 translate-x-0"
          leave-to-class="opacity-0 translate-x-1"
        >
          <span v-if="codeCopied" class="text-sm text-blue-300/80">Copié !</span>
        </Transition>
      </div>
      <button
        class="w-full mt-2 flex items-center justify-center gap-2 px-3 py-2 rounded-lg bg-white/5 hover:bg-red-500/10 border border-white/10 hover:border-red-500/30 text-white/60 hover:text-red-400 transition-all"
        @click="store.cancelMicrosoftAuth()"
      >
        <XCircle :size="16" />
        <span class="text-sm">Annuler</span>
      </button>
    </div>

    <!-- Loading state -->
    <div
      v-if="store.addingAccount && !store.microsoftAuthCode"
      class="flex items-center justify-center gap-3 py-4"
    >
      <Loader2 :size="20" class="text-amber-400 animate-spin" />
      <span class="text-sm text-white/60">Connexion en cours...</span>
    </div>

    <!-- Ajouter compte Microsoft -->
    <button
      class="w-full bg-white/5 hover:bg-white/10 rounded-xl p-4 border border-white/5 border-dashed hover:border-amber-500/30 transition-all group disabled:opacity-30 disabled:cursor-not-allowed disabled:hover:bg-white/5 disabled:hover:border-white/5"
      :disabled="store.addingAccount || launcherStore.isGameActive"
      @click="handleAddMicrosoft"
    >
      <div class="flex items-center justify-center gap-3">
        <Globe :size="20" class="text-white/40 group-hover:text-amber-400 transition-colors" />
        <span class="text-sm text-white/60 group-hover:text-white transition-colors">
          Ajouter un compte Microsoft
        </span>
      </div>
    </button>

    <!-- Ajouter compte offline -->
    <div v-if="showOfflineForm" class="bg-white/5 rounded-xl p-4 border border-white/5">
      <div class="flex items-center gap-3">
        <input
          v-model="offlineNameInput"
          type="text"
          placeholder="Nom du profil"
          :disabled="launcherStore.isGameActive"
          class="flex-1 input-field disabled:opacity-30 disabled:cursor-not-allowed"
          @keyup.enter="handleAddOffline"
        />
        <button
          class="px-4 py-2 bg-amber-500/20 hover:bg-amber-500/30 border border-amber-500/30 rounded-lg text-sm text-amber-400 transition-all disabled:opacity-30 disabled:cursor-not-allowed disabled:hover:bg-amber-500/20"
          :disabled="!offlineNameInput.trim() || launcherStore.isGameActive"
          @click="handleAddOffline"
        >
          Ajouter
        </button>
      </div>
    </div>
    <button
      v-else
      class="w-full bg-white/5 hover:bg-white/10 rounded-xl p-4 border border-white/5 border-dashed hover:border-white/20 transition-all group disabled:opacity-30 disabled:cursor-not-allowed disabled:hover:bg-white/5 disabled:hover:border-white/5"
      :disabled="store.addingAccount || launcherStore.isGameActive"
      @click="showOfflineForm = true"
    >
      <div class="flex items-center justify-center gap-3">
        <Wifi :size="20" class="text-white/40 group-hover:text-white/60 transition-colors" />
        <span class="text-sm text-white/60 group-hover:text-white transition-colors">
          Ajouter un compte hors-ligne
        </span>
      </div>
    </button>

  </div>
</template>
