import { list } from "@/queries/macro"
import { useQuery } from "@tanstack/vue-query"
import type { QuerySnapshot, DocumentData } from "firebase/firestore"

export function useMacroList() {
    const result = useQuery<QuerySnapshot<DocumentData>, Error, Array<UserMacro>>([list.key], list, {
        select: (data) => data.docs.map((item) => ({ uuid: item.id, ...item.data() })) as Array<UserMacro>,
        staleTime: 1000 * 60 * 1,
    })

    return result
}
