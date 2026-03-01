<script setup lang="ts">
import { FolderOpen, Cpu, ChevronRight, ChevronDown } from 'lucide-vue-next'
import { useLauncherStore } from '../../stores/launcherStore'

const store = useLauncherStore()
</script>

<template>
  <div class="space-y-4">
    <!-- Dossier modpack -->
    <SettingsSettingRow
      :icon="FolderOpen"
      title="Dossier du modpack"
      description="Ouvrir le dossier .minecraft du modpack"
    >
      <template #action>
        <button
          class="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-sm text-white/80 hover:text-white transition-all"
        >
          Ouvrir
        </button>
      </template>
    </SettingsSettingRow>

    <!-- RAM -->
    <SettingsSettingRow
      :icon="Cpu"
      title="Mémoire allouée"
      description="RAM dédiée à Minecraft"
    >
      <template #content>
        <div class="flex items-center gap-3 mt-3">
          <input
            type="text"
            :value="store.memoryInput"
            class="w-20 input-field"
            @input="store.updateMemoryInput(($event.target as HTMLInputElement).value)"
          />
          <span class="text-white/40 text-sm">Go</span>
          <div class="flex-1 mx-2 h-2 bg-black/30 rounded-full overflow-hidden">
            <div
              class="h-full bg-gradient-to-r from-amber-500 to-orange-500 rounded-full transition-all"
              :style="{ width: `${Math.min((store.memory / 16) * 100, 100)}%` }"
            />
          </div>
          <span class="text-xs text-white/30">max 16 Go</span>
        </div>
      </template>
    </SettingsSettingRow>

    <!-- Version Java -->
    <SettingsSettingRow
      :icon="ChevronRight"
      title="Version Java"
      description="Runtime Java utilisé"
    >
      <template #action>
        <div class="flex items-center gap-2 px-3 py-2 bg-black/30 border border-white/10 rounded-lg">
          <span class="text-sm text-white/80">Java 17 (Bundled)</span>
          <ChevronDown :size="14" class="text-white/40" />
        </div>
      </template>
    </SettingsSettingRow>
  </div>
</template>
