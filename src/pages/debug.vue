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
    active account: {{ activeAccount?.username }} ({{ activeAccount?.uuid }})
    <br />
    <h1>Modpacks</h1>
    <ul>
      <li v-for="modpack in modpacks" :key="modpack">
        {{ modpack }}
      </li>
    </ul>
    <ul>
      <li v-for="account in listAccount" :key="account">
        {{ account }}
        <button
          class="px-2 py-1 bg-green-600 text-white rounded-lg hover:bg-green-700"
          @click="switchActiveAccount(account)"
        >
          Switch Active
        </button>
        <button
          class="px-2 py-1 bg-yellow-600 text-white rounded-lg hover:bg-yellow-700"
          @click="displayAccount(account)"
        >
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
import { JavaDistribution } from "../types/settings";

const {
  listAccounts,
  displayAccount: displayAccountInternal,
  addAccount: addAccountInternal,
  delAccount: delAccountInternal,
  switchActiveAccount: switchActiveAccountInternal,
  getActiveAccount,
} = useAccountsCommand();
const { startModpack, listModpacks } = useModpacksCommand();
const { displayModpackSettings, updateModpackSettings } = useSettingsCommand();

const activeAccount = await getActiveAccount();
console.log("active account", activeAccount);
const modpacks = await listModpacks();

console.log("Debug page loaded");
console.log('settings for "mirabuild":', await displayModpackSettings("mirabuild"));
await updateModpackSettings("mirabuild", {
  javaDistribution: JavaDistribution.Liberica,
  minMemory: 2048,
  maxMemory: 6144,
  fullScreen: false,
  windowWidth: 1280,
  windowHeight: 720,
});

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

    const result = await startModpack(modpackName);
    console.log("✅ Launch result:", result);
  } catch (error) {
    console.error("❌ Launch failed:", error);
  }
}

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

    const result = await addAccountInternal(accountType, profileName, ({ code, url, cancel }) => {
      alert("Code: " + code + "\nURL: " + url);
      setTimeout(() => {
        cancel();
      }, 3_000);
    });
    console.log("✅ Add account result:", result);
  } catch (error) {
    console.error("❌ Add account failed:", error);
  }
}

async function displayAccount(account: string) {
  try {
    const result = await displayAccountInternal(account);
    console.log("✅ Display account result:", result);
  } catch (error) {
    console.error("❌ Display account failed:", error);
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

async function switchActiveAccount(account: string) {
  try {
    const result = await switchActiveAccountInternal(account);
    console.log("✅ Switch active account result:", result);
  } catch (error) {
    console.error("❌ Switch active account failed:", error);
  }
}
</script>
