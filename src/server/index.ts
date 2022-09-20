import { connectionListener } from "./connect"
import { register } from "./registerDevice"

export default {
    install: () => {
        register()
        connectionListener()
    },
}
