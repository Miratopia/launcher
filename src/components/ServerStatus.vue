<script setup lang="ts">
import { useAccountsCommand } from '../composables/useAccountsCommand';

const { displayActiveAccount } = useAccountsCommand();

const activeAccount = await displayActiveAccount();
console.log('active account', activeAccount);
</script>

<template>
  <div class="flex-1 flex items-center justify-center">
    <div class="text-start">
      <img
        src="/full-logo.webp"
        alt="Miratopia"
        class="h-20 object-contain mx-auto"
      />
      <pre class="w-64">{{ activeAccount }}</pre>
      <p class="text-amber-300/30 text-sm mt-3 tracking-widest uppercase">
        Serveur Minecraft Communautaire
      </p>

      <div class="flex items-center justify-center gap-8 mt-8">
        <div class="text-center">
          <div class="status-dot-online mx-auto" />
          <p class="text-xs text-white/30 mt-2">Serveur en ligne</p>
        </div>
      </div>
    </div>
  </div>
</template>