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
