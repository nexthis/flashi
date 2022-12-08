import {
    getFirestore,
    collection,
    doc,
    deleteDoc,
    query,
    getDocs,
    getDoc,
    addDoc,
    updateDoc,
    QuerySnapshot,
    DocumentData,
    DocumentReference,
    DocumentSnapshot,
} from "firebase/firestore"
import { getUserOrThrow } from "@/utils/auth"

// Because user can have only max 30~100 we fetch all because is simple but is expensive :-((
export async function all(): Promise<QuerySnapshot<DocumentData>> {
    const db = getFirestore()
    const user = await getUserOrThrow()

    console.debug("Call list")

    const q = query(collection(db, "users", user.uid, "macros"))
    return await getDocs(q)
}
all.key = "macros"

export async function get(uuid: string): Promise<DocumentSnapshot<DocumentData>> {
    const db = getFirestore()
    const user = await getUserOrThrow()
    const document = doc(db, "users", user.uid, "macros", uuid)
    return getDoc(document)
}
get.key = "macro"

export async function create(data: Partial<UserMacro>): Promise<DocumentReference<DocumentData>> {
    const db = getFirestore()
    const user = await getUserOrThrow()

    const collect = collection(db, "users", user.uid, "macros")

    return await addDoc(collect, data)
}
create.key = "macro.create"

export async function update(data: UserMacro): Promise<void> {
    const db = getFirestore()
    const user = await getUserOrThrow()
    const document = doc(db, "users", user.uid, "macros", data.uuid)
    await updateDoc(document, data)
}

export async function remove(data: UserMacro): Promise<void> {
    const db = getFirestore()
    const user = await getUserOrThrow()
    const document = doc(db, "users", user.uid, "macros", data.uuid)
    await deleteDoc(document)
}
remove.key = "macro.remove"
