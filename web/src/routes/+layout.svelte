<script lang="ts">
	import favicon from "$lib/assets/favicon.svg";
	import RemixLanding from "$lib/vendor/RemixLanding.svelte";
	import { page } from "$app/stores";
	import { base } from '$app/paths';


	let { children } = $props();

	const routes = [
		`${base}/`,
		`${base}/advanced`,
		`${base}/log`,
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
