import messages from "@intlify/vite-plugin-vue-i18n/messages"
import { createI18n } from "vue-i18n"
import { useStorage } from "@vueuse/core"
import { KEYS } from "@/constants/storage"

export default createI18n({
    locale: useStorage(KEYS.language, "en").value, // set locale
    fallbackLocale: "en",
    legacy: false,
    messages, // set locale messages
})
