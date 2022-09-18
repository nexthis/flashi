<template>
    <q-page class="container">
        <div class="menu">
            <q-btn class="item" size="xl" round>
                <q-avatar>U</q-avatar>
            </q-btn>

            <div class="menu-content">
                <q-btn :to="{ name: 'home' }" class="item" size="lg" round>
                    <q-icon color="white" name="mdi-view-dashboard-outline" />
                </q-btn>
                <q-btn class="item" size="lg" round>
                    <q-icon name="mdi-text-box-multiple-outline" />
                </q-btn>
                <q-btn class="item" size="lg" round>
                    <q-icon name="mdi-tune" />
                </q-btn>
            </div>
        </div>
        <div class="content">
            <slot />
        </div>
    </q-page>
</template>

<script lang="ts" setup></script>

<style scoped lang="scss">
.menu {
    height: inherit;
    padding: 10px 10px 10px 10px;
    background-color: $dark;
    display: flex;
    flex-direction: column;
    justify-content: left;
    align-items: baseline;
}

.menu-content {
    display: flex;
    flex-direction: column;
    justify-items: center;
    align-items: center;
    justify-content: center;
    gap: 15px;
    height: 100%;
    width: 100%;
}
.item {
    background-color: lighten($color: $dark, $amount: 20);
    & .router-link-active {
        background-color: $primary;
    }
}
.container {
    display: flex;
}
.content {
    height: inherit;
    width: 100%;
    padding: 15px 15px 15px 15px;
}
</style>
