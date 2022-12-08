import { createRouter, createWebHashHistory } from "vue-router"
import routes from "@/routes/modules/index"
import { checkAuth } from "@/utils/auth"

const router = createRouter({
    // 4. Provide the history implementation to use. We are using the hash history for simplicity here.
    history: createWebHashHistory(),
    routes, // short for `routes: routes`
})

router.beforeEach(async (to, from, next) => {
    const isAuth = await checkAuth()
    if (to.meta?.auth === false) {
        return next()
    }
    if (!isAuth) {
        console.log("un auth")

        return next({ name: "login" })
    }

    return next()
})

export default router
