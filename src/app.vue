<template>
  <nuxt-layout>
    <nuxt-page />
  </nuxt-layout>
</template>

<script lang="ts" setup>
import { useLightyEvents } from "./composables/useLightyEvents";

useLightyEvents();
</script>
