import { getAuth } from "firebase/auth"
import { connectionListener } from "./connect"
import { statusListener } from "./status"
import { register } from "./registerDevice"

const auth = getAuth()

export default {
    install: () => {
        auth.onAuthStateChanged(async (user) => {
            console.log("change onAuthStateChanged")

            if (user) {
                const device = await register(user)
                connectionListener(user, device)
                statusListener(user, device)
            } else {
                //TODO: unregister
            }
        })
    },
}
