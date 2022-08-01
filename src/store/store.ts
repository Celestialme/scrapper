// store
import {writable} from 'svelte/store'
export const use_proxy:SvelteStore<boolean> = writable(false)
export const min_ms:SvelteStore<number> = writable(4000)
export const max_ms:SvelteStore<number> = writable(15000)
export const proxy_list:SvelteStore<string> = writable("")
export const dark_theme:SvelteStore<bool> = writable(true)