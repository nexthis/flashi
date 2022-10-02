<template>
    <div class="flex items-center justify-between">
        <div class="text-h2">{{ t("title") }}</div>
    </div>
    <q-input v-model="name" label="name" />
    <q-input v-model="code" label="code" type="textarea" />
    <q-btn color="primary" @click="onModifier">Save</q-btn>
</template>

<script setup lang="ts">
import { useMacroModifier } from "@/composables/useMacroModifier"
import { ref } from "vue"
import { useI18n } from "vue-i18n"
import { useRouter } from "vue-router"

const name = ref("")
const code = ref("")

const { t } = useI18n()
const routes = useRouter()
const { updateOrCreate } = useMacroModifier()

const onModifier = async () => {
    await updateOrCreate({ name: name.value, code: code.value })
    routes.back()
}
</script>

<i18n>
    "en": {
        "title": "Awesome macro!",
        "create": "Add",
    }

    "pl": {
        "title": "Create",
        "create": "Dodaj",
    }
</i18n>
