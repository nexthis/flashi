<template>
    <div class="flex column full-height">
        <div class="flex items-center justify-between bar">
            <div @click="isNameEdit = true" v-show="!isNameEdit" class="text-h4 text-grey">
                {{ name }}
            </div>
            <q-input
                v-model="name"
                v-show="isNameEdit"
                @keypress.enter="isNameEdit = false"
                @blur="isNameEdit = false"
                borderless
                input-class="text-h4"
            />
            <q-btn @click="onModifier" size="small" color="primary">{{ isEdit ? "Update" : "Create" }}</q-btn>
        </div>
        <div class="flex-1">
            <!-- <div class="flex flex-center flex-1" v-if="isLoading">
                <q-spinner-puff color="primary" size="40%" />
            </div> -->
            <code-editor v-model="code" />
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue"
import { useI18n } from "vue-i18n"
import { useRoute, useRouter } from "vue-router"
import { useMacroCreate } from "@/composables/queries/useMacroCreate"
import CodeEditor from "@/components/CodeEditor/CodeEditor.vue"
import { useMacroGet } from "@/composables/queries/useMacroGet"
import { useMacroUpdate } from "@/composables/queries/useMacroUpdate"

const { mutateAsync: mutateAsyncCreate } = useMacroCreate()
const { mutateAsync: mutateAsyncUpdate } = useMacroUpdate()

const { t } = useI18n()
const routes = useRouter()
const route = useRoute()
const { data, suspense } = useMacroGet(route.params.id as string)

const isNameEdit = ref(false)
const isEdit = !!route.params.id
const documentId = route.params.id
const name = ref(t("title"))
const code = ref("")

if (isEdit) {
    await suspense()
    name.value = data.value!.name
    code.value = data.value!.code
}

const onModifier = async () => {
    if (isEdit) {
        await mutateAsyncUpdate({ uuid: documentId as string, name: name.value, code: code.value })
    } else {
        await mutateAsyncCreate({ name: name.value, code: code.value })
    }

    routes.back()
}

onMounted(async () => {
    document.body.style.backgroundColor = "#282c34"
})

onUnmounted(() => {
    document.body.style.backgroundColor = ""
})
</script>

<style lang="scss" scoped>
.flex-1 {
    flex: 1;
}
.bar {
    height: 56px;
}
</style>

<i18n>
    "en": {
        "title": "Name",
        "create": "Add",
    }

    "pl": {
        "title": "Nazwa",
        "create": "Dodaj",
    }
</i18n>
