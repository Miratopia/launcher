<script setup lang="ts">
import { UserPlus } from 'lucide-vue-next'
</script>

<template>
  <div class="space-y-4">
    <!-- Compte actuel -->
    <div class="bg-gradient-to-r from-amber-500/10 to-transparent rounded-xl p-4 border border-amber-500/20">
      <div class="flex items-center gap-4">
        <img
          src="https://mc-heads.net/avatar/MHF_Steve/48"
          alt="Skin"
          class="w-12 h-12 rounded-xl"
          style="image-rendering: pixelated"
        />
        <div class="flex-1">
          <h3 class="text-sm font-medium text-white">MiraPlayer_42</h3>
          <p class="text-xs text-amber-400/80 mt-0.5">Compte actif &bull; Premium</p>
        </div>
        <div class="w-3 h-3 bg-emerald-400 rounded-full shadow-lg shadow-emerald-400/50" />
      </div>
    </div>

    <!-- Ajouter compte -->
    <button
      class="w-full bg-white/5 hover:bg-white/10 rounded-xl p-4 border border-white/5 border-dashed hover:border-amber-500/30 transition-all group"
    >
      <div class="flex items-center justify-center gap-3">
        <UserPlus :size="20" class="text-white/40 group-hover:text-amber-400 transition-colors" />
        <span class="text-sm text-white/60 group-hover:text-white transition-colors">
          Ajouter un compte Microsoft
        </span>
      </div>
    </button>
  </div>
</template>
