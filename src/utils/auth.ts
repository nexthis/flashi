import { getAuth, User } from "firebase/auth"

export async function checkAuth(): Promise<User | false> {
    const auth = getAuth()
    return await new Promise((resolve, reject) => {
        const unsubscribe = auth.onAuthStateChanged(
            (user) => {
                unsubscribe()
                resolve(user ?? false)
            },
            () => reject(false)
        )
    })
}

export async function getUserOrThrow(): Promise<User> {
    const auth = getAuth()
    if (!auth.currentUser) {
        throw Error("User not define!")
    }

    return auth.currentUser
}
