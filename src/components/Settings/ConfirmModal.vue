<script setup lang="ts">
import { AlertTriangle, X } from 'lucide-vue-next'

defineProps<{
  show: boolean
  title: string
  message: string
  confirmLabel?: string
  variant?: 'danger' | 'warning'
}>()

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()
</script>

<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition duration-150 ease-out"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition duration-100 ease-in"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="show"
        class="fixed inset-0 z-[100] flex items-center justify-center p-8 surface-overlay"
        @click.self="emit('cancel')"
      >
        <Transition
          enter-active-class="transition duration-150 ease-out"
          enter-from-class="opacity-0 scale-95"
          enter-to-class="opacity-100 scale-100"
          leave-active-class="transition duration-100 ease-in"
          leave-from-class="opacity-100 scale-100"
          leave-to-class="opacity-0 scale-95"
        >
          <div
            v-if="show"
            class="w-full max-w-md surface-modal p-6 space-y-5"
          >
            <div class="flex items-start gap-4">
              <div
                class="p-2.5 rounded-lg shrink-0"
                :class="variant === 'danger' ? 'bg-red-500/10' : 'bg-amber-500/10'"
              >
                <AlertTriangle
                  :size="20"
                  :class="variant === 'danger' ? 'text-red-400' : 'text-amber-400'"
                />
              </div>
              <div class="flex-1 min-w-0">
                <h3 class="text-base font-semibold text-white">{{ title }}</h3>
                <p class="text-sm text-white/50 mt-1.5 leading-relaxed">{{ message }}</p>
              </div>
              <button
                class="p-1.5 hover:bg-white/10 rounded-lg transition-colors text-white/40 hover:text-white shrink-0"
                @click="emit('cancel')"
              >
                <X :size="16" />
              </button>
            </div>

            <div class="flex justify-end gap-3 pt-1">
              <button
                class="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-sm text-white/70 hover:text-white transition-all"
                @click="emit('cancel')"
              >
                Annuler
              </button>
              <button
                class="px-4 py-2 rounded-lg text-sm font-medium transition-all"
                :class="variant === 'danger'
                  ? 'bg-red-500/20 hover:bg-red-500/30 border border-red-500/30 text-red-400 hover:text-red-300'
                  : 'bg-amber-500/20 hover:bg-amber-500/30 border border-amber-500/30 text-amber-400 hover:text-amber-300'"
                @click="emit('confirm')"
              >
                {{ confirmLabel ?? 'Confirmer' }}
              </button>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>
