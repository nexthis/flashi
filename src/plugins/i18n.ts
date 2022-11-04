import { useStorage } from "@vueuse/core"
import { createI18n } from "vue-i18n"
// eslint-disable-next-line import/no-unresolved
import messages from "@intlify/vite-plugin-vue-i18n/messages"
import { SETTING_DEFAULT_VALUES } from "@/constants/setting"
import type { SettingStorageInterface } from "@/types/setting"

export default createI18n({
    locale: useStorage<SettingStorageInterface>("setting", SETTING_DEFAULT_VALUES).value.language, // set locale
    fallbackLocale: "en",
    legacy: false,
    messages, // set locale messages
})
