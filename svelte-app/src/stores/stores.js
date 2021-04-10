import { readable, writable, derived } from "svelte/store";

export const gameOfLife = writable(null);

export const rustMemBuffer = writable(null);