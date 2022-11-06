<template>
    <q-btn :to="{ name: 'login' }">back</q-btn>

    <q-input label="email" v-model="email" />
    <q-input label="password" v-model="password" />
    <q-input label="repeat password" v-model="passwordRepeat" />
    <q-btn @click="onRegister" color="primary">Create</q-btn>
</template>

<script lang="ts" setup>
import { getAuth, createUserWithEmailAndPassword } from "firebase/auth"
import { ref } from "vue"

const auth = getAuth()
const email = ref("")
const password = ref("")
const passwordRepeat = ref("")

const onRegister = async () => {
    await createUserWithEmailAndPassword(auth, email.value, password.value)
}
</script>

<i18n>
    "en": {
        "title": "Welcome back!",
    }

    "pl": {
        "title": "Witaj ponownie!",
    }
</i18n>
