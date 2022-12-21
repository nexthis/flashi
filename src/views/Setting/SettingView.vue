<template>
    <div class="text-h2">{{ t("title") }}</div>

    <q-list class="q-mt-lg">
        <!-- General -->
        <q-item-label header>{{ t("general") }}</q-item-label>
        <q-item>
            <q-item-section avatar>
                <q-icon name="translate" />
            </q-item-section>
            <q-item-section>
                <q-item-label>{{ t("language") }}</q-item-label>
            </q-item-section>

            <q-item-section side center>
                <q-select
                    borderless
                    dense
                    :model-value="language"
                    @update:model-value="changeLanguage"
                    :options="['pl', 'en']"
                ></q-select>
            </q-item-section>
        </q-item>
        <q-separator spaced inset />

        <q-item>
            <q-item-section avatar>
                <q-icon name="mdi-brightness-6" />
            </q-item-section>

            <q-item-section center>
                <q-item-label> {{ t("theme") }} </q-item-label>
            </q-item-section>

            <q-item-section side center>
                <q-toggle color="positive" :model-value="isDark" @update:model-value="theme"> </q-toggle>
            </q-item-section>
        </q-item>
        <q-separator spaced inset />

        <!-- App -->
        <q-item-label header>{{ t("app") }}</q-item-label>
        <q-item>
            <q-item-section avatar>
                <q-icon name="mdi-counter" />
            </q-item-section>

            <q-item-section center>
                <q-item-label> {{ t("version") }} </q-item-label>
            </q-item-section>

            <q-item-section side center> {{ tauriVersion }} </q-item-section>
        </q-item>
        <q-separator spaced inset />
        <q-item>
            <q-item-section avatar>
                <q-icon name="mdi-update" />
            </q-item-section>

            <q-item-section center>
                <q-item-label> {{ t("update") }} </q-item-label>
            </q-item-section>

            <q-item-section side center>
                <q-btn round size="small" color="primary" icon="mdi-download" />
            </q-item-section>
        </q-item>
        <q-separator spaced inset />
    </q-list>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n"
import { getVersion } from "@tauri-apps/api/app"
import { useSetting } from "@/composables/useSetting"

const { t } = useI18n()
const { changeLanguage, isDark, language, theme } = useSetting()
const tauriVersion = await getVersion()
</script>

<i18n>
    "en": {
        "title": "Settings",
        "theme": "Dark mode",
        "general": "General",
        "language": "Select language",
        "app": "App",
        "version": "Version",
        "update": "Update",
    }

    "pl": {
        "title": "Ustawienia",
        "theme": "Ciemny motyw",
        "general": "Ogólne",
        "language": "Wybierz język",
        "app": "Aplikacja",
        "version": "Wersja",
        "update": "Aktualizacja",
    }
</i18n>
