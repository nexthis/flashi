import { Quasar, Dialog, Notify } from "quasar"
import { useStorage } from "@vueuse/core"

//font
import "@quasar/extras/roboto-font-latin-ext/roboto-font-latin-ext.css"
// Import icon libraries
import "@quasar/extras/material-icons/material-icons.css"
import "@quasar/extras/mdi-v6/mdi-v6.css"
// Import Quasar css
import "quasar/src/css/index.sass"

//types
import type { QuasarPluginOptions } from "quasar/dist/types/plugin"
import type { SettingStorageInterface } from "@/types/setting"

import { SETTING_DEFAULT_VALUES } from "@/constants/setting"

export const config: Partial<QuasarPluginOptions> = {
    plugins: { Dialog, Notify },
    config: {
        dark: useStorage<SettingStorageInterface>("setting", SETTING_DEFAULT_VALUES).value.dark,
    },
}

export default Quasar
