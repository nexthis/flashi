import { getUserOrThrow } from "@/utils/auth"
import { getFirestore, collection, query, getDocs, QuerySnapshot, DocumentData } from "firebase/firestore"

export async function list(): Promise<QuerySnapshot<DocumentData>> {
    const db = getFirestore()
    const user = await getUserOrThrow()

    console.debug("Call list")

    const q = query(collection(db, "users", user.uid, "macros"))
    return await getDocs(q)
}
list.key = "macros"

//export async function get(): Promise<QuerySnapshot<DocumentData>>

//export async function create(): Promise<QuerySnapshot<DocumentData>>

//export async function update(): Promise<QuerySnapshot<DocumentData>>
