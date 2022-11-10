type RTCPeerConnectionState =
    | "new"
    | "unspecified"
    | "connecting"
    | "connected"
    | "disconnected"
    | "failed"
    | "closed"

interface RTCIceCandidateInterface {
    candidate: string
    sdpMid: string
    sdpMLineIndex: number
    usernameFragment: null
}
