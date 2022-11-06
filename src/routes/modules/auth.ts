import type { RouteRecordRaw } from "vue-router"
import SignIn from "@/views/Auth/SignInView.vue"
import RegisterView from "@/views/Auth/RegisterView.vue"

const routes: RouteRecordRaw = {
    path: "/auth",
    children: [
        {
            path: "login",
            name: "login",
            component: SignIn,
            meta: { auth: false },
        },
        {
            path: "registry",
            name: "registry",
            component: RegisterView,
            meta: { auth: false },
        },
    ],
}

export default routes
