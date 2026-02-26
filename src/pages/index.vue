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
    <pre>{{ progress }}%</pre>
    <progress :value="progress" max="100"></progress>
    <br />
    logs:
    <ul>
      <li v-for="log in logs" :key="log.timestamp">{{ log.line }}</li>
    </ul>
  </div>
</template>

<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/core";
import { useConsoleStore } from "../stores/consoleStore";
import { useDownloadStore } from "../stores/downloadStore";

const consoleStore = useConsoleStore();
const downloadStore = useDownloadStore();

const progress = computed(() => downloadStore.getStatusByInstance("mirabuild")?.percentage || 0);

import { computed } from "vue";
const logs = computed(() => consoleStore.getAllLogs);

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
