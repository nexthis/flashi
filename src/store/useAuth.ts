import { getAuth, User } from "firebase/auth"
import { defineStore } from "pinia"
import { ref, computed } from "vue"

//TODO: Remove
export const useAuth = defineStore("auth", () => {
    //state
    const auth = getAuth()
    const isInitializeState = ref(false)
    const isAuthState = ref(false)
    const userState = ref<User | null>(null)

    //lisiners
    auth.onAuthStateChanged((data) => {
        isInitializeState.value = true

        if (data) {
            userState.value = data
            isAuthState.value = true
            return
        }
        userState.value = null
        isAuthState.value = true
    })

    //getter
    const isInitialize = computed(() => isInitializeState.value)
    const isAuth = computed(() => isAuthState.value)
    const user = computed(() => userState.value)

    return {
        isAuth,
        isInitialize,
        user,
    }
})
