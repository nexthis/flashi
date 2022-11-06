<template>
    <div class="flex column full-height">
        <div class="flex items-center justify-between">
            <div class="text-h2">{{ t("title") }}</div>
        </div>
        <div ref="editor" class="flex-1">
            <code-editor />
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue"
import { useI18n } from "vue-i18n"
import { useRouter } from "vue-router"
import { useResizeObserver } from "@vueuse/core"
import { useMacroCreate } from "@/composables/queries/useMacroCreate"
import CodeEditor from "@/components/CodeEditor/CodeEditor.vue"

const { mutateAsync } = useMacroCreate()

const editor = ref()
const name = ref("")
const code = ref("")

const { t } = useI18n()
const routes = useRouter()

useResizeObserver(editor, (entries) => {
    const entry = entries[0]
    console.log(entry)

    const { height } = entry.contentRect
    const element = editor.value?.querySelector(".cm-editor") as HTMLDivElement
    element.style.height = `${height}px`
})

const onModifier = async () => {
    await mutateAsync({ name: name.value, code: code.value })
    routes.back()
}
</script>

<style lang="scss" scoped>
.flex-1 {
    flex: 1;
}
</style>

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
