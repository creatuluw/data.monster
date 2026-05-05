<script lang="ts">
	interface Props {
		for?: string;
		class?: string;
		children?: any;
	}

	let { for: htmlFor, class: className, children }: Props = $props();
</script>

<label
	for={htmlFor}
	class="block text-xs font-medium text-slate-500 dark:text-slate-400 mb-2 uppercase tracking-wide {className || ''}"
>
	{@render children?.()}
</label>

