<script lang="ts">
	import './controls.css';

	/** Drawer control kit — number input (tabular figures, undefined-aware). */
	let {
		value = $bindable<number | undefined>(undefined),
		placeholder = '',
		min,
		max,
		step,
		oncommit
	}: {
		value?: number | undefined;
		placeholder?: string;
		min?: number;
		max?: number;
		step?: number;
		/** fired on change; undefined when cleared */
		oncommit?: (v: number | undefined) => void;
	} = $props();

	function commit(e: Event) {
		const raw = (e.target as HTMLInputElement).value;
		value = raw === '' ? undefined : Number(raw);
		oncommit?.(value);
	}
</script>

<input type="number" class="ctl-input" {placeholder} {min} {max} {step} value={value ?? ''} onchange={commit} />
