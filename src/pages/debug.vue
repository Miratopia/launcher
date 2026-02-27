<template>
  <div>
    <router-link to="/" class="bg-gray-200 text-gray-800 px-4 py-2 rounded-lg hover:bg-gray-300"
      >go to index</router-link
    >
    Debug
    <button class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700" @click="launchGame">
      Launch this Game
    </button>
    <button class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700" @click="addAccount">
      Add Account
    </button>
    <br />
    <ul>
      <li v-for="account in listAccount" :key="account">
        {{ account }}
        <button class="px-2 py-1 bg-yellow-600 text-white rounded-lg hover:bg-yellow-700" @click="getAccount(account)">
          Get Account
        </button>
        <button class="px-2 py-1 bg-red-600 text-white rounded-lg hover:bg-red-700" @click="removeAccount(account)">
          Remove Account
        </button>
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
import { useConsoleStore } from "../stores/consoleStore";
import { useDownloadStore } from "../stores/downloadStore";
import { computed } from "vue";
import { useAccountsCommand } from "../composables/useAccountsCommand";
import { useModpacksCommand } from "../composables/useModpacksCommand";
import { useSettingsCommand } from "../composables/useSettingsCommand";

const {
  listAccounts,
  getAccount: getAccountInternal,
  addAccount: addAccountInternal,
  delAccount: delAccountInternal,
} = useAccountsCommand();
const { startModpack } = useModpacksCommand();
const { displaySettings, updateSettings } = useSettingsCommand();

console.log("Debug page loaded");
console.log('settings for "mirabuild":', await displaySettings("mirabuild"));
// await updateSettings("mirabuild", { java_distribution: "temurin", min_memory: 2048, max_memory: 4096 });

const consoleStore = useConsoleStore();
const downloadStore = useDownloadStore();

const progress = computed(() => downloadStore.getStatusByInstance("mirabuild")?.percentage || 0);

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

    const result = await startModpack(modpackName, profileName, javaDistribution);
    console.log("✅ Launch result:", result);
  } catch (error) {
    console.error("❌ Launch failed:", error);
  }
}

// const listAccount = await invoke("list_accounts");
const listAccount = await listAccounts();
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

    const result = await addAccountInternal(accountType, profileName);
    console.log("✅ Add account result:", result);
  } catch (error) {
    console.error("❌ Add account failed:", error);
  }
}

async function getAccount(account: string) {
  try {
    const result = await getAccountInternal(account);
    console.log("✅ Get account result:", result);
  } catch (error) {
    console.error("❌ Get account failed:", error);
  }
}

async function removeAccount(account: string) {
  try {
    const result = await delAccountInternal(account);
    console.log("✅ Remove account result:", result);
  } catch (error) {
    console.error("❌ Remove account failed:", error);
  }
}
</script>
