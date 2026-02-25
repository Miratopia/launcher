<template>
  <div>
    <router-link to="/splashscreen">go to splashscreen</router-link>
    Index
    <button @click="launchGame">Launch this Game</button>
    <button @click="addAccount">Add Account</button>
    <br />
    <ul>
      <li v-for="account in listAccount" :key="account">
        {{ account }} <button @click="getAccount(account)">Get Account</button>
        <button @click="removeAccount(account)">Remove Account</button>
      </li>
    </ul>
    <progress :value="progress" max="100"></progress>
  </div>
</template>

<script lang="ts" setup>
import { onBeforeUnmount, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

async function launchGame() {
  try {
    const modpackName = prompt("Game name:", "mirabuild");
    if (!modpackName) {
      alert("Game name is required.");
      return;
    }
    const profileName = prompt("Profile name:", "test");
    if (!profileName) {
      alert("Profile name is required.");
      return;
    }
    let javaDistribution = prompt("Java distribution (e.g., temurin):", "temurin");
    if (!javaDistribution) {
      javaDistribution = "temurin"; // Default to temurin if not provided
    }

    const result = await invoke("launch_game", {
      modpackName,
      profileName,
      javaDistribution,
    });
    console.log("✅ Launch result:", result);
  } catch (error) {
    console.error("❌ Launch failed:", error);
  }
}

import { listen } from "@tauri-apps/api/event";

let unlisten: null | (() => void) = null;
let javaDownloadStartedUnlisten: null | (() => void) = null;
let javaDownloadProgressUnlisten: null | (() => void) = null;
let javaDownloadCompletedUnlisten: null | (() => void) = null;
let launcherDownloadStartedUnlisten: null | (() => void) = null;
let launcherDownloadProgressUnlisten: null | (() => void) = null;
let launcherDownloadCompletedUnlisten: null | (() => void) = null;

const progress = ref(0);
const maxProgress = ref(100);

onMounted(async () => {
  unlisten = await listen("launcher:console", (e) => {
    console.log("[launcher]", e.payload);
  });
  javaDownloadStartedUnlisten = await listen<any>("launcher:java-download-started", (e) => {
    console.log("[launcher] Java download started:", e.payload);
    maxProgress.value = e.payload.total_bytes; // Assuming the payload contains the total size
  });
  javaDownloadProgressUnlisten = await listen<any>("launcher:java-download-progress", (e) => {
    progress.value = (e.payload.bytes * 100) / maxProgress.value; // Update progress based on bytes downloaded
    console.log(`Download progress: ${progress.value}/${maxProgress.value}`);
  });
  javaDownloadCompletedUnlisten = await listen<any>("launcher:java-download-completed", (e) => {
    progress.value = 100; // Set progress to max when completed
    console.log("Java download completed");
  });
  launcherDownloadStartedUnlisten = await listen<any>("launcher:launcher-download-started", (e) => {
    console.log("[launcher] Launcher download started:", e.payload);
    maxProgress.value = e.payload.total_bytes; // Assuming the payload contains the total size
  });
  launcherDownloadProgressUnlisten = await listen<any>("launcher:launcher-download-progress", (e) => {
    progress.value = (e.payload.bytes * 100) / maxProgress.value; // Update progress based on bytes downloaded
    console.log(`Download progress: ${progress.value}/${maxProgress.value}`);
  });
  launcherDownloadCompletedUnlisten = await listen<any>("launcher:launcher-download-completed", (e) => {
    progress.value = 100; // Set progress to max when completed
    console.log("Launcher download completed");
  });
});

onBeforeUnmount(async () => {
  if (unlisten) {
    await unlisten();
    unlisten = null;
  }
  if (javaDownloadStartedUnlisten) {
    await javaDownloadStartedUnlisten();
    javaDownloadStartedUnlisten = null;
  }
  if (javaDownloadProgressUnlisten) {
    await javaDownloadProgressUnlisten();
    javaDownloadProgressUnlisten = null;
  }
  if (javaDownloadCompletedUnlisten) {
    await javaDownloadCompletedUnlisten();
    javaDownloadCompletedUnlisten = null;
  }
  if (launcherDownloadStartedUnlisten) {
    await launcherDownloadStartedUnlisten();
    launcherDownloadStartedUnlisten = null;
  }
  if (launcherDownloadProgressUnlisten) {
    await launcherDownloadProgressUnlisten();
    launcherDownloadProgressUnlisten = null;
  }
  if (launcherDownloadCompletedUnlisten) {
    await launcherDownloadCompletedUnlisten();
    launcherDownloadCompletedUnlisten = null;
  }
});

const listAccount = await invoke("list_accounts");
console.log("Accounts:", listAccount);

async function addAccount() {
  try {
    const accountType = prompt("Account type (e.g., microsoft, offline):", "offline");
    if (!accountType) {
      alert("Account type is required.");
      return;
    }

    let profileName = null;
    if (accountType === "offline") {
      profileName = prompt("Profile name:", "test");
      if (!profileName) {
        alert("Profile name is required.");
        return;
      }
    }

    const result = await invoke("add_account", {
      accountType,
      profileName,
    });
    console.log("✅ Add account result:", result);
  } catch (error) {
    console.error("❌ Add account failed:", error);
  }
}

async function getAccount(account: string) {
  try {
    const result = await invoke("get_account", { profileName: account });
    console.log("✅ Get account result:", result);
  } catch (error) {
    console.error("❌ Get account failed:", error);
  }
}

async function removeAccount(account: string) {
  try {
    const result = await invoke("del_account", { profileName: account });
    console.log("✅ Remove account result:", result);
  } catch (error) {
    console.error("❌ Remove account failed:", error);
  }
}
</script>
