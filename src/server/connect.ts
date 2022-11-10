import { getAuth } from "firebase/auth"
import { getFirestore, doc, collection, addDoc, setDoc, onSnapshot, DocumentData } from "firebase/firestore"
import { User } from "firebase/auth"
import { invoke } from "@tauri-apps/api/tauri"
import { emit, listen } from "@tauri-apps/api/event"
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
    const connectionPool = doc(db, "users", user.uid, "connection_pool", "offer")

    onSnapshot(connectionPool, (snapshot) => {
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

    const offerCandidates = collection(db, "users", user.uid, "offer_candidates")
    onSnapshot(offerCandidates, (snapshot) => {
        snapshot.docChanges().forEach(async (type) => {
            if (type.type === "added") {
                const ice = type.doc.data() as RTCIceCandidateInterface
                console.log(ice)
                emit("remote-ice-candidate", {
                    candidate: ice.candidate,
                    sdpMLineIndex: ice.sdpMLineIndex,
                    sdpMid: ice.sdpMid,
                })
            }
        })
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

listen<RTCIceCandidateInterface>("on-ice-candidate", (event) => {
    const auth = getAuth()
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const user = auth.currentUser!
    addDoc(collection(db, "users", user.uid, "answer_candidates"), {
        ...event.payload,
    })
})
//candidate:842163049 1 udp 1677729535 37.47.225.160 24504 typ srflx raddr 0.0.0.0 rport 0 generation 0 ufrag 1/u9 network-cost 999
