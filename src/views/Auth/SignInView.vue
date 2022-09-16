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
                    <div class="text-primary forgot text-capitalize">{{ t("forgot") }}</div>
                </div>

                <div class="q-mt-lg flex justify-between">
                    <q-btn :to="{ name: 'home' }" size="lg" color="negative">{{ t("back") }}</q-btn>
                    <q-btn @click="onSignIn" size="lg" color="primary">{{ t("login") }}</q-btn>
                </div>
            </q-card-section>
        </q-card>
    </q-page>
</template>

<script setup lang="ts">
import { ref } from "vue"
import { getAuth, signInWithEmailAndPassword } from "firebase/auth"
import { useRouter } from "vue-router"
import { useI18n } from "vue-i18n"

const { t } = useI18n()
const auth = getAuth()
const router = useRouter()
const email = ref<string>("")
const remember = ref<boolean>(false)
const password = ref<string>("")

const onSignIn = async () => {
    await signInWithEmailAndPassword(auth, email.value, password.value)
    await router.push({ name: "home" })
}
</script>

<style scoped lang="scss">
.card {
    max-width: 550px;
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
    clip-path: ellipse(80% 42% at 36% 8%);
    max-width: 100%;
    height: auto;
}
.image-spacer {
    min-height: 230px;
    height: 100%;
    z-index: 10;
    margin-bottom: 30px;
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
    }

    "pl": {
        "title": "Witaj ponownie!",
        "email": "Email",
        "password": "Hasło",
        "login": "zaloguj",
        "back": "wstecz",
        "remember": "zapamiętaj mnie",
        "forgot": "zapomniałem hasła",
    }
</i18n>
