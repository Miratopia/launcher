<script setup lang="ts">
</script>

<template>
  <div class="flex flex-1 relative z-10 overflow-hidden">
    <!-- Left Panel -->
    <SidebarSidePanel />

    <!-- Main Content -->
    <div class="flex-1 flex flex-col">
      <HeaderBar />
      <ServerStatus />
      <PlayBar />
    </div>
  </div>
</template>
