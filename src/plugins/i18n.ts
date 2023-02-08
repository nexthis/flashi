import { createI18n } from "vue-i18n"
// eslint-disable-next-line import/no-unresolved
import messages from "@intlify/vite-plugin-vue-i18n/messages"
import { useSetting } from "@/composables/settings/useSetting"

export default createI18n({
    locale: useSetting().value.language, // set locale
    fallbackLocale: "en",
    legacy: false,
    messages, // set locale messages
})
