<script setup lang="ts">
import { onMounted, useTemplateRef } from 'vue'

const videoRef = useTemplateRef<HTMLVideoElement>('bg-video')

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
      <p class="text-amber-300/30 text-sm mt-3 tracking-widest uppercase">
        Serveur Minecraft Communautaire
      </p>
    </div>
  </div>
</template>