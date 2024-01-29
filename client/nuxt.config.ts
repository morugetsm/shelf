// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  css: [
    'assets/style.scss',
  ],
  devtools: { enabled: true },
  routeRules: {
    '/api/**': {
      proxy: 'http://localhost:7878/**',
    },
  },
  // vite: {
  //   server: {
  //     proxy: {
  //       '/api': {
  //         target: 'http://localhost:7878/',
  //         changeOrigin: true,
  //         rewrite: path => path.replace(/^\/api/, ''),
  //       }
  //     }
  //   }
  // },
  modules: [
    '@nuxtjs/color-mode',
    '@nuxtjs/i18n',
    '@nuxtjs/tailwindcss',
    'nuxt-headlessui',
  ],
  i18n: {},
})
