import { useI18n } from "vue-i18n"
import { useQuasar } from "quasar"
import { useStorage } from "@vueuse/core"
import { KEYS } from "@/constants/storage"

//TODO: Refactor task: https://trello.com/c/N37UkTdo
export function useSetting() {
    const { availableLocales, locale: localeGlobal } = useI18n({ useScope: "global" })
    const quasar = useQuasar()

    const locale = useStorage(KEYS.language, "en")
    const dark = useStorage(KEYS.dark, true)

    function localeChnage(lang: string) {
        locale.value = lang
        localeGlobal.value = lang
    }

    function darkChnage(value: boolean) {
        dark.value = value
        quasar.dark.set(value)
    }

    return {
        availableLocales,
        locale,
        dark,
        localeChnage,
    }
}
