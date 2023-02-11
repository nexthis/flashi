<template>
    <div class="flex flex-center">
        <div class="text-h2">{{ t("title") }}</div>
        <q-space />
        <q-btn @click="onLogout" color="negative">{{ t("logout") }}</q-btn>
    </div>

    <div class="row q-col-gutter-md q-mt-lg">
        <!-- <div class="col-6">
            <q-card>
                <q-card-section>
                    <div class="flex items-center">
                        <div class="text-h5 q-mt-sm q-mb-xs">Personalizacja</div>
                        <q-space />
                        <q-icon color="light-green" name="mdi-account-circle-outline" size="70px" />
                    </div>

                    <div class="q-mt-md">
                        Przejrzyj dane na swoim koncie i określ, jaka aktywność ma być zapisywana w celu
                        personalizowania usług
                    </div>
                </q-card-section>

                <q-btn flat class="full-width q-mt-sm">Zarządzaj swoimi danymi</q-btn>
            </q-card>
        </div>

        <div class="col-6">
            <q-card>
                <q-card-section>
                    <div class="flex items-center">
                        <div class="text-h5 q-mt-sm q-mb-xs">Bezpieczeństwo</div>
                        <q-space />
                        <q-icon color="cyan" name="mdi-security" size="70px" />
                    </div>

                    <div class="q-mt-md">
                        Przejrzyj dane na swoim koncie i określ, jaka aktywność ma być zapisywana w celu
                        personalizowania usług
                    </div>
                </q-card-section>

                <q-btn flat class="full-width q-mt-sm">bezpieczeństwo i hasła</q-btn>
            </q-card>
        </div> -->
    </div>
</template>

<script setup lang="ts">
import { getAuth } from "firebase/auth"
import { useI18n } from "vue-i18n"
import { useRouter } from "vue-router"

const auth = getAuth()
const router = useRouter()
const { t } = useI18n()

const onLogout = async () => {
    await auth.signOut()
    router.push({ name: "login" })
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
