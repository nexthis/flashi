<template>
    <q-page padding>
        <q-input label="email" v-model="email" />
        <q-input label="password" type="password" v-model="password" />
        <q-btn :to="{ name: 'home' }" color="negative">Back</q-btn>
        <q-btn @click="onSignIn" color="accent">Login</q-btn>
    </q-page>
</template>

<script setup lang="ts">
import { ref } from "vue"
import { getAuth, signInWithEmailAndPassword } from "firebase/auth"
import { useRouter } from "vue-router"

const auth = getAuth()
const router = useRouter()
const email = ref<string>("")
const password = ref<string>("")

const onSignIn = async () => {
    await signInWithEmailAndPassword(auth, email.value, password.value)
    await router.push({ name: "home" })
}
</script>

<style scoped></style>
