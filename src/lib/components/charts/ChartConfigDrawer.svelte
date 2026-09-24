<script lang="ts">
	import type { Snippet } from 'svelte';
	import Drawer from '../Drawer.svelte';

	let {
		open = $bindable(false),
		title = 'Chart configuration',
		width = '33vw',
		overlay = true,
		/** position within .app-body (below header/breadcrumb, above the tab bar)
		    instead of viewport-fixed — requires a positioned ancestor spanning
		    exactly that area; off for usages nested inside positioned cards */
		contained = false,
		onClosed,
		children
	}: {
		open?: boolean;
		title?: string;
		/** drawer width, any CSS length */
		width?: string;
		/** dim + click-away overlay; off in the focused config view (left chart stays live) */
		overlay?: boolean;
		contained?: boolean;
		/** config fields */
		children: Snippet;
		/** fired on close (overlay click / X) — for non-bound usage */
		onClosed?: () => void;
	} = $props();
</script>

<Drawer bind:open {title} {width} {overlay} {contained} {onClosed}>
	{@render children()}
</Drawer>
