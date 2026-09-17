<script lang="ts">
	/** Shared Settings | Danger zone tab bar for every config drawer. */
	let {
		active,
		onchange
	}: {
		active: 'settings' | 'danger';
		onchange: (tab: 'settings' | 'danger') => void;
	} = $props();
</script>

<div class="flex gap-1 bg-zinc-100 rounded-lg p-1">
	<button
		class="flex-1 px-3 py-1.5 rounded-md text-sm {active === 'settings'
			? 'bg-white shadow-sm font-medium text-zinc-900'
			: 'text-zinc-500'}"
		onclick={() => onchange('settings')}
	>
		Settings
	</button>
	<button
		class="flex-1 px-3 py-1.5 rounded-md text-sm {active === 'danger'
			? 'bg-white shadow-sm font-medium text-red-600'
			: 'text-zinc-500 hover:text-red-500'}"
		onclick={() => onchange('danger')}
	>
		Danger zone
	</button>
</div>
