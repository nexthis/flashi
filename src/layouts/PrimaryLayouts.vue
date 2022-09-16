<template>
    <q-layout>
        <q-header>
            <!-- this is where the Pages are injected -->
            <div data-tauri-drag-region class="titlebar">
                <div class="titlebar-button" @click="onMinimize">
                    <q-icon color="grey-4" name="mdi-window-minimize" />
                </div>
                <div class="titlebar-button" @click="onMaximize">
                    <q-icon color="grey-4" name="mdi-window-maximize" />
                </div>
                <div class="titlebar-button" @click="onClose">
                    <q-icon color="grey-4" name="mdi-window-close" />
                </div>
            </div>
        </q-header>

        <q-page-container>
            <slot />
        </q-page-container>
    </q-layout>
</template>

<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window"

const onClose = () => {
    appWindow.hide()
}

const onMinimize = () => {
    appWindow.minimize()
}

const onMaximize = () => {
    appWindow.toggleMaximize()
}
</script>

<style scoped lang="scss">
.titlebar {
    height: 30px;
    background: $primary;
    user-select: none;
    display: flex;
    -webkit-touch-callout: none; /* iOS Safari */
    -webkit-user-select: none; /* Safari */
    -khtml-user-select: none; /* Konqueror HTML */
    -moz-user-select: none; /* Old versions of Firefox */
    -ms-user-select: none;
    justify-content: flex-end;
}

.titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 30px;
}

.titlebar-button:hover {
    background: darken($primary, 5%);
}
</style>

<style lang="scss">
//TODO change
.q-page {
    border: $primary 2px solid;
    border-top: none;
}
</style>
