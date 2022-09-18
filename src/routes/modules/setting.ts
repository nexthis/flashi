import type { RouteRecordRaw } from "vue-router"
import AssideLayout from "@/layouts/AssideLayout.vue"
import Setting from "@/views/Setting/SettingView.vue"

const routes: RouteRecordRaw = {
    path: "/setting",
    component: AssideLayout,
    children: [
        {
            path: "",
            name: "setting",
            component: Setting,
        },
    ],
}

export default routes
