import { defineConfig } from "vite"
import { resolve, dirname } from "node:path"
import { fileURLToPath } from "url"
import { quasar, transformAssetUrls } from "@quasar/vite-plugin"
import vueI18n from "@intlify/vite-plugin-vue-i18n"
import vue from "@vitejs/plugin-vue"
import eslint from "vite-plugin-eslint"
import path from "path"

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        vue({ template: { transformAssetUrls } }),
        quasar({ sassVariables: "src/styles/variables.scss" }),
        eslint(),
        vueI18n({
            defaultSFCLang: "json5",
            include: resolve(dirname(fileURLToPath(import.meta.url)), "./src/locales/**"),
        }),
    ],
    resolve: {
        alias: {
            "@": path.resolve(resolve(dirname(fileURLToPath(import.meta.url))), "./src/"),
        },
    },
})
