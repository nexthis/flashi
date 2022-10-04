import { useAuth } from "@/store/useAuth"
import { getFirestore, addDoc, doc, collection, deleteDoc } from "firebase/firestore"
import { KEYS } from "@/constants/storage"

//TODO: Refactor task: https://trello.com/c/zSXjvU1P
export function useMacroModifier() {
    const auth = useAuth()
    const db = getFirestore()

    async function updateOrCreate(params: any) {
        const document = collection(db, "users", auth!.user!.uid, "macros")
        localStorage.removeItem(KEYS.macros)
        await addDoc(document, params)
    }

    async function remove(uuid: string) {
        const document = doc(db, "users", auth!.user!.uid, "macros", uuid)
        localStorage.removeItem(KEYS.macros)
        await deleteDoc(document)
    }

    return {
        updateOrCreate,
        remove,
    }
}
