<template>
    <q-card flat>
        <div v-if="isSuccess">
            <q-list>
                <q-item-label header>{{ t("title") }}</q-item-label>
                <q-item v-for="item in data" :key="item.name">
                    <q-item-section avatar>
                        <q-icon
                            color="secondary"
                            :name="osToIcon[item.os.toLocaleLowerCase() as keyof typeof osToIcon] ?? 'mdi-penguin'"
                        />
                    </q-item-section>

                    <q-item-section>
                        <q-item-label>{{ item.name }}</q-item-label>
                        <q-item-label caption>{{ item.os }}</q-item-label>
                    </q-item-section>
                </q-item>
            </q-list>
        </div>
        <div v-if="isLoading">
            <q-circular-progress indeterminate rounded size="50px" color="secondary" class="q-ma-md" />
        </div>
    </q-card>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n"
import { useDeviceList } from "@/composables/queries/useDeviceList"

const { data, isSuccess, isLoading } = useDeviceList()

const { t } = useI18n()

const osToIcon = {
    windows: "mdi-microsoft-windows",
    android: "mdi-android",
    ios: "mdi-ios",
}
</script>

<i18n>
    "en": {
        "title": "Devices List",
    }

    "pl": {
        "title": "Lista Urządzeń ",
    }
</i18n>
