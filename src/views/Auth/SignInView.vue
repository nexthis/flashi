<template>
    <q-page padding class="flex">
        <q-card class="card">
            <div class="image-box">
                <img src="@/assets/images/login.jpg" class="image" alt="bg" />
            </div>
            <div class="image-spacer flex flex-col justify-center">
                <div class="text-h3 text-center text-bold z-top">{{ t("title") }}</div>
                <div class="flex justify-center q-mt-lg">
                    <q-btn round size="lg" color="red" icon="mdi-google" />
                    <q-btn round size="lg" class="q-ml-lg" color="blue" icon="mdi-facebook" />
                    <q-btn round size="lg" class="q-ml-lg" color="dark" icon="mdi-github" />
                </div>
            </div>

            <q-card-section>
                <q-input outlined :label="t('email')" v-model="email" />
                <q-input outlined class="q-mt-lg" :label="t('password')" type="password" v-model="password" />
                <div class="q-mt-sm flex justify-between items-center">
                    <q-checkbox class="checkbox text-capitalize" v-model="remember">
                        {{ t("remember") }}
                    </q-checkbox>
                    <div>
                        <router-link :to="{ name: 'registry' }" class="text-primary forgot text-capitalize">
                            {{ t("register") }}
                        </router-link>
                        |
                        <a class="text-primary forgot text-capitalize">{{ t("forgot") }}</a>
                    </div>
                </div>

                <div class="q-mt-lg flex justify-end">
                    <q-btn @click="onSignIn" size="lg" color="primary">{{ t("login") }}</q-btn>
                </div>
            </q-card-section>
        </q-card>
    </q-page>
</template>

<script setup lang="ts">
import { ref } from "vue"
import {
    getAuth,
    signInWithEmailAndPassword,
    setPersistence,
    inMemoryPersistence,
    AuthError,
} from "firebase/auth"
import { useRouter } from "vue-router"
import { useQuasar } from "quasar"
import { useI18n } from "vue-i18n"

const email = ref<string>("")
const remember = ref<boolean>(true)
const password = ref<string>("")

const { t } = useI18n()
const auth = getAuth()
const router = useRouter()
const quasar = useQuasar()

const onSignIn = async () => {
    if (!remember.value) {
        await setPersistence(auth, inMemoryPersistence)
    }

    try {
        await signInWithEmailAndPassword(auth, email.value, password.value)
    } catch (error) {
        const errorTyped = error as AuthError
        quasar.notify({
            message: t("error." + errorTyped.code),
            // caption: errorTyped.code,
            color: "negative",
            position: "bottom-right",
        })
    }

    await router.push({ name: "dashboard" })
}
</script>

<style scoped lang="scss">
a {
    text-decoration: none;
}
.card {
    max-width: 450px;
    width: 100%;
    margin: auto;
    padding: 20px 30px;
}

.flex-col {
    flex-direction: column;
}

.checkbox {
    margin-left: -5px;
}

.image-box {
    position: absolute;
    left: 0;
    top: 0;
    z-index: 0;
}

.image {
    clip-path: ellipse(80% 45% at 36% 8%);
    max-width: 100%;
    height: auto;
}

.image-spacer {
    min-height: 230px;
    height: 100%;
    z-index: 10;
    margin-bottom: 10px;

    @media (max-height: 700px) {
        min-height: 190px;
    }
}

.forgot {
    position: relative;
    cursor: pointer;

    &::after {
        content: "";
        position: absolute;
        bottom: 0;
        left: 0;
        width: 100%;
        height: 0.1em;
        background-color: $primary;
        opacity: 0;
        transition: opacity 300ms, transform 300ms;
    }

    &:hover::after,
    &:focus::after {
        opacity: 1;
        transform: translate3d(0, 0.2em, 0);
        will-change: transform;
    }
}
</style>

<i18n>
    "en": {
        "title": "Welcome back!",
        "email": "Email",
        "password": "Password",
        "login": "login",
        "back": "back",
        "remember": "remember me",
        "forgot": "forgot password",
        "register": "register",
        "error": {
            "auth/user-not-found": "Email or password not exist",
            "auth/missing-email": "Email or password not exist",
            "auth/internal-error": "Error when trying to log in",
            "auth/wrong-password": "Email or password not exist",
        },
    }

    "pl": {
        "title": "Witaj ponownie!",
        "email": "Email",
        "password": "Hasło",
        "login": "zaloguj",
        "back": "wstecz",
        "remember": "zapamiętaj mnie",
        "forgot": "zapomniałem hasła",
        "register": "rejestracja",
        "error": {
            "auth/user-not-found": "Nieprawidłowy email lub hasło",
            "auth/missing-email": "Nieprawidłowy email lub hasło",
            "auth/internal-error": "Błąd przy próbie logowania",
            "auth/wrong-password": "Nieprawidłowy email lub hasło",
        },
    }
</i18n>
