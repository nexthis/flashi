<template>
    <q-item>
        <q-item-section avatar>
            <q-icon name="mdi-refresh-auto" />
        </q-item-section>

        <q-item-section center>
            <q-item-label> {{ t("theme") }} </q-item-label>
        </q-item-section>
        <q-item-section side center>
            <q-toggle color="positive" :model-value="isEnable" @update:model-value="change"> </q-toggle>
        </q-item-section>
    </q-item>
</template>

<script lang="ts" setup>
import { useI18n } from "vue-i18n"
import { isEnabled, disable, enable } from "tauri-plugin-autostart-api"
import { ref } from "vue"

const isEnable = ref(await isEnabled())
const { t } = useI18n()

const change = async () => {
    if (isEnable.value) {
        await disable()
        isEnable.value = false
        return
    }
    await enable()
    isEnable.value = true
}
</script>

<i18n>
    "en": {
        "theme": "Auto start",
    }

    "pl": {
        "theme": "Auto start",
    }
</i18n>
