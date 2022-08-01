// store
import {writable} from 'svelte/store'
export const use_proxy:SvelteStore<boolean> = writable(false)
export const low_ms:SvelteStore<number> = writable(4000)
export const high_ms:SvelteStore<number> = writable(15000)