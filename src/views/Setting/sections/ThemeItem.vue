<template>
    <q-item>
        <q-item-section avatar>
            <q-icon name="mdi-update" />
        </q-item-section>

        <q-item-section center>
            <q-item-label> {{ t("theme") }} </q-item-label>
        </q-item-section>
        <q-item-section side center>
            <q-toggle color="positive" :model-value="isDark" @update:model-value="theme"> </q-toggle>
        </q-item-section>
    </q-item>
</template>

<script lang="ts" setup>
import { useI18n } from "vue-i18n"
import { useTheme } from "@/composables/settings/useTheme"

const { t } = useI18n()

const { isDark, theme } = useTheme()
</script>

<i18n>
    "en": {
        "theme": "Dark mode",
    }

    "pl": {
        "theme": "Ciemny motyw",
    }
</i18n>
