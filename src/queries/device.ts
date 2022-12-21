import { getFirestore, collection, query, getDocs, QuerySnapshot, DocumentData } from "firebase/firestore"
import { getUserOrThrow } from "@/utils/auth"

// Because user can have only max 30~100 we fetch all because is simple but is expensive :-((
export async function all(): Promise<QuerySnapshot<DocumentData>> {
    const db = getFirestore()
    const user = await getUserOrThrow()

    const q = query(collection(db, "users", user.uid, "devices"))
    return await getDocs(q)
}
all.key = "devices"
