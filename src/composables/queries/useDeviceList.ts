import { useQuery } from "@tanstack/vue-query"
import type { QuerySnapshot, DocumentData } from "firebase/firestore"
import { all } from "@/queries/device"

// Because user can have only max 30~100 we fetch all because is cheaper / firebase doesn't support full pagination yet ;-))
export function useDeviceList() {
    return useQuery<QuerySnapshot<DocumentData>, Error, Array<DeviceInterface>>([all.key], all, {
        select: (request) => {
            const items = request.docs.map((item) => item.data())

            return items as Array<DeviceInterface>
        },
        staleTime: 1000 * 60 * 25,
    })
}
