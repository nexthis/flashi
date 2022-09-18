<template>
    <asside-layout>
        <q-btn :to="{ name: 'auth' }" color="accent">Login</q-btn>
    </asside-layout>
</template>

<script setup lang="ts">
import AssideLayout from "@/layouts/AssideLayout.vue"
</script>

<style scoped></style>
