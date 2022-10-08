import type { QuerySnapshot, DocumentData } from "firebase/firestore"
import { paginate } from "@/queries/macro"
import { useQuery } from "@tanstack/vue-query"
import { computed, ref } from "vue"
import _ from "lodash"

// Because user can have only max 30~100 we fetch all because is cheaper / firebase doesn't support full pagination yet ;-))
export function useMacroPaginate() {
    const data = ref<Paginate<UserMacro>>()
    const currentPage = ref(1)
    const max = ref(0)

    const query = useQuery<QuerySnapshot<DocumentData>, Error, Paginate<UserMacro>>(
        [paginate.key],
        paginate,
        {
            select: (request) => {
                const items = request.docs.map((item) => ({ uuid: item.id, ...item.data() }))
                const result = {
                    data: items,
                    max: request.docs.length,
                    next: false,
                    previous: 1,
                } as Paginate<UserMacro>

                data.value = result
                return result
            },
            staleTime: 1000 * 60 * 1,
        }
    )

    const pagginate = (): Paginate<UserMacro> => {
        const result = _.chunk(data.value?.data, 4)

        const previous = currentPage.value >= 2 ? currentPage.value - 1 : false
        const next = currentPage.value < result.length ? currentPage.value + 1 : false
        max.value = result.length

        return {
            data: result[currentPage.value - 1],
            max: result.length,
            previous,
            next,
        }
    }

    const next = () => {
        currentPage.value = _.clamp(currentPage.value + 1, 1, max.value)
    }

    const previous = () => {
        currentPage.value = _.clamp(currentPage.value - 1, 1, max.value)
    }

    const page = (value: number) => {
        currentPage.value = _.clamp(value, 1, max.value)
    }

    return { ...query, next, page, previous, data: computed(() => pagginate()) }
}
