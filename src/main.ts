import { createApp } from "vue"
import "@/plugins/firebase"
import Quasar, { config } from "@/plugins/quasar"
import routes from "@/routes"
import App from "./App.vue"

const app = createApp(App)

app.use(Quasar, config)
app.use(routes)

app.mount("#app")
