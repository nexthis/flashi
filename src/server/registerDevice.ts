import { User } from "firebase/auth"
import { invoke } from "@tauri-apps/api/tauri"

import { getFirestore, doc, setDoc } from "firebase/firestore"

const db = getFirestore()

export async function register(user: User): Promise<DeviceInterface> {
    const result = await invoke<DeviceInterface>("registry")
    const document = doc(db, "users", user.uid, "devices", result.key)
    setDoc(document, result, { merge: true })
    return result
}
