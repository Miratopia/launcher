<template>
  <div
    class="w-full min-h-screen flex flex-col items-center justify-center relative overflow-hidden transition-opacity duration-700"
    :class="fadeOut ? 'opacity-0' : 'opacity-100'"
    style="background-color: #070b14"
  >
    <!-- Background effects -->
    <div class="absolute inset-0 bg-gradient-to-br from-amber-950/20 via-transparent to-yellow-950/10" />
    <div class="absolute top-1/3 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[500px] h-[500px] bg-amber-500/10 rounded-full blur-3xl animate-pulse" />
    <div class="absolute bottom-0 left-1/4 w-72 h-72 bg-orange-500/5 rounded-full blur-3xl" />
    <div class="absolute top-1/4 right-1/4 w-48 h-48 bg-yellow-500/5 rounded-full blur-3xl" />

    <!-- Floating particles -->
    <div
      v-for="particle in particles"
      :key="particle.id"
      class="absolute w-1 h-1 bg-amber-400/30 rounded-full animate-pulse"
      :style="{
        left: particle.left,
        top: particle.top,
        animationDelay: particle.delay,
        animationDuration: particle.duration,
      }"
    />

    <!-- Main content -->
    <div class="relative z-10 flex flex-col items-center">
      <!-- Logo icon -->
      <div class="relative mb-8">
        <div class="absolute inset-0 bg-amber-500/20 blur-2xl animate-pulse" />
        <img
          src="~/assets/images/logo.svg"
          alt="Miratopia"
          class="relative w-24 h-24 shadow-amber-500/30 animate-float"
        >
      </div>

      <!-- Logo text -->
      <img
        src="/full-logo.webp"
        alt="Miratopia Launcher"
        class="h-12 mb-16 drop-shadow-[0_0_30px_rgba(251,191,36,0.4)]"
      >

      <!-- Progress section -->
      <div class="w-80 flex flex-col items-center">
        <!-- Progress bar -->
        <div class="w-full h-1.5 bg-white/5 rounded-full overflow-hidden mb-4">
          <div
            class="h-full bg-gradient-to-r from-amber-500 via-yellow-500 to-orange-500 rounded-full transition-all duration-500 ease-out relative"
            :style="{ width: `${progress}%` }"
          >
            <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/30 to-transparent animate-shimmer" />
          </div>
        </div>

        <!-- Status text -->
        <div class="flex items-center gap-1">
          <span class="text-sm text-white/50">{{ status }}</span>
          <span class="text-sm text-white/50 w-4">{{ dots }}</span>
        </div>

        <!-- Percentage -->
        <span class="text-xs text-amber-400/50 mt-2 font-medium">{{ progress }}%</span>
      </div>
    </div>

    <!-- Version -->
    <div class="absolute bottom-6 text-xs text-white/20">
      v{{ appVersion }} · © 2025 Miratopia
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { check } from '@tauri-apps/plugin-updater'
import { getVersion } from '@tauri-apps/api/app'
import { useRouter } from 'vue-router'

definePageMeta({ layout: false })

const router = useRouter()

const progress = ref(0)
const status = ref('Initialisation...')
const fadeOut = ref(false)
const dots = ref('')
const appVersion = ref('')

interface Particle {
  id: number
  left: string
  top: string
  delay: string
  duration: string
}

const particles: Particle[] = Array.from({ length: 20 }, (_, i) => ({
  id: i,
  left: `${Math.random() * 100}%`,
  top: `${Math.random() * 100}%`,
  delay: `${Math.random() * 2}s`,
  duration: `${2 + Math.random() * 3}s`,
}))

const stages = [
  { progress: 15, status: 'Vérification des fichiers' },
  { progress: 30, status: 'Connexion aux serveurs' },
  { progress: 50, status: 'Chargement des modpacks' },
  { progress: 70, status: 'Synchronisation des données' },
  { progress: 85, status: 'Préparation de l\'interface' },
  { progress: 100, status: 'Lancement' },
]

function runFakeLoading(): Promise<void> {
  return new Promise((resolve) => {
    let currentStage = 0
    const interval = setInterval(() => {
      if (currentStage < stages.length) {
        progress.value = stages[currentStage].progress
        status.value = stages[currentStage].status
        currentStage++
      }
      else {
        clearInterval(interval)
        resolve()
      }
    }, 300)
  })
}

async function testUpdater(): Promise<void> {
  if (!import.meta.env.PROD) return

  try {
    const update = await check()
    if (!update) return

    status.value = 'Mise à jour du launcher'
    progress.value = 0

    let totalBytes: number | undefined
    let downloadedBytes = 0

    await update.downloadAndInstall((event) => {
      if (event.event === 'Started') {
        totalBytes = event.data.contentLength
      }
      else if (event.event === 'Progress') {
        downloadedBytes += event.data.chunkLength
        if (totalBytes) {
          progress.value = Math.min(Math.round((downloadedBytes / totalBytes) * 100), 100)
        }
      }
      else if (event.event === 'Finished') {
        progress.value = 100
        status.value = 'Installation terminée'
      }
    })
  }
  catch (error) {
    console.error('Update check failed:', error)
  }
}

let dotInterval: ReturnType<typeof setInterval> | undefined

onMounted(async () => {
  appVersion.value = await getVersion().catch(() => '0.0.0')

  dotInterval = setInterval(() => {
    dots.value = dots.value.length >= 3 ? '' : dots.value + '.'
  }, 400)

  await runFakeLoading()
  await testUpdater()

  fadeOut.value = true
  await new Promise(resolve => setTimeout(resolve, 700))

  router.push('/launcher')
})

onUnmounted(() => {
  if (dotInterval) clearInterval(dotInterval)
})
</script>

<style scoped>
@keyframes float {
  0%, 100% { transform: translateY(0px); }
  50% { transform: translateY(-10px); }
}

@keyframes shimmer {
  0% { transform: translateX(-100%); }
  100% { transform: translateX(100%); }
}

.animate-float {
  animation: float 3s ease-in-out infinite;
}

.animate-shimmer {
  animation: shimmer 1.5s infinite;
}
</style>
