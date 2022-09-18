import type { RouteRecordRaw } from "vue-router"
import AssideLayout from "@/layouts/AssideLayout.vue"
import Macro from "@/views/Macro/MacroView.vue"

const routes: RouteRecordRaw = {
    path: "/macro",
    component: AssideLayout,
    children: [
        {
            path: "",
            name: "macro",
            component: Macro,
        },
    ],
}

export default routes
