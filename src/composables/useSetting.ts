import type { SettingStorageInterface } from "@/types/setting"
import { SETTING_DEFAULT_VALUES } from "@/constants/setting"
import { computed } from "vue"
import { Dark } from "quasar"
import { useStorage } from "@vueuse/core"
import i18n from "@/plugins/i18n"

export function useSetting() {
    const state = useStorage<SettingStorageInterface>("setting", SETTING_DEFAULT_VALUES)

    const language = computed(() => state.value.language)
    const isDark = computed(() => state.value.dark)

    function changeLanguage(lang: SupportLanguage) {
        i18n.global.locale.value = lang
        state.value.language = lang
    }

    function theme(dark: boolean) {
        Dark.set(dark)
        state.value.dark = dark
    }

    return {
        language,
        isDark,
        changeLanguage,
        theme,
    }
}
