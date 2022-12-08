import type { RouteRecordRaw } from "vue-router"
import AsideLayout from "@/layouts/AsideLayout.vue"
import Macro from "@/views/Macro/MacroView.vue"
import Editor from "@/views/Macro/EditorView.vue"

const routes: RouteRecordRaw = {
    path: "/macro",
    component: AsideLayout,
    children: [
        {
            path: "",
            name: "macro",
            component: Macro,
        },
        {
            path: "editor",
            name: "macro.create",
            component: Editor,
        },
        {
            path: "editor/:id",
            name: "macro.editor",
            component: Editor,
        },
    ],
}

export default routes
