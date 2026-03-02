<template>
  <nuxt-layout>
    <nuxt-page />
  </nuxt-layout>
</template>

<script lang="ts" setup>
import { onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useLightyEvents } from "./composables/useLightyEvents";

const router = useRouter()

useLightyEvents();

// Raccourci clavier Ctrl + Alt + D pour accéder à la page debug
const handleKeydown = (event: KeyboardEvent) => {
  if (event.ctrlKey && event.altKey && event.key === 'd') {
    event.preventDefault();
    router.push('/debug');
  }
};

onMounted(() => {
  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});
</script>
