<script setup lang="ts">
import { computed } from 'vue'
import { Settings, Rocket, Package, Users, X } from 'lucide-vue-next'
import { useLauncherStore } from '../../stores/launcherStore'

const store = useLauncherStore()

const modpackTabs = computed(() =>
  store.modpacks.map((pack) => ({
    id: `modpack:${pack.id}`,
    label: pack.name,
  })),
)

function selectTab(tabId: string) {
  store.settingsTab = tabId
  if (tabId.startsWith('modpack:')) {
    const modpackId = tabId.slice('modpack:'.length)
    store.loadModpackSettings(modpackId)
  }
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

              <div class="p-2 flex-1 overflow-y-auto">
                <!-- Général -->
                <div class="mb-4">
                  <span class="px-3 text-[11px] font-semibold uppercase tracking-wider text-white/30">
                    Général
                  </span>
                  <div class="mt-1.5 space-y-1">
                    <button
                      :class="store.settingsTab === 'launcher' ? 'settings-tab-active' : 'settings-tab-inactive'"
                      @click="selectTab('launcher')"
                    >
                      <Rocket :size="16" />
                      <span class="text-sm font-medium">Launcher</span>
                    </button>
                  </div>
                </div>

                <!-- Modpacks -->
                <div>
                  <span class="px-3 text-[11px] font-semibold uppercase tracking-wider text-white/30">
                    Modpacks
                  </span>
                  <div class="mt-1.5 space-y-1">
                    <button
                      v-for="tab in modpackTabs"
                      :key="tab.id"
                      :class="store.settingsTab === tab.id ? 'settings-tab-active' : 'settings-tab-inactive'"
                      @click="selectTab(tab.id)"
                    >
                      <Package :size="16" />
                      <span class="text-sm font-medium">{{ tab.label }}</span>
                    </button>
                  </div>
                </div>
              </div>

              <div class="p-2 border-t border-white/5">
                <button
                  :class="store.settingsTab === 'accounts' ? 'settings-tab-active' : 'settings-tab-inactive'"
                  @click="selectTab('accounts')"
                >
                  <Users :size="16" />
                  <span class="text-sm font-medium">Comptes</span>
                </button>
              </div>
            </div>

            <!-- Content -->
            <div class="flex-1 flex flex-col">
              <div class="px-4 h-14 border-b border-white/5 flex items-center justify-between shrink-0">
                <h2 class="text-lg font-semibold text-white">
                  {{ store.settingsTitle }}
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
                <SettingsModpack
                  v-else-if="store.activeSettingsModpackId"
                  :key="store.activeSettingsModpackId"
                  :modpack-id="store.activeSettingsModpackId"
                />
                <SettingsAccounts v-else-if="store.settingsTab === 'accounts'" />
              </div>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>
