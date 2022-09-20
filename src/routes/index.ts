import routes from "@/routes/modules/index"
import { createRouter, createWebHashHistory } from "vue-router"
import { getAuth } from "firebase/auth"

const router = createRouter({
    // 4. Provide the history implementation to use. We are using the hash history for simplicity here.
    history: createWebHashHistory(),
    routes, // short for `routes: routes`
})

router.beforeEach(async (to, from, next) => {
    const auth = getAuth()

    //TODO:Refactor
    const user = await new Promise((resolve, reject) => {
        const unsubscribe = auth.onAuthStateChanged(
            (user) => {
                unsubscribe()
                console.log("beforeEach waiting for user !refactor!")

                resolve(user)
            },
            () => reject(false)
        )
    })

    if (to.meta?.auth === false) {
        return next()
    }
    if (!user) {
        console.log("unauth")

        return next({ name: "auth" })
    }
    next()
})

export default router
