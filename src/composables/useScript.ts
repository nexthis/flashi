import { invoke } from "@tauri-apps/api/tauri"

export function useScript() {
    async function run(script: string) {
        await invoke("compile", { value: script })
    }

    return {
        run,
    }
}
