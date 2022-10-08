import { create, paginate } from "@/queries/macro"
import { useMutation, useQueryClient } from "@tanstack/vue-query"

export function useMacroCreate() {
    const queryClient = useQueryClient()

    const mutation = useMutation(create, {
        onSuccess: () => {
            queryClient.invalidateQueries([paginate.key])
        },
    })

    return mutation
}
