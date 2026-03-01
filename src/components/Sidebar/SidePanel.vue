<script setup lang="ts">
import { ExternalLink, Loader2 } from 'lucide-vue-next'
import { useLauncherStore } from '../../stores/launcherStore'

const store = useLauncherStore()
</script>

<template>
  <div class="w-80 surface-panel flex flex-col">
    <div class="p-5 flex-1 overflow-auto">
      <!-- Annonces -->
      <div class="flex items-center justify-between mb-4">
        <h2 class="section-label">Annonces</h2>
        <ExternalLink
          :size="14"
          class="text-white/20 hover:text-amber-400 cursor-pointer transition-colors"
        />
      </div>

      <div class="space-y-3">
        <SidebarAnnouncementCard
          v-for="(a, i) in store.announcements"
          :key="i"
          :announcement="a"
        />
      </div>

      <!-- Modpacks -->
      <div class="mt-8">
        <h2 class="section-label mb-4">Modpacks</h2>

        <div v-if="store.modpacksLoading" class="flex items-center justify-center py-6">
          <Loader2 :size="20" class="text-amber-400 animate-spin" />
        </div>

        <div v-else-if="store.modpacks.length === 0" class="text-center py-6">
          <p class="text-sm text-white/30">Aucun modpack disponible</p>
        </div>

        <div v-else class="space-y-2">
          <SidebarModpackCard
            v-for="pack in store.modpacks"
            :key="pack.id"
            :pack="pack"
            :selected="store.selectedPack === pack.id"
            @select="store.selectPack"
          />
        </div>
      </div>
    </div>

    <SidebarMemorySelector />
  </div>
</template>
