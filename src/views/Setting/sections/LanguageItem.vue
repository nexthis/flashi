<template>
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
</template>

<script lang="ts" setup>
import { useI18n } from "vue-i18n"
import { useLanguage } from "@/composables/settings/useLanguage"

const { t } = useI18n()
const { language, changeLanguage } = useLanguage()
</script>

<i18n>
    "en": {
        "language": "Select language",
    }

    "pl": {
        "language": "Wybierz język",
    }
</i18n>
