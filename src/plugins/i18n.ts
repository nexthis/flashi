import messages from "@intlify/vite-plugin-vue-i18n/messages"
import { createI18n } from "vue-i18n"

export default createI18n({
    locale: "en", // set locale
    fallbackLocale: "en",
    messages, // set locale messages
})
