import { useMutation, useQueryClient } from "@tanstack/vue-query"
import { create, paginate } from "@/queries/macro"

export function useMacroCreate() {
    const queryClient = useQueryClient()

    const mutation = useMutation(create, {
        onSuccess: () => {
            queryClient.invalidateQueries([paginate.key])
        },
    })

    return mutation
}
