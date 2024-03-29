import { useMutation, useQueryClient } from "@tanstack/vue-query"
import { create, all } from "@/queries/macro"

export function useMacroCreate() {
    const queryClient = useQueryClient()

    const mutation = useMutation(create, {
        onSuccess: () => {
            queryClient.invalidateQueries([all.key])
        },
    })

    return mutation
}
