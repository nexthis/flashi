<template>
    <q-card>
        <q-card-section>
            <div class="text-h6">{{ item.name }}</div>
        </q-card-section>
        <q-card-actions align="right">
            <q-btn color="accent" @click="onDelete(item)" round>
                <q-icon name="mdi-delete" />
            </q-btn>
            <q-btn color="accent" round>
                <q-icon name="mdi-eye" />
            </q-btn>
            <q-btn color="accent" @click="onEdit(item)" round>
                <q-icon name="mdi-pencil" />
            </q-btn>
            <q-btn color="accent" @click="onRun(item)" round>
                <q-icon name="mdi-play" />
            </q-btn>
        </q-card-actions>
    </q-card>
</template>

<script setup lang="ts">
/* global UserMacro */
import { useRouter } from "vue-router"
import { useQuasar } from "quasar"
import { useI18n } from "vue-i18n"
import { useMacroRemove } from "@/composables/queries/useMacroRemove"
import { useScript } from "@/composables/useScript"

defineProps<{
    item: UserMacro
}>()

const quasar = useQuasar()
const routes = useRouter()

const { t } = useI18n()
const { mutate } = useMacroRemove()
const { run } = useScript()

const onDelete = (value: UserMacro) => {
    quasar
        .dialog({
            title: t("delete.title"),
            message: t("delete.message"),
            cancel: true,
        })
        .onOk(() => {
            mutate(value)
        })
}

const onEdit = (value: UserMacro) => {
    routes.push({ name: "macro.editor", params: { id: value.uuid } })
}

const onRun = async (value: UserMacro) => {
    await run(value.code)
}
</script>

<i18n>
    "en": {
        "delete": {
            "title": "Delete macro",
            "message": "You are sure you want to delete this item? ",
        },
    }

    "pl": {
        "delete": {
            "title": "Usuwanie makra ",
            "message": "Jesteś pewien że chcesz usunąć element? ",
        },
    }
</i18n>
