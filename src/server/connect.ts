import { getAuth } from "firebase/auth"
import { User } from "firebase/auth"
import { invoke } from "@tauri-apps/api/tauri"
import { emit, listen } from "@tauri-apps/api/event"
import { Notify } from "quasar"
import {
    getFirestore,
    collection,
    addDoc,
    onSnapshot,
    query,
    where,
    Timestamp,
    serverTimestamp,
} from "firebase/firestore"

import { useConnection } from "@/store/useConnection"

//Global values
const db = getFirestore()
let clientKey = ""
let serverKey = ""

export async function connectionListener(user: User, device: DeviceInterface) {
    console.log("connectionListener")
    const q = query(
        collection(db, "users", user.uid, "server"),
        where("server", "==", device.key),
        where("createdAt", ">", Timestamp.now())
    )

    serverKey = device.key

    const onOffer = (offer: RTCConnectionMarkInterface) => {
        //add client key
        clientKey = offer.client
        establishConnection(user, offer)
    }

    const onIce = (ice: RTCIceCandidateInterface) => {
        console.log("Add Ice Candidate")

        emit("remote-ice-candidate", {
            candidate: ice.candidate,
            sdpMLineIndex: ice.sdpMLineIndex,
            sdpMid: ice.sdpMid,
        })
    }

    onSnapshot(q, (snapshot) => {
        const changes = snapshot.docChanges()
        for (const item of changes) {
            if (item.type !== "added") {
                console.log("connectionListener: " + item.type)
                break
            }
            const data = item.doc.data()

            switch (data.type) {
                case "offer":
                    console.log("connectionListener: " + item.type, data)
                    onOffer(data as RTCConnectionMarkInterface)
                    break
                case "ice":
                    onIce(data as RTCIceCandidateInterface)
                    break
                default:
                    console.log("connectionListener: Not identified type", data)
                    break
            }
        }
    })
}

async function establishConnection(user: User, offer: { sdp: string; type: string }) {
    try {
        const result = await invoke<{ sdp: string; type: string }>("connect", {
            offer: JSON.stringify(offer),
        })

        await addDoc(collection(db, "users", user.uid, "client"), {
            ...result,
            server: serverKey,
            client: clientKey,
            createdAt: serverTimestamp(),
        })
        // setTimeout(async () => {
        //     console.log(`add answer: `, result)
        //     await addDoc(collection(db, "users", user.uid, "client"), {
        //         ...result,
        //         server: serverKey,
        //         client: clientKey,
        //         createdAt: serverTimestamp(),
        //     })
        // }, 5000)
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
    addDoc(collection(db, "users", user.uid, "client"), {
        ...event.payload,
        server: serverKey,
        client: clientKey,
        type: "ice",
        createdAt: serverTimestamp(),
    })
})
//candidate:842163049 1 udp 1677729535 37.47.225.160 24504 typ srflx raddr 0.0.0.0 rport 0 generation 0 ufrag 1/u9 network-cost 999
