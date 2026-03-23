<template>
  <div
    class="w-full min-h-screen flex flex-col overflow-hidden relative launcher-bg"
  >
    <div class="absolute inset-0 bg-gradient-to-br from-amber-950/20 via-transparent to-yellow-950/10 pointer-events-none" />

    <slot />
  </div>
</template>
