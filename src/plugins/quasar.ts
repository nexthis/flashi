import { SETTING_DEFAULT_VALUES } from "@/constants/setting"
import { Quasar, Dialog } from "quasar"
import { useStorage } from "@vueuse/core"
import type { SettingStorageInterface } from "@/types/setting"

//font
import "@quasar/extras/roboto-font-latin-ext/roboto-font-latin-ext.css"
// Import icon libraries
import "@quasar/extras/material-icons/material-icons.css"
import "@quasar/extras/mdi-v6/mdi-v6.css"
// Import Quasar css
import "quasar/src/css/index.sass"

//types
import type { QuasarPluginOptions } from "quasar/dist/types/plugin"

export const config: Partial<QuasarPluginOptions> = {
    plugins: { Dialog },
    config: {
        dark: useStorage<SettingStorageInterface>("setting", SETTING_DEFAULT_VALUES).value.dark,
    },
}

export default Quasar
