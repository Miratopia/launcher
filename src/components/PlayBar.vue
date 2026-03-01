<script setup lang="ts">
import { Play } from 'lucide-vue-next'
import { useLauncherStore } from '../stores/launcherStore'

const store = useLauncherStore()
</script>

<template>
  <div class="p-6">
    <div
      class="backdrop-blur-sm rounded-2xl border border-white/5 p-4 flex items-center gap-4"
      style="background-color: rgba(255, 255, 255, 0.02)"
    >
      <div class="flex-1">
        <p class="text-white/90 font-medium">
          {{ store.selectedModpack?.name }}
        </p>
        <p class="text-white/30 text-sm">
          {{ store.launching ? 'Vérification des fichiers...' : 'Prêt à jouer' }}
        </p>
      </div>

      <button
        :disabled="store.launching"
        :class="store.launching ? 'btn-play-loading' : 'btn-play'"
        @click="store.launchGame()"
      >
        <template v-if="store.launching">
          <div class="w-5 h-5 border-2 border-white/20 border-t-white/60 rounded-full animate-spin" />
          Lancement...
        </template>
        <template v-else>
          <Play :size="20" fill="currentColor" />
          Jouer
        </template>
      </button>
    </div>
  </div>
</template>
