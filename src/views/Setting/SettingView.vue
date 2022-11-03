<template>
    <div class="text-h2">{{ t("title") }}</div>
    <q-select :model-value="language" @update:model-value="changeLanguage" :options="['pl', 'en']"></q-select>
    <q-toggle color="positive" :model-value="isDark" @update:model-value="theme">
        {{ t("theme") }}
    </q-toggle>
</template>

<script setup lang="ts">
import { useSetting } from "@/composables/useSetting"
import { useI18n } from "vue-i18n"

const { t } = useI18n()
const { changeLanguage, isDark, language, theme } = useSetting()
</script>

<i18n>
    "en": {
        "title": "Settings",
        "theme": "Dark mode",
    }

    "pl": {
        "title": "Ustawienia",
        "theme": "Ciemny motyw",
    }
</i18n>
