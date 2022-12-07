<template>
    <div class="box" ref="editorRef"></div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue"
import { EditorView, basicSetup } from "codemirror"
import { EXAMPLE } from "flashi-language"
import { oneDark } from "@codemirror/theme-one-dark"
//import { EditorState } from "@codemirror/state"

const emit = defineEmits(["update:modelValue"])

const editorRef = ref()
let editor: EditorView

onMounted(() => {
    editor = new EditorView({
        extensions: [
            basicSetup,
            EditorView.updateListener.of((value) => {
                console.log(value.state.doc.toString())
                emit("update:modelValue", value.state.doc.toString())
            }),
            EXAMPLE(),
            oneDark,
        ],
        parent: editorRef.value,
    })

    //editor.se
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
    overflow: scroll;
}
</style>
