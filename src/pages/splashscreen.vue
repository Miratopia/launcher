<template>
  <div>
    <router-link to="/">index</router-link>
    Splashscreen
    <br />
    pre:
    <pre v-html="JSON.stringify(data, null, 2)"></pre>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { check } from "@tauri-apps/plugin-updater";

let data = ref<any>();
async function testUpdater() {
  const res = await check();
  data.value = res;
  console.log(res);
}
testUpdater();
</script>
