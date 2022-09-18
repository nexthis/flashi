<template>
    <div class="text-h2">{{ t("title") }}</div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n"

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
