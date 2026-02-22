<template>
  <div>
    <router-link to="/splashscreen">go to splashscreen</router-link>
    Index
    <button @click="launchGame">Launch Game</button>
    <button @click="addAccount">Add Account</button>
    <br />
    <ul>
      <li v-for="account in listAccount" :key="account">
        {{ account }} <button @click="getAccount(account)">Get Account</button>
      </li>
    </ul>
  </div>
</template>

<script lang="ts" setup>
import { onBeforeUnmount, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

async function launchGame() {
  try {
    const name = prompt("Game name:", "mirabuild");
    if (!name) {
      alert("Game name is required.");
      return;
    }
    const profileName = prompt("Profile name:", "test");
    if (!profileName) {
      alert("Profile name is required.");
      return;
    }
    let javaDistribution = prompt(
      "Java distribution (e.g., temurin):",
      "temurin",
    );
    if (!javaDistribution) {
      javaDistribution = "temurin"; // Default to temurin if not provided
    }

    const result = await invoke("launch_game", {
      name,
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

onMounted(async () => {
  unlisten = await listen("launcher:console", (e) => {
    console.log("[launcher]", e.payload);
  });
});

onBeforeUnmount(async () => {
  if (unlisten) {
    await unlisten();
    unlisten = null;
  }
});

const listAccount = await invoke("list_accounts");
console.log("Accounts:", listAccount);

async function addAccount() {
  try {
    const accountType = prompt(
      "Account type (e.g., microsoft, offline):",
      "offline",
    );
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
</script>
