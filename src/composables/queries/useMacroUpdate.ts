import { useMutation, useQueryClient } from "@tanstack/vue-query"
import { update, all } from "@/queries/macro"

export function useMacroUpdate() {
    const queryClient = useQueryClient()

    const mutation = useMutation(update, {
        onSuccess: () => {
            queryClient.invalidateQueries([all.key])
        },
    })

    return mutation
}
