import { useAuth } from "@/store/useAuth"
import { useStorage } from "@vueuse/core"
import { getFirestore, collection, query, getDocs } from "firebase/firestore"
import { KEYS } from "@/constants/storage"
import { ref } from "vue"
import _ from "lodash"

export function useMacroList() {
    const auth = useAuth()
    const db = getFirestore()

    const items = useStorage<Array<UserMacro>>(KEYS.macros, [])

    const isLoading = ref(true)

    async function refresh() {
        isLoading.value = true
        if (!_.isEmpty(items.value)) {
            isLoading.value = false
            return
        }

        const q = query(collection(db, "users", auth!.user!.uid, "macros"))
        const data = await getDocs(q)

        items.value = data.docs.map((item) => ({ uuid: item.id, ...item.data() })) as Array<UserMacro>
        isLoading.value = false
    }

    refresh()

    return {
        items,
        isLoading,
        refresh,
    }
}
