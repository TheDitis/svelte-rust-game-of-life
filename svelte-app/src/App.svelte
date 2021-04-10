<script lang="ts">
	import {onMount} from "svelte";
	import GameOfLife from "./Components/GameOfLife.svelte";
	import MainWorldControls from "./Components/MainWorldControls.svelte";
	import FrameRateInfo from "./Components/FrameRateInfo.svelte";
	import {gameOfLife, rustMemBuffer} from "./stores/stores";
	import wasm from '../../wasm-game-of-life/Cargo.toml';
	import {memory} from "wasm-game-of-life/rust_game_of_life_tutorial_bg.wasm.d.ts";

	onMount(async () => {
		const wasmPackage = await wasm();
		gameOfLife.set(wasmPackage);
		rustMemBuffer.set(memory.buffer)
	})

</script>

<main>
	{#if $gameOfLife}
		<GameOfLife/>
		<MainWorldControls/>
		<FrameRateInfo/>
		<h1>{$gameOfLife.greet()}!</h1>
		<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
	{/if}
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
