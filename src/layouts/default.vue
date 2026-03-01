<template>
  <div
    class="w-full min-h-screen flex flex-col overflow-hidden relative launcher-bg"
    :style="store.wallpaper ? { backgroundImage: `url(${store.wallpaper})`, backgroundSize: 'cover', backgroundPosition: 'center' } : undefined"
  >
    <!-- Dark overlay for readability when wallpaper is set -->
    <div
      v-if="store.wallpaper"
      class="absolute inset-0 bg-black/50 pointer-events-none"
    />

    <!-- Gradient overlay -->
    <div class="absolute inset-0 bg-gradient-to-br from-amber-950/20 via-transparent to-yellow-950/10 pointer-events-none" />

    <TitleBar />

    <slot />

    <SettingsModal />
  </div>
</template>

<script setup lang="ts">
import { useLauncherStore } from '../stores/launcherStore'

const store = useLauncherStore()
</script>
