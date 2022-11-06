import type { RouteRecordRaw } from "vue-router"
import AsideLayout from "@/layouts/AsideLayout.vue"
import Dashboard from "@/views/Dashboard/DashboardView.vue"

const routes: RouteRecordRaw = {
    path: "/",
    component: AsideLayout,
    children: [
        {
            path: "",
            name: "dashboard",
            component: Dashboard,
        },
    ],
}

export default routes
