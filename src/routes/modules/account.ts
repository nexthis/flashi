import type { RouteRecordRaw } from "vue-router"
import AssideLayout from "@/layouts/AssideLayout.vue"
import Account from "@/views/Account/AccountView.vue"

const routes: RouteRecordRaw = {
    path: "/account",
    component: AssideLayout,
    children: [
        {
            path: "",
            name: "account",
            component: Account,
        },
    ],
}

export default routes
