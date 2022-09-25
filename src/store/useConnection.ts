import { defineStore } from "pinia"
import { ref, computed } from "vue"

export const useConnection = defineStore("connection", () => {
    const isConnectedState = ref(false)

    const isConnected = computed(() => isConnectedState.value)

    function connect() {
        isConnectedState.value = true
    }

    function disconnect() {
        isConnectedState.value = false
    }

    return {
        isConnected,
        connect,
        disconnect,
    }
})
