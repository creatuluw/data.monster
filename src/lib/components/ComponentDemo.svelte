<script lang="ts">
	/**
	 * The demo registry behind the /components detail pages — every entry here
	 * renders the REAL component from src/lib/components with static fixture
	 * data, so each card shows the component's actual design.
	 * Keep in sync with SHOWCASEABLE in $lib/app-components.ts.
	 */
	import { createPageRuntime } from '$lib/charts/page-runtime.svelte';
	import type { PageDoc, ChartBlockSpec } from '$lib/charts/spec-types';
	import type { MasterItem } from '$lib/charts/items';
	import type { TableSchemas } from '$lib/charts/query/compile';
	import Badge from './Badge.svelte';
	import Breadcrumb from './Breadcrumb.svelte';
	import Drawer from './Drawer.svelte';
	import LabsPlaceholder from './LabsPlaceholder.svelte';
	import Pagination from './Pagination.svelte';
	import PreviewPane from './PreviewPane.svelte';
	import TagInput from './TagInput.svelte';
	import Tabs from './Tabs.svelte';
	import TableDrawer from './TableDrawer.svelte';
	import TableOverview from './TableOverview.svelte';
	import TableViewer from './TableViewer.svelte';
	import ColumnFunctionDrawer from './ColumnFunctionDrawer.svelte';
	import ModelManager from './ModelManager.svelte';
	import BlockInspector from './charts/BlockInspector.svelte';
	import ChartCard from './charts/ChartCard.svelte';
	import ChartConfigDrawer from './charts/ChartConfigDrawer.svelte';
	import ExprEditor from './charts/ExprEditor.svelte';
	import ItemEditor from './charts/ItemEditor.svelte';
	import PageGrid from './charts/PageGrid.svelte';
	import RelationshipEditor from './charts/RelationshipEditor.svelte';
	import RolePickerModal from './charts/RolePickerModal.svelte';
	import SkeletonSetup from './charts/SkeletonSetup.svelte';
	import BarChartRenderer from './charts/renderers/BarChartRenderer.svelte';
	import HeatmapRenderer from './charts/renderers/HeatmapRenderer.svelte';
	import TableRenderer from './charts/renderers/TableRenderer.svelte';
	import Btn from './charts/controls/Btn.svelte';
	import DangerZone from './charts/controls/DangerZone.svelte';
	import Field from './charts/controls/Field.svelte';
	import NumberInput from './charts/controls/NumberInput.svelte';
	import RemoveBtn from './charts/controls/RemoveBtn.svelte';
	import Section from './charts/controls/Section.svelte';
	import Select from './charts/controls/Select.svelte';
	import TextInput from './charts/controls/TextInput.svelte';
	import Toggle from './charts/controls/Toggle.svelte';

	// ── static fixtures shared by the chart-family demos ──
	const demoSchemas: TableSchemas = {
		orders: ['order_id', 'city', 'region', 'sales'],
		customers: ['customer_id', 'name', 'segment']
	};
	const demoItems: MasterItem[] = [
		{ id: 'mi-sales', kind: 'measure', table: 'orders', label: 'Total sales', expr: 'sum(sales)' }
	];
	const demoRows: Record<string, unknown>[] = [
		{ city: 'Berlin', region: 'EU', sales: 120 },
		{ city: 'Tokyo', region: 'APAC', sales: 340 },
		{ city: 'Oslo', region: 'EU', sales: 90 }
	];
	const demoChart: ChartBlockSpec = {
		type: 'bar',
		title: 'Sales by city',
		source: { table: 'orders' },
		dimensions: [],
		measures: []
	};
	const demoDoc: PageDoc = {
		slug: 'demo',
		title: 'Demo page',
		rows: [{ columns: [{ span: 12, blocks: [{ type: 'chart', chart: { ...demoChart, dimensions: [{ col: 'city' }], measures: [{ expr: 'sum(sales)' }] } }] }] }]
	};
	const demoRuntime = createPageRuntime(demoDoc, {
		schemas: demoSchemas,
		items: demoItems,
		relationships: [],
		runQuery: async () => demoRows
	});

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
	let tableDrawerOpen = $state(false);
	let colFuncOpen = $state(false);
	let rolePickerOpen = $state(false);
	let configOpen = $state(true);
	let exprVal = $state('sum(sales)');
</script>

