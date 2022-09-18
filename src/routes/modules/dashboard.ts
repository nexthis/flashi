import type { RouteRecordRaw } from "vue-router"
import AssideLayout from "@/layouts/AssideLayout.vue"
import Dashboard from "@/views/Dashboard/DashboardView.vue"

const routes: RouteRecordRaw = {
    path: "/",
    component: AssideLayout,
    children: [
        {
            path: "",
            name: "dashboard",
            component: Dashboard,
        },
    ],
}

export default routes
