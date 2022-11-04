import { getFirestore, doc, setDoc, onSnapshot, DocumentData } from "firebase/firestore"
import { User } from "firebase/auth"
import { invoke } from "@tauri-apps/api/tauri"
import { listen } from "@tauri-apps/api/event"
import { Notify } from "quasar"

import _ from "lodash"
import { useConnection } from "@/store/useConnection"

interface OfferInterface {
    value: null | undefined | DocumentData
    isInit: boolean
}

const db = getFirestore()

const offer: OfferInterface = { value: null, isInit: false }

export async function connectionListener(user: User) {
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
            establishConnection(user, offer.value)
        }
        console.log("change onSnapshot")
    })
}

async function establishConnection(user: User, offer: { sdp: string; type: "offer" }) {
    try {
        const result = await invoke<{ sdp: string; type: string }>("connect", {
            offer: JSON.stringify(offer),
        })

        await setDoc(doc(db, "users", user.uid, "connection_pool", "answer"), result)
    } catch (err) {
        Notify.create({ color: "negative", message: "Connection fails ", position: "bottom-right" })
    }
}

listen<RTCPeerConnectionState>("peer-connection-state-change", (s) => {
    const connection = useConnection()

    if (s.payload === "connected") {
        connection.connect()
    } else {
        connection.disconnect()
    }
})
