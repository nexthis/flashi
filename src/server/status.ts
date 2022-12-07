import { User } from "firebase/auth"
import { getDatabase, onDisconnect, set, ref, serverTimestamp } from "firebase/database"

const db = getDatabase()

export async function statusListener(user: User, device: DeviceInterface) {
    const query = ref(db, `/status/${user.uid}/${device.key}`)

    const payload = (isOnline = true) => ({
        active: isOnline,
        ...device,
        last_changed: serverTimestamp(),
    })

    //TODO: Test when internet come back
    await set(query, payload())

    onDisconnect(query).set(payload(false))
}
