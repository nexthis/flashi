import { getAuth } from "firebase/auth"
import { connectionListener } from "./connect"
import { register } from "./registerDevice"

const auth = getAuth()

export default {
    install: () => {
        auth.onAuthStateChanged((user) => {
            if (user) {
                register(user)
                connectionListener(user)
            } else {
                //TODO: unregister
            }
        })
    },
}
