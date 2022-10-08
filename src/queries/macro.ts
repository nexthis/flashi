import { getUserOrThrow } from "@/utils/auth"
import {
    getFirestore,
    collection,
    query,
    getDocs,
    addDoc,
    QuerySnapshot,
    DocumentData,
    DocumentReference,
} from "firebase/firestore"

// Because user can have only max 30~100 we fetch all because is cheaper ;-))
export async function paginate(): Promise<QuerySnapshot<DocumentData>> {
    const db = getFirestore()
    const user = await getUserOrThrow()

    console.debug("Call list")

    const q = query(collection(db, "users", user.uid, "macros"))
    return await getDocs(q)
}
paginate.key = "macros"

//export async function get(): Promise<QuerySnapshot<DocumentData>>

export async function create(data: Partial<UserMacro>): Promise<DocumentReference<DocumentData>> {
    const db = getFirestore()
    const user = await getUserOrThrow()

    const collect = collection(db, "users", user.uid, "macros")

    return await addDoc(collect, data)
}

//export async function update(): Promise<QuerySnapshot<DocumentData>>
