<template>
  <div>
    <router-link to="/">index</router-link>
    Splashscreen
  </div>
</template>

<script setup lang="ts">
import { check } from "@tauri-apps/plugin-updater";

async function testUpdater() {
  const res = await check();
  console.log(res);
}
testUpdater();
</script>
