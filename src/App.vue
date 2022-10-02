<template>
    <Suspense>
        <primary-layout>
            <router-view v-if="auth.isInitialize"></router-view>
        </primary-layout>
    </Suspense>
</template>

<script setup lang="ts">
import PrimaryLayout from "@/layouts/PrimaryLayout.vue"
import { useAuth } from "./store/useAuth"

const auth = useAuth()
</script>

<style scoped lang="scss"></style>
