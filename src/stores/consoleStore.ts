import { defineStore } from 'pinia'

export const useConsoleStore = defineStore('console', {
  state: () => ({
    logs: new Map<number, { pid: number; stream: 'stdout' | 'stderr'; line: string; timestamp: number }[]>(),
  }),

  getters: {
    getLogsByPid: (state) => {
      return (pid: number) => state.logs.get(pid) || [];
    },
    getAllLogs: (state) => {
      const allLogs: { pid: number; stream: 'stdout' | 'stderr'; line: string; timestamp: number }[] = [];
      state.logs.forEach((logs) => {
        allLogs.push(...logs);
      });
      return allLogs;
    },
  },

  actions: {
    addLog(pid: number, log: { pid: number; stream: 'stdout' | 'stderr'; line: string; timestamp: number }) {
      if (!this.logs.has(pid)) {
        this.logs.set(pid, []);
      }
      this.logs.get(pid)!.push(log);
    },
    clearLogs(pid: number) {
      this.logs.delete(pid);
    },
    clearAllLogs() {
      this.logs.clear();
    },
  },
})
