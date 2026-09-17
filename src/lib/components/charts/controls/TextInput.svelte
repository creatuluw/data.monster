<script lang="ts">
	import './controls.css';

	/** Drawer control kit — text input (mono variant for expressions/SQL). */
	let {
		value = $bindable(''),
		placeholder = '',
		mono = false,
		disabled = false,
		oncommit
	}: {
		value?: string;
		placeholder?: string;
		/** Geist Mono — for expressions, SQL, code-like values */
		mono?: boolean;
		disabled?: boolean;
		/** fired on change (commit), with the new value */
		oncommit?: (v: string) => void;
	} = $props();

	function commit(e: Event) {
		oncommit?.((e.target as HTMLInputElement).value);
	}
</script>

<input
	type="text"
	class="ctl-input"
	class:ctl-mono={mono}
	{placeholder}
	{disabled}
	bind:value
	onchange={commit}
/>
