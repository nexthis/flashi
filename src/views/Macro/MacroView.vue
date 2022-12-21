<template>
    <div class="layout-base">
        <div class="flex items-center justify-between">
            <div class="text-h2">{{ t("title") }}</div>
            <q-btn :to="{ name: 'macro.create' }" color="primary">{{ t("create") }}</q-btn>
        </div>
        <div class="spacer"></div>

        <div class="flex flex-center flex-1" v-if="isLoading">
            <q-spinner-puff color="primary" size="40%" />
        </div>

        <div class="layout-grid" v-else>
            <macro-card :item="item" v-for="item in data?.data" :key="item.uuid" />
        </div>
        <div class="flex-1"></div>
        <div class="q-pa-lg flex flex-center">
            <q-pagination
                v-model="currentPage"
                @update:model-value="page"
                :max="data.max"
                size="15px"
                direction-links
            />
        </div>
    </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n"
import { ref } from "vue"
import MacroCard from "@/components/Macro/MacroCard.vue"
import { useMacroPaginate } from "@/composables/queries/useMacroPaginate"

const currentPage = ref(1)

const { t } = useI18n()

const { data, isLoading, page } = useMacroPaginate()
</script>

<style scoped lang="scss">
.layout-base {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
}

.layout-grid {
    display: grid;
    gap: 25px;
    grid-template-columns: repeat(2, minmax(0, 1fr));
}
.spacer {
    height: 30px;
}
.flex-1 {
    flex: 1;
}
</style>

<i18n>
    "en": {
        "title": "Macro",
        "create": "Create",
    }

    "pl": {
        "title": "Macro",
        "create": "Utwórz",
    }
</i18n>
