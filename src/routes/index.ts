import routes from "@/routes/modules/index"
import { createRouter, createWebHashHistory } from "vue-router"

const router = createRouter({
    // 4. Provide the history implementation to use. We are using the hash history for simplicity here.
    history: createWebHashHistory(),
    routes, // short for `routes: routes`
})

// // Creates a `nextMiddleware()` function which not only
// // runs the default `next()` callback but also triggers
// // the subsequent Middleware function.
// function nextFactory(context, middleware, index) {
//     const subsequentMiddleware = middleware[index]
//     // If no subsequent Middleware exists,
//     // the default `next()` callback is returned.
//     if (!subsequentMiddleware) return context.next
//
//     return (...parameters) => {
//         // Run the default Vue Router `next()` callback first.
//         context.next(...parameters)
//         // Than run the subsequent Middleware with a new
//         // `nextMiddleware()` callback.
//         const nextMiddleware = nextFactory(context, middleware, index + 1)
//         subsequentMiddleware({ ...context, next: nextMiddleware })
//     }
// }
//
// router.beforeEach((to, from, next) => {
//     if (to.meta.middleware) {
//         const middleware = Array.isArray(to.meta.middleware) ? to.meta.middleware : [to.meta.middleware]
//
//         const context = {
//             from,
//             next,
//             router,
//             to,
//         }
//         const nextMiddleware = nextFactory(context, middleware, 1)
//
//         return middleware[0]({ ...context, next: nextMiddleware })
//     }
//
//     return next()
// })

export default router
