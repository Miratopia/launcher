<script setup lang="ts">
import { computed } from 'vue'
import { Puzzle } from 'lucide-vue-next'
import type { FileModpackInfo } from '~/types/modpack'

const props = withDefaults(
  defineProps<{
    mods: FileModpackInfo[]
    selectedPaths: string[]
    updatable?: boolean
    disabled?: boolean
    emptyMessage?: string
  }>(),
  {
    updatable: false,
    disabled: false,
    emptyMessage: 'Aucun mod optionnel pour ce modpack.',
  },
)

const emit = defineEmits<{
  'update:selectedPaths': [value: string[]]
}>()

const selectedSet = computed(() => new Set(props.selectedPaths))

function isEnabled(path: string): boolean {
  return selectedSet.value.has(path)
}

function toggle(path: string) {
  if (!props.updatable || props.disabled) return
  const next = new Set(selectedSet.value)
  if (next.has(path)) {
    next.delete(path)
  } else {
    next.add(path)
  }
  emit('update:selectedPaths', Array.from(next))
}

function displayName(mod: FileModpackInfo): string {
  return mod.name?.trim() || mod.path
}
</script>

<template>
  <div class="space-y-2">
    <div
      v-if="mods.length === 0"
      class="surface-card rounded-xl p-6 text-center text-sm text-white/40"
    >
      {{ emptyMessage }}
    </div>

    <div
      v-for="mod in mods"
      :key="mod.path"
      class="surface-card-hover rounded-xl p-4"
    >
      <div class="flex items-center gap-4">
        <div class="icon-box">
          <Puzzle :size="20" class="text-amber-400" />
        </div>

        <div class="flex-1 min-w-0">
          <h3 class="text-sm font-medium text-white truncate" :title="displayName(mod)">
            {{ displayName(mod) }}
          </h3>
          <p
            v-if="mod.description"
            class="text-xs text-white/40 mt-0.5"
          >
            {{ mod.description }}
          </p>
          <p
            v-else
            class="text-xs text-white/30 mt-0.5 truncate"
            :title="mod.path"
          >
            {{ mod.path }}
          </p>
        </div>

        <template v-if="updatable">
          <button
            type="button"
            :disabled="disabled"
            :class="[
              isEnabled(mod.path) ? 'toggle-track-on' : 'toggle-track-off',
              disabled && 'opacity-30 cursor-not-allowed',
            ]"
            :aria-pressed="isEnabled(mod.path)"
            :aria-label="`Activer ${displayName(mod)}`"
            @click="toggle(mod.path)"
          >
            <div
              :class="[
                'toggle-thumb',
                isEnabled(mod.path) ? 'translate-x-4' : 'translate-x-0',
              ]"
            />
          </button>
        </template>

        <template v-else>
          <span
            v-if="mod.optional"
            class="shrink-0 inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-amber-500/10 text-amber-400 border border-amber-500/20"
          >
            Optionnel
          </span>
        </template>
      </div>
    </div>
  </div>
</template>
