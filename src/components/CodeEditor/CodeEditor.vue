<template>
    <div class="box" ref="editorRef"></div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue"
import { EditorView, basicSetup } from "codemirror"
import { oneDark } from "@codemirror/theme-one-dark"
import { StreamLanguage } from "@codemirror/language"
import { flashi } from "./lang"

const props = defineProps<{ modelValue: string | undefined }>()
const emit = defineEmits(["update:modelValue"])

const editorRef = ref()
let editor: EditorView

onMounted(() => {
    editor = new EditorView({
        extensions: [
            basicSetup,
            EditorView.updateListener.of((value) => {
                //https://www.raresportan.com/how-to-make-a-code-editor-with-codemirror6/ Listen for Changes
                if (value.docChanged) {
                    emit("update:modelValue", value.state.doc.toString())
                }
            }),
            //flashi(),
            StreamLanguage.define(flashi),
            oneDark,
        ],
        parent: editorRef.value,
        doc: props.modelValue,
    })

    ///editor.state.doc.
})
</script>

<style>
.box {
    height: 100%;
    max-height: 100%;
    overflow: auto;
}
.cm-editor {
    height: 100%;
    max-height: 100%;
}
</style>
