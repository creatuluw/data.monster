<script lang="ts">
	/**
	 * /components visual demos: renders a live, standalone demo for every component
	 * that can run without app data/tauri. ds/ components self-showcase (same as /ui);
	 * the rest get hand-written dummy-data demos; genuinely app-wired components show
	 * the honest note instead.
	 */
	import Accordion from './ds/Accordion.svelte';
	import Buttons from './ds/Buttons.svelte';
	import Cards from './ds/Cards.svelte';
	import ColorPalette from './ds/ColorPalette.svelte';
	import Footer from './ds/Footer.svelte';
	import Hero from './ds/Hero.svelte';
	import Icons from './ds/Icons.svelte';
	import Inputs from './ds/Inputs.svelte';
	import Modal from './ds/Modal.svelte';
	import Motion from './ds/Motion.svelte';
	import Nav from './ds/Nav.svelte';
	import Principles from './ds/Principles.svelte';
	import SearchAhead from './ds/SearchAhead.svelte';
	import Spacing from './ds/Spacing.svelte';
	import Tags from './ds/Tags.svelte';
	import Toast from './ds/Toast.svelte';
	import Toggles from './ds/Toggles.svelte';
	import Typography from './ds/Typography.svelte';

	import Btn from './charts/controls/Btn.svelte';
	import Field from './charts/controls/Field.svelte';
	import Section from './charts/controls/Section.svelte';
	import Select from './charts/controls/Select.svelte';
	import TextInput from './charts/controls/TextInput.svelte';
	import NumberInput from './charts/controls/NumberInput.svelte';
	import Toggle from './charts/controls/Toggle.svelte';
	import RemoveBtn from './charts/controls/RemoveBtn.svelte';
	import DangerZone from './charts/controls/DangerZone.svelte';
	import BarChart from './charts/BarChart.svelte';

	import Tag from './Tag.svelte';
	import Tabs from './Tabs.svelte';
	import Tooltip from './Tooltip.svelte';
	import Pagination from './Pagination.svelte';
	import TagInput from './TagInput.svelte';
	import TableList from './TableList.svelte';
	import Badge from './Badge.svelte';
	import Breadcrumb from './Breadcrumb.svelte';
	import Drawer from './Drawer.svelte';
	import LabsPlaceholder from './LabsPlaceholder.svelte';
	import PreviewPane from './PreviewPane.svelte';

	const DS_SHOWCASE: Record<string, any> = {
		Accordion, Buttons, Cards, ColorPalette, Footer, Hero, Icons, Inputs, Modal,
		Motion, Nav, Principles, SearchAhead, Spacing, Tags, Toast, Toggles, Typography
	};

	let { name }: { name: string } = $props();

	const noop = () => {};
	let tabsActive = $state('preview');
	let tagValue = $state<string[]>(['sales', 'eu']);
	let pageNum = $state(2);
	let selectVal = $state('eu');
	let textVal = $state('');
	let numVal = $state<number | undefined>(42);
	let toggleVal = $state(true);
	let drawerOpen = $state(false);
	let drawerToggle = $state(true);
</script>

