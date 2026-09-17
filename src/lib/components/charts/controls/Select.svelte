<script lang="ts">
	import type { Snippet } from 'svelte';

	/**
	 * Drawer control kit — styled select. Pass options via the children snippet
	 * (option/optgroup markup stays plain HTML).
	 */
	let {
		value = $bindable(''),
		small = false,
		onchange,
		children
	}: {
		value?: string;
		/** compact variant for inline rows (grain/format pickers) */
		small?: boolean;
		onchange?: (v: string) => void;
		children: Snippet;
	} = $props();

	function change(e: Event) {
		onchange?.((e.target as HTMLSelectElement).value);
	}
</script>

<select class="ctl-select" class:ctl-small={small} bind:value onchange={change}>
	{@render children()}
</select>
