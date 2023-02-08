import { computed } from "vue"
import { Dark } from "quasar"
import { useSetting } from "./useSetting"

export function useTheme() {
    const state = useSetting()
    const isDark = computed(() => state.value.dark)

    function theme(dark: boolean) {
        Dark.set(dark)
        state.value.dark = dark
    }

    return {
        isDark,
        theme,
    }
}
