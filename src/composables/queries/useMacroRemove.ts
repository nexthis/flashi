import { useMutation, useQueryClient } from "@tanstack/vue-query"
import { remove, all } from "@/queries/macro"

export function useMacroRemove() {
    const queryClient = useQueryClient()

    const mutation = useMutation(remove, {
        onSuccess: () => {
            //queryClient.
            queryClient.invalidateQueries([all.key])
        },
    })

    return mutation
}
