import { getFirestore, doc, onSnapshot, DocumentData } from "firebase/firestore"
import { getAuth, User } from "firebase/auth"
import { invoke } from "@tauri-apps/api/tauri"
import _ from "lodash"

interface OfferInterface {
    value: null | undefined | DocumentData
    isInit: boolean
}

const db = getFirestore()
const auth = getAuth()
const offer: OfferInterface = { value: null, isInit: false }

export async function connectionListener() {
    //TODO: Refactor
    const user = (await new Promise((resolve, reject) => {
        const unsubscribe = auth.onAuthStateChanged(
            (user) => {
                unsubscribe()
                console.log("connectionListener waiting for user !refactor!")

                if (user) {
                    resolve(user)
                }
            },
            () => reject(false)
        )
    })) as User

    const q = doc(db, "users", user.uid, "connection_pool", "offer")

    onSnapshot(q, (snapshot) => {
        if (!offer.isInit) {
            offer.value = snapshot.data()
            offer.isInit = true
            return
        }
        offer.value = snapshot.data()
        if (_.has(offer, "value.sdp") && _.has(offer, "value.type")) {
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            //@ts-ignore
            establishConnection(offer.value)
        }
    })
}

function establishConnection(offer: { sdp: string; type: "offer" }) {
    console.log("connect", offer)
    invoke("connect")
}
