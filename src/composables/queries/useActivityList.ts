import { useQuery } from "@tanstack/vue-query"
import type { QuerySnapshot, DocumentData } from "firebase/firestore"
import { last } from "@/queries/connection"

// Because user can have only max 30~100 we fetch all because is cheaper / firebase doesn't support full pagination yet ;-))
export function useActivityList() {
    return useQuery<QuerySnapshot<DocumentData>, Error, Array<RTCConnectionMarkInterface>>([last.key], last, {
        select: (request) => {
            const items = request.docs.map((item) => item.data())

            return items as Array<RTCConnectionMarkInterface>
        },
        staleTime: 1000 * 60 * 1,
    })
}
