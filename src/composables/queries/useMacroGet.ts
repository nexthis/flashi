import { useQuery } from "@tanstack/vue-query"
import type { DocumentData, DocumentSnapshot } from "firebase/firestore"
import type { Ref } from "vue"
import { get } from "@/queries/macro"

type UUID = Ref<string> | string | null | undefined

export function useMacroGet(uuid: UUID) {
    return useQuery<DocumentSnapshot<DocumentData>, Error, UserMacro>(
        [get.key, uuid],
        ({ queryKey }) => get(queryKey[1] as string),
        {
            select: (request) => {
                return request.data() as UserMacro
            },
            enabled: !!uuid,
        }
    )
}
