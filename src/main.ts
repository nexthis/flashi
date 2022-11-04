import "@/plugins/firebase"
import { VueQueryPlugin } from "@tanstack/vue-query"
import { createPinia } from "pinia"
import { createApp } from "vue"
import Quasar, { config } from "@/plugins/quasar"
import routes from "@/routes"
import App from "@/App.vue"
import i18n from "@/plugins/i18n"
import server from "@/server"

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.use(server)
app.use(Quasar, config)
app.use(routes)
app.use(i18n)
app.use(VueQueryPlugin)

app.mount("#app")
