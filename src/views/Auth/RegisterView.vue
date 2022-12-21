<template>
    <q-page padding class="flex bg-image">
        <q-card class="card">
            <div class="flex flex-col justify-center header-image">
                <div class="q-py-xl">
                    <div class="text-h3 text-center text-bold z-top">{{ t("register") }}</div>
                    <div class="flex justify-center q-mt-lg"></div>
                </div>
            </div>
            <q-card-section class="card-body">
                <q-input :label="t('name')" v-model="email" />
                <q-input :label="t('email')" v-model="email" />
                <q-input :label="t('password')" v-model="password" />
                <q-input :label="t('repeat')" v-model="passwordRepeat" />

                <div class="flex q-mt-lg">
                    <q-btn color="negative" :to="{ name: 'login' }">{{ t("back") }}</q-btn>
                    <q-space />
                    <q-btn @click="onRegister" color="positive">{{ t("create") }}</q-btn>
                </div>
            </q-card-section>
        </q-card>
    </q-page>
</template>

<script lang="ts" setup>
import { getAuth, createUserWithEmailAndPassword } from "firebase/auth"
import { ref } from "vue"
import { useI18n } from "vue-i18n"

const auth = getAuth()
const email = ref("")
const password = ref("")
const passwordRepeat = ref("")
const { t } = useI18n()

const onRegister = async () => {
    await createUserWithEmailAndPassword(auth, email.value, password.value)
}
</script>

<style lang="scss" scoped>
.header-image {
    background-image: url(@/assets/images/login.svg);
    background-repeat: no-repeat;
    background-size: cover;
    background-position: center bottom;
}
.card {
    max-width: 450px;
    width: 100%;
    margin: auto;
}

.card-body {
    padding: 20px 30px;
}
</style>

<i18n>
    "en": {
        "title": "Welcome back!",
        "create": "Create",
        "back": "Back",
        "name": "Name",
        "email": "Email",
        "password": "Password",
        "repeat": "Repeat password",
        "register": "Register",
    }

    "pl": {
        "title": "Witaj ponownie!",
        "name": "Nazwa",
        "email": "Email",
        "password": "Hasło",
        "repeat": "Powtórz Hasło",
        "create": "Utwórz",
        "back": "Wstecz",
        "register": "Rejestracja",
    }
</i18n>
