<script setup lang="ts">
import { Settings, Rocket, Monitor, Users, X } from 'lucide-vue-next'
import { useLauncherStore } from '../../stores/launcherStore'

const store = useLauncherStore()

const tabs = [
  { id: 'launcher' as const, icon: Rocket, label: 'Launcher' },
  { id: 'modpack' as const, icon: Monitor, label: 'Miratopia SMP' },
]

const bottomTabs = [
  { id: 'accounts' as const, icon: Users, label: 'Comptes' },
]

const settingsTitles: Record<string, string> = {
  launcher: 'Launcher',
  modpack: 'Miratopia SMP',
  accounts: 'Comptes',
}
</script>

<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition duration-200 ease-out"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition duration-150 ease-in"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="store.showSettings"
        class="fixed inset-0 z-50 flex items-center justify-center p-8 surface-overlay"
        @click.self="store.closeSettings()"
      >
        <Transition
          enter-active-class="transition duration-200 ease-out"
          enter-from-class="opacity-0 scale-95"
          enter-to-class="opacity-100 scale-100"
          leave-active-class="transition duration-150 ease-in"
          leave-from-class="opacity-100 scale-100"
          leave-to-class="opacity-0 scale-95"
        >
          <div
            v-if="store.showSettings"
            class="w-full max-w-3xl surface-modal flex"
            style="height: 70vh"
          >
            <!-- Sidebar -->
            <div class="w-56 surface-modal-sidebar flex flex-col">
              <div class="px-4 h-14 border-b border-white/5 flex items-center gap-3 shrink-0">
                <Settings :size="18" class="text-amber-400" />
                <span class="font-semibold text-white">Paramètres</span>
              </div>
              <div class="p-2 flex-1">
                <button
                  v-for="tab in tabs"
                  :key="tab.id"
                  :class="store.settingsTab === tab.id ? 'settings-tab-active' : 'settings-tab-inactive'"
                  @click="store.settingsTab = tab.id"
                >
                  <component :is="tab.icon" :size="16" />
                  <span class="text-sm font-medium">{{ tab.label }}</span>
                </button>
              </div>
              <div class="p-2 border-t border-white/5">
                <button
                  v-for="tab in bottomTabs"
                  :key="tab.id"
                  :class="store.settingsTab === tab.id ? 'settings-tab-active' : 'settings-tab-inactive'"
                  @click="store.settingsTab = tab.id"
                >
                  <component :is="tab.icon" :size="16" />
                  <span class="text-sm font-medium">{{ tab.label }}</span>
                </button>
              </div>
            </div>

            <!-- Content -->
            <div class="flex-1 flex flex-col">
              <div class="px-4 h-14 border-b border-white/5 flex items-center justify-between shrink-0">
                <h2 class="text-lg font-semibold text-white">
                  {{ settingsTitles[store.settingsTab] }}
                </h2>
                <button
                  class="p-2 hover:bg-white/10 rounded-lg transition-colors text-white/40 hover:text-white"
                  @click="store.closeSettings()"
                >
                  <X :size="18" />
                </button>
              </div>

              <div class="flex-1 overflow-y-auto p-5">
                <SettingsLauncher v-if="store.settingsTab === 'launcher'" />
                <SettingsModpack v-else-if="store.settingsTab === 'modpack'" />
                <SettingsAccounts v-else-if="store.settingsTab === 'accounts'" />
              </div>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>
