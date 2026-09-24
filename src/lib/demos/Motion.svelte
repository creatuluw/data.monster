<script lang="ts">
	import { onMount } from 'svelte';

	let motionStages = $state<boolean[]>([]);

	function toggleStage(index: number) {
		motionStages[index] = !motionStages[index];
		motionStages = [...motionStages];
	}

	let cellVisible = $state<boolean[]>(Array(18).fill(false));

	function triggerStagger() {
		cellVisible = Array(18).fill(false);
		cellVisible.forEach((_, i) => {
			setTimeout(() => {
				cellVisible[i] = true;
				cellVisible = [...cellVisible];
			}, i * 50);
		});
	}

	onMount(() => {
		setTimeout(triggerStagger, 800);
	});
</script>

<section class="section" id="motion" aria-labelledby="motion-title">
	<div class="section-header entrance">
		<span class="section-number">05</span>
		<h2 class="section-title" id="motion-title">Motion</h2>
	</div>

	<div class="motion-demos entrance entrance-d1">
		<div class="motion-demo">
			<div class="motion-demo-header">
				<span class="motion-demo-name">Out Expo</span>
				<span class="motion-demo-label">Easing</span>
			</div>
			<div
				class="motion-stage ease-expo"
				class:active={motionStages[0]}
				role="presentation"
				aria-label="Exponential ease-out demo"
				onclick={() => toggleStage(0)}
			>
				<div class="motion-block"></div>
			</div>
			<span class="motion-curve">cubic-bezier(0.16, 1, 0.3, 1)</span>
		</div>
		<div class="motion-demo">
			<div class="motion-demo-header">
				<span class="motion-demo-name">Out Quart</span>
				<span class="motion-demo-label">Easing</span>
			</div>
			<div
				class="motion-stage ease-quart"
				class:active={motionStages[1]}
				role="presentation"
				aria-label="Quartic ease-out demo"
				onclick={() => toggleStage(1)}
			>
				<div class="motion-block"></div>
			</div>
			<span class="motion-curve">cubic-bezier(0.25, 1, 0.5, 1)</span>
		</div>
		<div class="motion-demo">
			<div class="motion-demo-header">
				<span class="motion-demo-name">Industrial</span>
				<span class="motion-demo-label">Easing</span>
			</div>
			<div
				class="motion-stage ease-industrial"
				class:active={motionStages[2]}
				role="presentation"
				aria-label="Industrial easing demo"
				onclick={() => toggleStage(2)}
			>
				<div class="motion-block"></div>
			</div>
			<span class="motion-curve">cubic-bezier(0.22, 0.61, 0.36, 1)</span>
		</div>
	</div>

	<div class="duration-grid entrance entrance-d2">
		<div class="duration-item">
			<span class="duration-token">--duration-fast</span>
			<span class="duration-value">120ms</span>
			<span class="duration-usage">Hover feedback, focus rings, micro state</span>
		</div>
		<div class="duration-item">
			<span class="duration-token">--duration-base</span>
			<span class="duration-value">250ms</span>
			<span class="duration-usage">Toggles, inline transitions, panel swaps</span>
		</div>
		<div class="duration-item">
			<span class="duration-token">--duration-slow</span>
			<span class="duration-value">450ms</span>
			<span class="duration-usage">Page regions, expand/collapse, modals</span>
		</div>
		<div class="duration-item">
			<span class="duration-token">--duration-enter</span>
			<span class="duration-value">600ms</span>
			<span class="duration-usage">Page loads, stagger reveals, hero entrance</span>
		</div>
	</div>

	<div class="stagger-demo entrance entrance-d3">
		<span class="component-group-label">Interaction Demo</span>
		<h3 class="component-group-title">Stagger Reveal</h3>
		<p class="desc">
			Click the grid to trigger a staggered fill. Elements fill
			sequentially, creating rhythm without chaos — like indicators lighting
			up on a control panel.
		</p>
		<div class="stagger-grid" role="button" tabindex="0" aria-label="Click to trigger stagger animation" onclick={triggerStagger} onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); triggerStagger(); } }}>
			{#each Array(18) as _, i}
				<div class="stagger-cell" class:visible={cellVisible[i]}></div>
			{/each}
		</div>
	</div>
</section>

<style>
	.motion-demos {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
		gap: var(--space-6);
	}

	.motion-demo {
		background: var(--color-surface);
		border: 1px solid var(--color-surface-sunken);
		border-radius: var(--radius-md);
		padding: var(--space-8);
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		transition: box-shadow var(--duration-fast) ease, border-color var(--duration-fast) ease;
	}

	.motion-demo:hover {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}

	.motion-demo-header {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
	}

	.motion-demo-label {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.motion-demo-name {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 700;
		color: var(--color-text);
	}

	.motion-stage {
		height: 40px;
		display: flex;
		align-items: center;
		position: relative;
		overflow: hidden;
		background: var(--color-surface-sunken);
		cursor: pointer;
	}

	.motion-block {
		width: 24px;
		height: 24px;
		background: var(--color-accent);
		position: absolute;
		left: 8px;
		transition-property: transform;
	}

	.motion-stage:hover .motion-block,
	.motion-stage.active .motion-block {
		transform: translateX(calc(100% * 5));
	}

	.ease-expo .motion-block { transition-duration: 700ms; transition-timing-function: var(--ease-out-expo); }
	.ease-quart .motion-block { transition-duration: 500ms; transition-timing-function: var(--ease-out-quart); }
	.ease-industrial .motion-block { transition-duration: 600ms; transition-timing-function: var(--ease-industrial); }

	.motion-curve {
		font-family: var(--font-mono);
		font-size: 10px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.02em;
	}

	.duration-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: var(--space-6);
		margin-top: var(--space-8);
	}

	.duration-item {
		background: var(--color-surface);
		border: 1px solid var(--color-surface-sunken);
		border-radius: var(--radius-md);
		padding: var(--space-6);
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		transition: box-shadow var(--duration-fast) ease, border-color var(--duration-fast) ease;
	}

	.duration-item:hover {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}

	.duration-token {
		font-family: var(--font-mono);
		font-size: 10px;
		font-weight: 700;
		letter-spacing: 0.06em;
		color: var(--color-accent);
	}

	.duration-value {
		font-family: var(--font-mono);
		font-size: 10px;
		color: var(--color-text-tertiary);
	}

	.duration-usage {
		font-size: var(--text-sm);
		color: var(--color-text-secondary);
		line-height: var(--leading-normal);
	}

	.stagger-demo {
		margin-top: var(--space-12);
	}

	.component-group-label {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
		margin-bottom: var(--space-6);
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.component-group-label::before {
		content: "\25A0";
		color: var(--color-accent);
		font-size: 7px;
	}

	.component-group-title {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
		margin-bottom: var(--space-6);
	}

	.desc {
		font-size: var(--text-sm);
		color: var(--color-text-secondary);
		margin-bottom: var(--space-4);
		max-width: 48ch;
	}

	.stagger-grid {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: 2px;
		margin-top: var(--space-4);
	}

	.stagger-cell {
		aspect-ratio: 2;
		background: var(--color-surface-sunken);
		position: relative;
		overflow: hidden;
		cursor: pointer;
	}

	.stagger-cell::after {
		content: "";
		position: absolute;
		inset: 0;
		background: var(--color-accent);
		transform: scaleX(0);
		transform-origin: left;
		transition: transform var(--duration-base) var(--ease-out-expo);
	}

	.stagger-cell.visible::after {
		transform: scaleX(1);
	}

	@media (max-width: 640px) {
		.stagger-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}
</style>
