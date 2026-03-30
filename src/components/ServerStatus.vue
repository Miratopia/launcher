<script setup lang="ts">
import { ref, watch, onMounted, useTemplateRef } from 'vue'
import { useLauncherStore } from '~/stores/launcherStore'
import { splashPhrases } from '~/data/splashPhrases'

const launcherStore = useLauncherStore()
const videoRef = useTemplateRef<HTMLVideoElement>('bg-video')

function randomSplash(exclude?: string): string {
  const pool = exclude ? splashPhrases.filter(p => p !== exclude) : splashPhrases
  return pool[Math.floor(Math.random() * pool.length)]
}

const currentSplash = ref(randomSplash())

watch(() => launcherStore.selectedPack, () => {
  currentSplash.value = randomSplash(currentSplash.value)
})

const playlist = [
  '/videos/video-test.mp4',
  '/videos/Pyramid.mp4',
  '/videos/Frelheim.mp4',
  '/videos/Tour_ensorceleur.mp4',
]

let currentIndex = 0

function playNext() {
  const video = videoRef.value
  if (!video) return
  video.src = playlist[currentIndex]
  video.load()
  video.play().catch(() => {})
}

onMounted(() => {
  const video = videoRef.value
  if (!video) return

  video.addEventListener('ended', () => {
    currentIndex = (currentIndex + 1) % playlist.length
    playNext()
  })

  playNext()
})
</script>

<template>
  <div class="relative flex-1 flex items-center justify-center overflow-hidden">
    <video
      ref="bg-video"
      muted
      playsinline
      class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 min-w-full min-h-full w-auto h-auto scale-125"
    />

    <div class="absolute inset-0 bg-black/60" />

    <div class="relative z-10 text-center">
      <img
        src="/full-logo.webp"
        alt="Miratopia"
        class="h-20 object-contain mx-auto"
      />
      <p
        class="absolute -right-28 -top-3 text-primary-300 font-bold minecraft-text splash-text max-w-[380px] text-center leading-tight"
        style="font-family: 'Minecraft', 'MinecraftRegular', monospace;"
      >
        {{ currentSplash }}
      </p>
    </div>
  </div>
</template>

<style scoped>
@font-face {
  font-family: 'Minecraft';
  src: url('/fonts/Minecraft.otf') format('opentype');
  font-display: swap;
}

@keyframes splash {
  0%, 100% { transform: rotate(14deg) scale(1); }
  50% { transform: rotate(14deg) scale(1.15); }
}

.splash-text {
  transform-origin: center;
  animation: splash 1.5s ease-in-out infinite;
}

.minecraft-text {
  text-shadow:
    2px 2px 0 #000,
    -1px -1px 0 #000,
    1px -1px 0 #000,
    -1px 1px 0 #000,
    1px 1px 0 #000,
    2px 0 0 #000,
    0 2px 0 #000,
    -1px 0 0 #000,
    0 -1px 0 #000;
}
</style>