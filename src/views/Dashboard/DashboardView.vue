<template>
    <div class="text-h2">{{ t("title") }}</div>

    <div class="row q-col-gutter-md q-mt-lg">
        <div class="col-6">
            <devices />
        </div>
        <div class="col-6">
            <last-activity />
        </div>
    </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n"
import Devices from "./sections/DevicesSection.vue"
import LastActivity from "./sections/LastActivitySection.vue"

const { t } = useI18n()
</script>

<style scoped></style>

<i18n>
    "en": {
        "title": "Dashboard"
    }

    "pl": {
        "title": "Panel"
    }
</i18n>