{#if DS_SHOWCASE[name]}
	{@const Demo = DS_SHOWCASE[name]}
	<Demo />
{:else if name === 'Btn'}
	<div class="demo-row">
		<Btn variant="primary" onclick={noop}>Primary</Btn>
		<Btn variant="secondary" onclick={noop}>Secondary</Btn>
		<Btn variant="ghost" onclick={noop}>Ghost</Btn>
		<Btn variant="danger" onclick={noop}>Danger</Btn>
		<Btn variant="primary" disabled onclick={noop}>Disabled</Btn>
	</div>
{:else if name === 'Field'}
	<div class="demo-col">
		<Field label="Measure" hint="DuckDB expression">
			<TextInput bind:value={textVal} placeholder="sum(amount)" mono />
		</Field>
		<Field label="Aggregation" inline>
			<Select bind:value={selectVal}>
				<option value="sum">sum</option>
				<option value="avg">avg</option>
				<option value="count">count</option>
			</Select>
		</Field>
		<Field label="Rounding" inline>
			<NumberInput bind:value={numVal} placeholder="2" />
		</Field>
	</div>
{:else if name === 'Section'}
	<div class="demo-col">
		<Section title="Page settings">
			<p class="demo-note">Section groups drawer settings under a collapsible title.</p>
		</Section>
	</div>
{:else if name === 'Select'}
	<div class="demo-row">
		<Select bind:value={selectVal}>
			<option value="eu">Europe</option>
			<option value="us">Americas</option>
			<option value="apac">APAC</option>
		</Select>
		<Select bind:value={selectVal} small>
			<option value="eu">Small</option>
			<option value="us">Small select</option>
		</Select>
	</div>
{:else if name === 'TextInput'}
	<div class="demo-col">
		<TextInput bind:value={textVal} placeholder="Type and commit (enter)" />
		<TextInput bind:value={textVal} placeholder="mono" mono />
		<TextInput value="locked" disabled />
	</div>
{:else if name === 'NumberInput'}
	<div class="demo-col">
		<NumberInput bind:value={numVal} placeholder="min 0, max 100" min={0} max={100} />
		<NumberInput value={undefined} placeholder="empty" />
	</div>
{:else if name === 'Toggle'}
	<div class="demo-col">
		<Toggle bind:checked={toggleVal} label="Show legend" />
		<Toggle bind:checked={toggleVal} label="Same state (shared bind)" />
	</div>
{:else if name === 'RemoveBtn'}
	<div class="demo-row">
		<RemoveBtn onclick={noop} />
		<RemoveBtn onclick={noop} title="Delete page" />
		<RemoveBtn onclick={noop} disabled />
	</div>
{:else if name === 'DangerZone'}
	<div class="demo-col">
		<DangerZone
			heading="Delete this page"
			description="The page file is removed from dm/pages/. This cannot be undone."
			confirmLabel="Delete page"
			onconfirm={noop}
		/>
	</div>
{:else if name === 'Tag'}
	<div class="demo-row">
		<Tag>default</Tag>
		<Tag variant="accent">accent</Tag>
		<Tag variant="success">success</Tag>
	</div>
{:else if name === 'Tabs'}
	{#snippet prevLabel()}Preview{/snippet}
	{#snippet schemaLabel()}Schema{/snippet}
	{#snippet codeLabel()}Code{/snippet}
	<div class="demo-col">
		<Tabs
			items={[
				{ key: 'preview', label: prevLabel },
				{ key: 'schema', label: schemaLabel },
				{ key: 'code', label: codeLabel }
			]}
			bind:activeKey={tabsActive}
			onchange={noop}
		/>
	</div>
{:else if name === 'Tooltip'}
	<div class="demo-row">
		<Tooltip text="Shows on hover">
			<Btn variant="secondary" onclick={noop}>Hover me</Btn>
		</Tooltip>
	</div>
{:else if name === 'Pagination'}
	<div class="demo-col">
		<Pagination currentPage={pageNum} totalItems={87} perPage={20} onchange={(p) => (pageNum = p)} />
	</div>
{:else if name === 'TagInput'}
	<div class="demo-col">
		<TagInput bind:tags={tagValue} suggestions={['marketing', 'finance']} placeholder="Add tag…" onchange={noop} />
	</div>
{:else if name === 'TableList'}
	<div class="demo-col">
		<TableList
			tables={['superstore_orders', 'superstore_returns', 'customers', 'regions']}
			selected="superstore_orders"
			onselect={noop}
			onconnect={noop}
		/>
	</div>
{:else if name === 'BarChart'}
	{#snippet barTip(d: { region: string; sales: number })}<span>{d.region}: {d.sales}</span>{/snippet}
	<div class="chart-stage">
		<BarChart
			data={[
				{ region: 'EU', sales: 340 },
				{ region: 'Americas', sales: 520 },
				{ region: 'APAC', sales: 410 },
				{ region: 'MEA', sales: 180 },
				{ region: 'Nordics', sales: 260 }
			]}
			category={(d) => d.region}
			value={(d) => d.sales}
			tooltip={barTip}
			labelFor={(d) => d.region}
		/>
	</div>
{:else if name === 'Badge'}
	<div class="demo-row">
		<Badge />
		<Badge count={3} />
		<Badge count={12} variant="accent" />
		<Badge count={99} variant="danger" />
	</div>
{:else if name === 'Breadcrumb'}
	<div class="demo-col">
		<Breadcrumb items={[{ href: '/', label: 'Home' }, { href: '/pages', label: 'Pages' }, { href: '/pages/revenue', label: 'Revenue' }]} />
	</div>
{:else if name === 'LabsPlaceholder'}
	<div class="demo-col">
		<LabsPlaceholder title="Sankey diagram" />
	</div>
{:else if name === 'PreviewPane'}
	<div class="demo-col">
		<PreviewPane
			data={{
				columns: ['order_id', 'city', 'sales'],
				rows: [
					{ order_id: 1001, city: 'Berlin', sales: 120 },
					{ order_id: 1002, city: 'Tokyo', sales: 340 },
					{ order_id: 1003, city: 'Oslo', sales: 90 }
				],
				detectedTypes: [
					{ name: 'order_id', type: 'int' },
					{ name: 'city', type: 'str' },
					{ name: 'sales', type: 'float' }
				],
				totalRows: 3,
				sourceName: 'sample.csv'
			}}
			columnOverrides={[]}
			onnext={noop}
		/>
	</div>
{:else if name === 'SearchAhead'}
	<SearchAhead />
{:else if name === 'Drawer'}
	{#snippet drawerBody()}
		<div class="demo-col">
			<p>
				A side panel for detail views, settings, and multi-step flows that
				need more room than a modal.
			</p>
			<Field label="Source name">
				<TextInput value="PostgreSQL Production" mono />
			</Field>
			<Toggle bind:checked={drawerToggle} label="Refresh every 15 min" />
		</div>
	{/snippet}
	{#snippet drawerFooter()}
		<Btn variant="ghost" onclick={() => (drawerOpen = false)}>Close</Btn>
		<Btn onclick={() => (drawerOpen = false)}>Save</Btn>
	{/snippet}
	<div class="demo-row">
		<Btn onclick={() => (drawerOpen = true)}>Open Drawer</Btn>
	</div>
	<Drawer
		bind:open={drawerOpen}
		title="Structured Queries"
		width="420px"
		onClosed={() => (drawerOpen = false)}
	>
		{@render drawerBody()}
		{@render drawerFooter()}
	</Drawer>
{:else}
	<p class="no-live">
		This component needs app data/props, so there is no standalone preview — see it
		working in the app. Components under <code class="font-mono">ds/</code>, the
		<code class="font-mono">charts/controls</code> kit and a few others render live here.
	</p>
{/if}

<style>
	.demo-row {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: var(--space-3);
	}

	.demo-col {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		max-width: 480px;
	}

	.demo-note {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		margin: 0;
	}

	.chart-stage {
		max-width: 720px;
	}

	.no-live {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		max-width: 64ch;
		line-height: var(--leading-relaxed);
	}
</style>
