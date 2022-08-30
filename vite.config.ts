import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import eslint from 'vite-plugin-eslint';
import { quasar, transformAssetUrls } from '@quasar/vite-plugin'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
      vue({template: { transformAssetUrls }}),
      quasar({sassVariables: 'src/styles/variables.scss'}),
      eslint()
  ],
    resolve: {
      alias: {
          '@': path.resolve(__dirname, './src/')
      }
    }
})
