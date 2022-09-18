import type { RouteRecordRaw } from "vue-router"
import SignIn from "@/views/Auth/SignInView.vue"

const routes: RouteRecordRaw = {
    path: "/auth",
    name: "auth",
    component: SignIn,
    meta: { auth: false },
}

export default routes
