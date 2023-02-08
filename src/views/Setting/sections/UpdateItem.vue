<template>
    <q-item>
        <q-item-section avatar>
            <q-icon name="mdi-update" />
        </q-item-section>

        <q-item-section center>
            <q-item-label> {{ t("update") }} </q-item-label>
        </q-item-section>

        <q-item-section side center>
            <q-btn @click="onUpdate" round size="small" color="primary" icon="mdi-download" />
        </q-item-section>
    </q-item>
</template>

<script lang="ts" setup>
import { checkUpdate, installUpdate } from "@tauri-apps/api/updater"
import { relaunch } from "@tauri-apps/api/process"
import { useI18n } from "vue-i18n"
const { t } = useI18n()

const onUpdate = async () => {
    try {
        const { shouldUpdate, manifest } = await checkUpdate()
        if (shouldUpdate) {
            // display dialog
            await installUpdate()
            // install complete, restart the app
            await relaunch()
        }
    } catch (error) {
        console.log(error)
    }
}
</script>

<i18n>
    "en": {
        "update": "Update",
    }

    "pl": {
        "update": "Aktualizacja",
    }
</i18n>
