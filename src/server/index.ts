import { connectionListener } from "./connect"
import { register } from "./registerDevice"

export default {
    install: () => {
        console.log("installed")
        register()
        connectionListener()
    },
    // unmount: () => {
    //     console.log("unmount")
    // },
}
