<template>
    <q-item>
        <q-item-section avatar>
            <q-icon name="mdi-counter" />
        </q-item-section>

        <q-item-section center>
            <q-item-label> {{ t("version") }} </q-item-label>
        </q-item-section>

        <q-item-section side center> {{ tauriVersion }} </q-item-section>
    </q-item>
</template>

<script lang="ts" setup>
import { getVersion } from "@tauri-apps/api/app"
import { useI18n } from "vue-i18n"
const { t } = useI18n()

const tauriVersion = await getVersion()
</script>

<i18n>
    "en": {
        "version": "Version",
    }

    "pl": {
         "version": "Wersja",
    }
</i18n>
