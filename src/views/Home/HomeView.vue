<template>
    <q-page class="fit">
        <div>Hej! {{ user?.email }}</div>
        <q-btn :to="{ name: 'auth' }" color="accent">Login</q-btn>
    </q-page>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api"
import { getFirestore, onSnapshot, query, setDoc, doc, collection } from "firebase/firestore"
import { getAuth, User } from "firebase/auth"

const auth = getAuth()
const db = getFirestore()

const user = (await new Promise((resolve) =>
    auth.onAuthStateChanged(
        (user) => resolve(user),

        (e) => resolve(null)
    )
)) as User | null

if (user) {
    const q = query(collection(db, "users", user.uid, "connection_pool"))

    onSnapshot(q, async (snapshot) => {
        snapshot.docChanges().forEach(async (type) => {
            if (type.type === "modified") {
                const offer = type.doc.data()

                const answer = await invoke("connect", {
                    payload: JSON.stringify(offer),
                })
                console.log(answer)
                await setDoc(doc(db, "users", auth.currentUser!.uid, "connection_pool", "answer"), answer)
            }
        })
    })
}
</script>

<style scoped></style>
