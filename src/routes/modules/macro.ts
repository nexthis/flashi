import type { RouteRecordRaw } from "vue-router"
import AsideLayout from "@/layouts/AsideLayout.vue"
import Macro from "@/views/Macro/MacroView.vue"
import Create from "@/views/Macro/CreateView.vue"

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
            path: "create",
            name: "macro.create",
            component: Create,
        },
    ],
}

export default routes
