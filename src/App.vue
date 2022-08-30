<template>
    <Suspense>
        <PrimaryLayouts>
            <router-view></router-view>
        </PrimaryLayouts>
    </Suspense>
</template>

<script setup lang="ts">
import PrimaryLayouts from "@/layouts/PrimaryLayouts.vue"
</script>

<style scoped lang="scss"></style>
