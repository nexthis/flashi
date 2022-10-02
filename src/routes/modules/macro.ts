import type { RouteRecordRaw } from "vue-router"
import AssideLayout from "@/layouts/AssideLayout.vue"
import Macro from "@/views/Macro/MacroView.vue"
import Create from "@/views/Macro/CreateView.vue"

const routes: RouteRecordRaw = {
    path: "/macro",
    component: AssideLayout,
    children: [
        {
            path: "",
            name: "macro",
            component: Macro,
        },
        {
            path: "create",
            name: "macro.create",
            component: Create,
        },
    ],
}

export default routes
