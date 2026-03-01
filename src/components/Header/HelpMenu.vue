<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { HelpCircle, Search, Palette, Upload, ExternalLink } from 'lucide-vue-next'

const open = ref(false)
const menuRef = ref<HTMLElement | null>(null)

const links = [
  { icon: Search, label: 'Trouver un skin', url: 'https://namemc.com/minecraft-skins' },
  { icon: Palette, label: 'Éditer mon skin', url: 'https://www.minecraftskins.com/skin-editor/' },
  { icon: Upload, label: 'Uploader mon skin', url: 'https://www.minecraft.net/fr-fr/msaprofile/mygames/editskin' },
]

function toggle() {
  open.value = !open.value
}

function close() {
  open.value = false
}

function onClickOutside(e: MouseEvent) {
  if (menuRef.value && !menuRef.value.contains(e.target as Node)) {
    close()
  }
}

onMounted(() => document.addEventListener('mousedown', onClickOutside))
onUnmounted(() => document.removeEventListener('mousedown', onClickOutside))

defineExpose({ close })
</script>

<template>
  <div ref="menuRef" class="relative">
    <button :class="open ? 'btn-icon-active' : 'btn-icon-default'" @click="toggle">
      <HelpCircle :size="18" />
    </button>

    <Transition
      enter-active-class="transition duration-150 ease-out"
      enter-from-class="opacity-0 scale-95"
      enter-to-class="opacity-100 scale-100"
      leave-active-class="transition duration-100 ease-in"
      leave-from-class="opacity-100 scale-100"
      leave-to-class="opacity-0 scale-95"
    >
      <div v-if="open" class="absolute right-0 mt-2 w-56 surface-dropdown z-50">
        <div class="p-2">
          <a
            v-for="link in links"
            :key="link.url"
            :href="link.url"
            target="_blank"
            rel="noopener noreferrer"
            class="flex items-center gap-3 px-3 py-2.5 rounded-lg hover:bg-white/10 transition-colors cursor-pointer group"
          >
            <component :is="link.icon" :size="16" class="text-amber-400/70" />
            <span class="text-sm text-white/80 group-hover:text-white">{{ link.label }}</span>
            <ExternalLink :size="12" class="ml-auto text-white/20" />
          </a>
        </div>
      </div>
    </Transition>
  </div>
</template>
