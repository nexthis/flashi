interface UserConnectionPool {
    sdp: string
    type: "answer" | "offer"
}

interface UserMacro {
    uuid: string
    code: string
    name: string
}