{#if name === 'Btn'}
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
{:else if name === 'Pagination'}
	<div class="demo-col">
		<Pagination currentPage={pageNum} totalItems={87} perPage={20} onchange={(p) => (pageNum = p)} />
	</div>
{:else if name === 'TagInput'}
	<div class="demo-col">
		<TagInput bind:tags={tagValue} suggestions={['marketing', 'finance']} placeholder="Add tag…" onchange={noop} />
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
{:else if name === 'TableDrawer'}
	<div class="demo-row">
		<Btn onclick={() => (tableDrawerOpen = true)}>Open Table settings</Btn>
	</div>
	{#if tableDrawerOpen}
		<TableDrawer tableName="orders" onclose={() => (tableDrawerOpen = false)} />
	{/if}
{:else if name === 'ColumnFunctionDrawer'}
	<div class="demo-row">
		<Btn onclick={() => (colFuncOpen = true)}>Open Add function</Btn>
	</div>
	{#if colFuncOpen}
		<ColumnFunctionDrawer
			columnName="sales"
			columnType="float"
			fieldFunctions={[{ id: 'f-fmt', label: 'Format thousands', description: '1.2k style', sql_template: 'ROUND({col}/1000, 1)', applies_to: 'numeric', output_type: 'string' }]}
			activeFunctions={[]}
			onclose={() => (colFuncOpen = false)}
			onapply={noop}
		/>
	{/if}
{:else if name === 'ModelManager'}
	<div class="demo-col">
		<ModelManager />
	</div>
{:else if name === 'TableOverview'}
	<div class="demo-col">
		<TableOverview tables={['orders', 'customers', 'regions']} onselect={noop} onopendrawer={noop} />
	</div>
{:else if name === 'TableViewer'}
	<div class="demo-col">
		<TableViewer
			tableName="orders"
			result={{ columns: ['city', 'sales'], rows: demoRows, totalRows: 3 }}
		/>
	</div>
{:else if name === 'ChartCard'}
	<div class="demo-col">
		<ChartCard title="Sales by city" subtitle="orders · this year">
			<p class="demo-note">Chart content renders as the card's children — bars, heatmaps, tables.</p>
		</ChartCard>
		<ChartCard title="Unconfigured chart" status="setup" />
	</div>
{:else if name === 'ChartConfigDrawer'}
	{#snippet configFields()}
		<Field label="Top-N buckets" inline>
			<NumberInput value={10} min={1} />
		</Field>
		<Toggle bind:checked={toggleVal} label="Sort descending" />
	{/snippet}
	<div class="demo-row">
		<Btn onclick={() => (configOpen = !configOpen)}>{configOpen ? 'Hide' : 'Show'} config drawer</Btn>
	</div>
	<ChartConfigDrawer bind:open={configOpen} title="Bar chart configuration" width="380px">
		{@render configFields()}
	</ChartConfigDrawer>
{:else if name === 'BlockInspector'}
	<div class="demo-col">
		<BlockInspector
			doc={demoDoc}
			ri={0}
			ci={0}
			bi={0}
			schemas={demoSchemas}
			items={demoItems}
			relationships={[]}
		/>
	</div>
{:else if name === 'ItemEditor'}
	<div class="demo-col">
		<ItemEditor
			kind="measure"
			schemas={demoSchemas}
			metas={{ orders: [{ name: 'order_id', type: 'int' }, { name: 'city', type: 'str' }, { name: 'sales', type: 'float' }] }}
		/>
	</div>
{:else if name === 'ExprEditor'}
	<div class="demo-col">
		<ExprEditor
			bind:value={exprVal}
			kind="measure"
			table="orders"
			columns={demoSchemas.orders}
			masterItems={demoItems.filter((i) => i.kind === 'measure')}
			placeholder="e.g. sum(amount)"
			preview={false}
		/>
	</div>
{:else if name === 'PageGrid'}
	<div class="demo-col" style="max-width: none;">
		<PageGrid doc={demoDoc} runtime={demoRuntime} schemas={demoSchemas} items={demoItems} relationships={[]} />
	</div>
{:else if name === 'RelationshipEditor'}
	<div class="demo-col">
		<RelationshipEditor schemas={demoSchemas} />
	</div>
{:else if name === 'RolePickerModal'}
	<div class="demo-row">
		<Btn onclick={() => (rolePickerOpen = true)}>Open role picker</Btn>
	</div>
	{#if rolePickerOpen}
		<RolePickerModal
			kind="dimension"
			chart={demoChart}
			schemas={demoSchemas}
			items={demoItems}
			relationships={[]}
			onClose={() => (rolePickerOpen = false)}
		/>
	{/if}
{:else if name === 'SkeletonSetup'}
	<div class="demo-col" style="max-width: 420px;">
		<SkeletonSetup chart={demoChart} schemas={demoSchemas} items={demoItems} relationships={[]} />
	</div>
{:else if name === 'BarChartRenderer'}
	<div class="chart-stage">
		<BarChartRenderer
			rows={demoRows}
			dimensionAliases={['city']}
			measureAliases={['sales']}
			options={{}}
			annotations={[]}
			heightVh={0.3}
			title="Sales by city"
			selected={null}
			onSelect={noop}
			colorScale={demoRuntime.colorScale}
			fmts={{}}
		/>
	</div>
{:else if name === 'HeatmapRenderer'}
	<div class="chart-stage">
		<HeatmapRenderer
			rows={[
				{ city: 'Berlin', month: 'Jan', sales: 120 },
				{ city: 'Berlin', month: 'Feb', sales: 140 },
				{ city: 'Tokyo', month: 'Jan', sales: 220 },
				{ city: 'Tokyo', month: 'Feb', sales: 190 },
				{ city: 'Oslo', month: 'Jan', sales: 80 },
				{ city: 'Oslo', month: 'Feb', sales: 95 }
			]}
			dimensionAliases={['city', 'month']}
			measureAliases={['sales']}
			options={{}}
			annotations={[]}
			heightVh={0.3}
			title="Sales heatmap"
			selected={null}
			onSelect={noop}
			colorScale={demoRuntime.colorScale}
			fmts={{}}
		/>
	</div>
{:else if name === 'TableRenderer'}
	<div class="demo-col" style="max-width: 560px;">
		<TableRenderer rows={demoRows} columns={['city', 'region', 'sales']} title="Orders sample" status="ok" error="" />
	</div>
{:else}
	<p class="no-live">
		No standalone preview for this entry.
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
