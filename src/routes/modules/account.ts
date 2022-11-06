import type { RouteRecordRaw } from "vue-router"
import AsideLayout from "@/layouts/AsideLayout.vue"
import Account from "@/views/Account/AccountView.vue"

const routes: RouteRecordRaw = {
    path: "/account",
    component: AsideLayout,
    children: [
        {
            path: "",
            name: "account",
            component: Account,
        },
    ],
}

export default routes
