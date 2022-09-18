<template>
    <Suspense>
        <primary-layout>
            <router-view></router-view>
        </primary-layout>
    </Suspense>
</template>

<script setup lang="ts">
import PrimaryLayout from "@/layouts/PrimaryLayout.vue"
</script>

<style scoped lang="scss"></style>
