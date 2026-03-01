<script setup lang="ts">
import type { Announcement } from '../../stores/launcherStore'

defineProps<{
  announcement: Announcement
}>()
</script>

<template>
  <div
    class="group bg-white/5 hover:bg-white/10 border border-white/5 hover:border-amber-500/30 rounded-2xl p-4 cursor-pointer transition-all duration-300"
  >
    <span class="text-xs text-amber-400/80 font-medium uppercase tracking-wider">
      {{ announcement.date }}
    </span>
    <p class="text-sm text-white/80 mt-1.5 leading-relaxed">
      {{ announcement.title }}
    </p>
  </div>
</template>
