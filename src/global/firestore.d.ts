interface RTCBaseInterface {
    server: string
    client: string
    createdAt: Timestamp
}

interface RTCConnectionMarkInterface extends RTCBaseInterface {
    sdp: string
    type: "offer" | "answer"
}

interface RTCIceCandidateInterface extends RTCBaseInterface {
    candidate: string
    sdpMid: string
    sdpMLineIndex: number
    usernameFragment: null
    type: "ice"
}

interface UserMacro {
    uuid: string
    code: string
    name: string
}

interface DeviceInterface {
    name: string
    key: string
    os: string
}
