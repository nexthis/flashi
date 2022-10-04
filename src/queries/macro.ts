import { getAuth } from "firebase/auth"
import { getFirestore, collection, query, getDocs, QuerySnapshot, DocumentData } from "firebase/firestore"

export async function list(): Promise<QuerySnapshot<DocumentData>> {
    const auth = getAuth()
    const db = getFirestore()

    if (!auth.currentUser) {
        throw Error("User not define!")
    }

    const q = query(collection(db, "users", auth.currentUser.uid, "macros"))
    return await getDocs(q)
}

//export async function get(): Promise<QuerySnapshot<DocumentData>>

//export async function create(): Promise<QuerySnapshot<DocumentData>>

//export async function update(): Promise<QuerySnapshot<DocumentData>>
