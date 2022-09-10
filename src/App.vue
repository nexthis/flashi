<template>
    <Suspense>
        <primary-layouts>
            <router-view></router-view>
        </primary-layouts>
    </Suspense>
</template>

<script setup lang="ts">
import PrimaryLayouts from "@/layouts/PrimaryLayouts.vue"
</script>

<style scoped lang="scss"></style>
