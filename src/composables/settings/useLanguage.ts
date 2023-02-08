import { computed } from "vue"
import { useSetting } from "./useSetting"
import i18n from "@/plugins/i18n"

export function useLanguage() {
    const state = useSetting()
    const language = computed(() => state.value.language)

    function changeLanguage(lang: SupportLanguage) {
        i18n.global.locale.value = lang
        state.value.language = lang
    }

    return {
        language,
        changeLanguage,
    }
}
