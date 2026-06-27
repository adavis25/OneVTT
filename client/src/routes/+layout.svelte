<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { onMount } from 'svelte';
	import { getTheme } from '$lib/state/theme.svelte';

	let { children } = $props();

	onMount(() => getTheme().init());
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
{@render children()}
