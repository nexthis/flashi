import { getFirestore, doc, onSnapshot, DocumentData } from "firebase/firestore"
import { getAuth, User } from "firebase/auth"
import _ from "lodash"

type OfferType = null | undefined | DocumentData

const db = getFirestore()
const auth = getAuth()
let offer: OfferType = null

export async function connectionListener() {
    const user = (await new Promise((resolve, reject) => {
        const unsubscribe = auth.onAuthStateChanged(
            (user) => {
                unsubscribe()
                console.log("wait")

                if (user) {
                    resolve(user)
                }
            },
            () => reject(false)
        )
    })) as User

    const q = doc(db, "users", user.uid, "connection_pool", "offer")

    onSnapshot(q, (snapshot) => {
        if (!offer) {
            offer = snapshot.data()
            return
        }
        offer = snapshot.data()
        if (_.has(offer, "sdp") && _.has(offer, "type")) {
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            //@ts-ignore
            establishConnection(offer)
        }
    })
}

function establishConnection(offer: { sdp: string; type: "offer" }) {
    console.log("connect", offer)
}
