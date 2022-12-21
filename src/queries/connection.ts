// FOR SERVER AND CLIENT Collection
import {
    getFirestore,
    collection,
    query,
    getDocs,
    QuerySnapshot,
    DocumentData,
    where,
    orderBy,
    limit,
} from "firebase/firestore"
import { getUserOrThrow } from "@/utils/auth"

/**
 * Get Last 10 connection to server
 * @returns
 */
export async function last(): Promise<QuerySnapshot<DocumentData>> {
    const db = getFirestore()
    const user = await getUserOrThrow()

    const q = query(
        collection(db, "users", user.uid, "server"),
        where("type", "==", "offer"),
        orderBy("createdAt", "desc"),
        limit(6)
    )
    return await getDocs(q)
}
last.key = "connection.last"
