<template>
    <div class="text-h2">{{ t("title") }}</div>
    <q-select v-model="locale" @update:model-value="onLocaleChnage" :options="availableLocales"></q-select>
    <q-toggle
        color="positive"
        @update:model-value="onDarkChnage"
        :model-value="quasar.dark.isActive"
        @click="quasar.dark.toggle"
    >
        {{ t("theme") }}
    </q-toggle>
</template>

<script setup lang="ts">
import { useQuasar } from "quasar"
import { useI18n } from "vue-i18n"
import { useStorage } from "@vueuse/core"
import { KEYS } from "@/constants/storage"

const quasar = useQuasar()
const { t } = useI18n()
const { availableLocales, locale } = useI18n({ useScope: "global" })

const localeState = useStorage(KEYS.language, locale.value)
const darkState = useStorage(KEYS.dark, quasar.dark.isActive)

const onLocaleChnage = (lang: string) => {
    localeState.value = lang
}

const onDarkChnage = (val: boolean) => {
    darkState.value = val
}
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
