<template>
    <div class="text-h2">{{ t("title") }}</div>

    <q-list class="q-mt-lg">
        <!-- General -->
        <q-item-label header>{{ t("general") }}</q-item-label>

        <LanguageItem />

        <q-separator spaced inset />

        <ThemeItem />

        <q-separator spaced inset />

        <!-- App -->
        <q-item-label header>{{ t("app") }}</q-item-label>

        <VersionItem />

        <q-separator spaced inset />

        <AutoStartItem />

        <q-separator spaced inset />

        <UpdateItem />

        <q-separator spaced inset />
    </q-list>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n"
import LanguageItem from "./sections/LanguageItem.vue"
import ThemeItem from "./sections/ThemeItem.vue"
import UpdateItem from "./sections/UpdateItem.vue"
import VersionItem from "./sections/VersionItem.vue"
import AutoStartItem from "./sections/AutoStartItem.vue"

const { t } = useI18n()
</script>

<i18n>
    "en": {
        "title": "Settings",
        "general": "General",
        "app": "App",
    }

    "pl": {
        "title": "Ustawienia",
        "general": "Ogólne",
        "app": "Aplikacja",
    }
</i18n>
