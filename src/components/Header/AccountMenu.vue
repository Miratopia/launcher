<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { User, UserPlus, LogOut } from 'lucide-vue-next'

const open = ref(false)
const menuRef = ref<HTMLElement | null>(null)

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
      <User :size="18" />
    </button>

    <Transition
      enter-active-class="transition duration-150 ease-out"
      enter-from-class="opacity-0 scale-95"
      enter-to-class="opacity-100 scale-100"
      leave-active-class="transition duration-100 ease-in"
      leave-from-class="opacity-100 scale-100"
      leave-to-class="opacity-0 scale-95"
    >
      <div v-if="open" class="absolute right-0 mt-2 w-52 surface-dropdown z-50">
        <div class="p-3 border-b border-white/5 flex items-center gap-3">
          <img
            src="https://mc-heads.net/avatar/MHF_Steve/32"
            alt="Skin"
            class="w-8 h-8 rounded-lg"
            style="image-rendering: pixelated"
          />
          <div>
            <p class="text-sm font-medium text-white">MiraPlayer_42</p>
            <p class="text-xs text-white/40">Compte Premium</p>
          </div>
        </div>
        <div class="p-2">
          <button
            class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg hover:bg-white/10 transition-colors"
          >
            <UserPlus :size="16" class="text-amber-400/70" />
            <span class="text-sm text-white/80">Ajouter un compte</span>
          </button>
          <button
            class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg hover:bg-red-500/10 transition-colors group"
          >
            <LogOut :size="16" class="text-white/40 group-hover:text-red-400" />
            <span class="text-sm text-white/80 group-hover:text-red-400">Se déconnecter</span>
          </button>
        </div>
      </div>
    </Transition>
  </div>
</template>
