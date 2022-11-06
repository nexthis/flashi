import type { RouteRecordRaw } from "vue-router"
import AsideLayout from "@/layouts/AsideLayout.vue"
import Setting from "@/views/Setting/SettingView.vue"

const routes: RouteRecordRaw = {
    path: "/setting",
    component: AsideLayout,
    children: [
        {
            path: "",
            name: "setting",
            component: Setting,
        },
    ],
}

export default routes
