<template>
    <q-page class="container">
        <div class="menu">
            <q-btn :to="{ name: 'account' }" class="item" size="xl" round>
                <q-avatar>{{ "T" }}</q-avatar>
            </q-btn>

            <div class="menu-content">
                <router-link :to="{ name: 'dashboard' }" v-slot="{ isActive }" class="link-clear">
                    <q-btn class="item" :class="{ active: isActive }" size="lg" round>
                        <q-icon color="white" name="mdi-view-dashboard-outline" />
                    </q-btn>
                </router-link>
                <router-link :to="{ name: 'macro' }" v-slot="{ isActive }" class="link-clear">
                    <q-btn class="item" size="lg" :class="{ active: isActive }" round>
                        <q-icon color="white" name="mdi-text-box-multiple-outline" />
                    </q-btn>
                </router-link>
                <router-link :to="{ name: 'setting' }" v-slot="{ isActive }" class="link-clear">
                    <q-btn class="item" size="lg" :class="{ active: isActive }" round>
                        <q-icon color="white" name="mdi-tune" />
                    </q-btn>
                </router-link>
            </div>
        </div>
        <div ref="content" class="content aside-content scrollbar scroll-y">
            <router-view />
        </div>
    </q-page>
</template>

<script lang="ts" setup>
// import { ref, onMounted, onUnmounted } from "vue"
// import { dom, debounce } from "quasar"

// const content = ref<HTMLDivElement>()

// onMounted(() => {
//     setContentHeight()
//     window.addEventListener("resize", onResize)
// })

// onUnmounted(() => {
//     window.removeEventListener("resize", onResize)
// })

// const setContentHeight = () => {
//     if (!content.value) {
//         return
//     }

//     content.value.style.height = `${dom.height(content.value)}px`
//     console.log("in")
// }

// const onResize = debounce(() => {
//     console.log("Test")

//     setContentHeight()
// }, 500)
</script>

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
    &.active {
        background-color: $primary !important;
    }
}
.link-clear {
    text-decoration: unset;
}
.container {
    display: flex;
}
.content {
    height: $content-size;
    width: 100%;
    padding: 15px 15px 15px 15px;
}

//when white mode
.body--light {
    .item {
        background-color: lighten($color: $dark, $amount: 50);
    }
    .menu {
        background-color: darken($color: white, $amount: 20);
    }
}
</style>
