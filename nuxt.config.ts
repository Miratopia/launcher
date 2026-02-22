// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  telemetry: false,
  devtools: { enabled: true },
  ssr: false,
  pages: true,
  components: true,
  srcDir: 'src/',
  vite: {
    clearScreen: false,
    // https://v2.tauri.app/reference/environment-variables/
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true,
      host: '0.0.0.0',
      hmr: {  
        protocol: 'ws',
        host: '0.0.0.0',
        port: 5183,
      },
    },
  },
});