import type { RouteRecordRaw } from "vue-router"
import HomeView from "@/views/Home/HomeView.vue"

const routes: RouteRecordRaw = {
    path: "/",
    name: "home",
    component: HomeView,
}

export default routes
