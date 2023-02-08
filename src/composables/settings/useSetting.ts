import { useStorage } from "@vueuse/core"
import { SETTING_DEFAULT_VALUES } from "@/constants/setting"
import type { SettingStorageInterface } from "@/types/setting"

export function useSetting() {
    return useStorage<SettingStorageInterface>("setting", SETTING_DEFAULT_VALUES)
}
