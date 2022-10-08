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
            <q-card v-for="item in data?.data" :key="item.uuid">
                <q-card-section>
                    {{ item.uuid }}
                    <div class="text-h6">{{ item.name }}</div>
                </q-card-section>
                <q-card-actions align="right">
                    <q-btn color="accent" @click="onDelete(item)" round>
                        <q-icon name="mdi-delete" />
                    </q-btn>
                    <q-btn color="accent" round>
                        <q-icon name="mdi-eye" />
                    </q-btn>
                    <q-btn color="accent" round>
                        <q-icon name="mdi-pencil" />
                    </q-btn>
                    <q-btn color="accent" @click="onRun(item)" round>
                        <q-icon name="mdi-play" />
                    </q-btn>
                </q-card-actions>
            </q-card>
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
/* globals  UserMacro*/
import { useQuasar } from "quasar"
import { useMacroPaginate } from "@/composables/queries/useMacroPaginate"
import { useScript } from "@/composables/useScript"
import { useI18n } from "vue-i18n"
import { ref } from "vue"

const currentPage = ref(1)

const { t } = useI18n()
const quasar = useQuasar()
const { data, isLoading, page } = useMacroPaginate()
const { run } = useScript()

const onDelete = async (value: UserMacro) => {
    quasar
        .dialog({
            title: t("delete.title"),
            message: t("delete.message"),
            cancel: true,
        })
        .onOk(async () => {
            console.log(value)
        })
}

const onRun = async (value: UserMacro) => {
    await run(value.code)
}
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
        "delete": {
            "title": "Delete macro",
            "message": "You are sure you want to delete this item? ",
        },
    }

    "pl": {
        "title": "Macro",
        "create": "Utwórz",
        "delete": {
            "title": "Usuwanie makra ",
            "message": "Jesteś pewien że chcesz usunąć element? ",
        },
    }
</i18n>
