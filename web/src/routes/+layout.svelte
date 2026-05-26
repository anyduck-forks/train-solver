<script lang="ts">
	import favicon from "$lib/assets/favicon.svg";
	import RemixLanding from "$lib/landing/RemixLanding.svelte";
	import { page } from "$app/stores";

	let { children } = $props();

	const routes = [
		"/",
		"/advanced",
		"/log",
	];

	function getIndex(pathname: string): number {
		const idx = routes.indexOf(pathname);
		return idx >= 0 ? idx : 0;
	}


	import { onNavigate } from '$app/navigation';

onNavigate((navigation) => {
	if (!document.startViewTransition) return;

	return new Promise((resolve) => {
		document.startViewTransition(async () => {
			resolve();
			await navigation.complete;
		});
	});
});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<RemixLanding presetIndex={getIndex($page.url.pathname)} />

<nav class="page-nav">
	{#each routes as route, index}
		<a class:active={route === $page.url.pathname} href={route}>
			Page {index + 1}
		</a>
	{/each}
</nav>

<div class="page-shell">
	{@render children()}
</div>

<style>
	.page-shell {
		position: relative;
		z-index: 2;
		min-height: 100vh;
		view-transition-name: shell;
	}

	.page-nav {
		position: fixed;
		top: 24px;
		right: 24px;
		z-index: 3;
		display: flex;
		gap: 12px;
		padding: 10px 14px;
		background: rgba(8, 12, 20, 0.7);
		border-radius: 999px;
		backdrop-filter: blur(12px);
	}

	.page-nav a {
		font-size: 12px;
		letter-spacing: 0.16em;
		text-transform: uppercase;
		color: rgba(220, 232, 245, 0.7);
		text-decoration: none;
	}

	.page-nav a.active {
		color: #ffffff;
	}

	@media (max-width: 720px) {
		.page-nav {
			left: 16px;
			right: 16px;
			justify-content: space-between;
		}
	}


	@keyframes fade-in {
	from {
		opacity: 0;
	}
}

@keyframes fade-out {
	to {
		opacity: 0;
	}
}

@keyframes slide-from-top {
	from {
		transform: translateY(-30px);
	}
}

@keyframes slide-to-bottom {
	to {
		transform: translateY(30px);
	}
}

::view-transition-old(shell) {
	animation:
		90ms cubic-bezier(0.4, 0, 1, 1) both fade-out,
		300ms cubic-bezier(0.4, 0, 0.2, 1) both slide-to-bottom;
}

::view-transition-new(shell) {
	animation:
		210ms cubic-bezier(0, 0, 0.2, 1) 90ms both fade-in,
		300ms cubic-bezier(0.4, 0, 0.2, 1) both slide-from-top;
}
</style>
