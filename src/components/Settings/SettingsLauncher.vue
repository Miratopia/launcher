<script setup lang="ts">
import { FolderOpen, Monitor, Rocket, ChevronRight, X, ToggleLeft, ToggleRight } from 'lucide-vue-next'
import { useLauncherStore } from '../../stores/launcherStore'

const store = useLauncherStore()
</script>

<template>
  <div class="space-y-4">
    <!-- Dossier launcher -->
    <SettingsSettingRow
      :icon="FolderOpen"
      title="Dossier du launcher"
      description="Ouvrir le dossier dans l'explorateur"
    >
      <template #action>
        <button
          class="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-sm text-white/80 hover:text-white transition-all"
        >
          Ouvrir
        </button>
      </template>
    </SettingsSettingRow>

    <!-- Résolution -->
    <SettingsSettingRow
      :icon="Monitor"
      title="Résolution de l'écran"
      description="Résolution appliquée au lancement"
    >
      <template #content>
        <div class="flex items-center gap-3 mt-3">
          <input
            v-model="store.resWidth"
            type="text"
            class="w-20 input-field"
          />
          <span class="text-white/30">&times;</span>
          <input
            v-model="store.resHeight"
            type="text"
            class="w-20 input-field"
          />
          <button
            :class="[
              'flex items-center gap-2 px-3 py-2 rounded-lg border transition-all',
              store.fullscreen
                ? 'bg-amber-500/20 border-amber-500/30 text-amber-400'
                : 'bg-black/30 border-white/10 text-white/60 hover:text-white',
            ]"
            @click="store.fullscreen = !store.fullscreen"
          >
            <component :is="store.fullscreen ? ToggleRight : ToggleLeft" :size="18" />
            <span class="text-sm">Plein écran</span>
          </button>
        </div>
      </template>
    </SettingsSettingRow>

    <!-- Auto update -->
    <SettingsSettingToggle
      :icon="Rocket"
      title="Mise à jour automatique"
      description="Télécharger automatiquement les mises à jour"
      v-model="store.autoUpdate"
    />

    <!-- Console -->
    <SettingsSettingToggle
      :icon="ChevronRight"
      title="Afficher la console"
      description="Afficher les logs au lancement"
      v-model="store.showConsole"
    />

    <!-- Fermer launcher -->
    <SettingsSettingToggle
      :icon="X"
      title="Fermer le launcher"
      description="Fermer le launcher au lancement du jeu"
      v-model="store.closeLauncher"
    />
  </div>
</template>
