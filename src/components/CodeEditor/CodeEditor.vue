<template>
    <div class="box" ref="editorRef"></div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue"
import { EditorView, basicSetup } from "codemirror"
//import { EditorState } from "@codemirror/state"

const editorRef = ref()
let editor: EditorView

onMounted(() => {
    editor = new EditorView({
        extensions: [
            basicSetup,
            EditorView.updateListener.of((value) => {
                //console.log(value.state.doc.text)
            }),
        ],
        parent: editorRef.value,
    })
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
