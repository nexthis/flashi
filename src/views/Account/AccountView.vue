<template>
    <div class="text-h2">{{ t("title") }}</div>

    <q-btn @click="onLogout" color="primary">{{ t("logout") }}</q-btn>
</template>

<script setup lang="ts">
import { getAuth } from "firebase/auth"
import { useI18n } from "vue-i18n"
import { useRouter } from "vue-router"

const router = useRouter()
const { t } = useI18n()

const onLogout = async () => {
    const auth = getAuth()
    await auth.signOut()
    router.push({ name: "auth" })
}
</script>

<i18n>
    en: {
        title: "Account",
        logout: "Logout",
    }
    pl: {
        title: "Konto",
        logout: "Wyloguj",
    }
</i18n>
