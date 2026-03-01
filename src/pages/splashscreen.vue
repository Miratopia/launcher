<template>
  <div>
    <router-link to="/" class="bg-gray-200 text-gray-800 px-4 py-2 rounded-lg hover:bg-gray-300"
      >go to index</router-link
    >
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
  if (!import.meta.env.PROD) {
    data.value = { skipped: true, reason: "dev-mode" };
    return;
  }

  try {
    const res = await check();
    data.value = res ?? { update: false };
    console.log(res);

    if (res) {
      await res.downloadAndInstall();
      data.value = { update: true, installed: true };
    }
  } catch (error) {
    data.value = { error: String(error) };
    console.error(error);
  }
}
testUpdater();
</script>
