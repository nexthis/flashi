import type { SettingStorageInterface } from "@/types/setting"
import { SETTING_DEFAULT_VALUES } from "@/constants/setting"
import { useStorage } from "@vueuse/core"
import { createI18n } from "vue-i18n"
import messages from "@intlify/vite-plugin-vue-i18n/messages"

export default createI18n({
    locale: useStorage<SettingStorageInterface>("setting", SETTING_DEFAULT_VALUES).value.language, // set locale
    fallbackLocale: "en",
    legacy: false,
    messages, // set locale messages
})
