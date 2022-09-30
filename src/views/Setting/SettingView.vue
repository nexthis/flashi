<template>
    <div class="text-h2">{{ t("title") }}</div>
    <q-toggle color="positive" v-model="test">Test</q-toggle>
</template>

<script setup lang="ts">
import { ref } from "vue"
import { useI18n } from "vue-i18n"

const test = ref(false)
const { t } = useI18n()
</script>

<i18n>
    "en": {
        "title": "Settings"
    }

    "pl": {
        "title": "Ustawienia"
    }
</i18n>
