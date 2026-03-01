<template>
  <div class="w-full min-h-screen flex flex-col overflow-hidden relative launcher-bg">
    <!-- Gradient overlay -->
    <div class="absolute inset-0 bg-gradient-to-br from-amber-950/20 via-transparent to-yellow-950/10 pointer-events-none" />

    <TitleBar />

    <slot />

    <SettingsModal />
  </div>
</template>

<script setup lang="ts">
</script>
