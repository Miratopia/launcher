<script setup lang="ts">
import { ref } from 'vue'
import { Settings } from 'lucide-vue-next'
import { useLauncherStore } from '~/stores/launcherStore'

const store = useLauncherStore()

const helpMenuRef = ref<{ close: () => void } | null>(null)
const accountMenuRef = ref<{ close: () => void } | null>(null)
</script>

<template>
  <div class="flex items-center justify-end gap-2 p-4">
    <HeaderHelpMenu ref="helpMenuRef" />

    <button class="btn-icon-default" @click="store.openSettings()">
      <Settings :size="18" />
    </button>

    <!-- TODO: decomment this when we implement notification system -->
    <!-- <button class="btn-icon-accent">
      <Bell :size="18" />
    </button> -->

    <HeaderAccountMenu ref="accountMenuRef" />
  </div>
</template>
