<script lang="ts">
	let {
		onfile,
		onurl,
		loading = false
	}: {
		onfile: (file: File) => void;
		onurl: (url: string) => void;
		loading?: boolean;
	} = $props();

	let urlInput = $state('');
	let isDragOver = $state(false);

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		isDragOver = false;
		if (e.dataTransfer?.files.length) {
			onfile(e.dataTransfer.files[0]);
		}
	}

	function handleFileInput(e: Event) {
		const input = e.target as HTMLInputElement;
		if (input.files?.length) {
			onfile(input.files[0]);
		}
	}

	function handleUrlSubmit() {
		const url = urlInput.trim();
		if (url) onurl(url);
	}
</script>

<div class="mx-auto flex max-w-lg flex-col gap-8 py-12">
	<!-- File drop zone -->
	<label
		class="flex cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed transition-colors {isDragOver
			? 'border-sage-400 bg-sage-50'
			: 'border-sand-200 bg-sand-50 hover:border-sand-300 hover:bg-sand-100'} py-16"
		role="button"
		tabindex="0"
		ondragover={(e) => { e.preventDefault(); isDragOver = true; }}
		ondragleave={() => { isDragOver = false; }}
		ondrop={handleDrop}
		onkeydown={(e) => e.key === 'Enter' && (e.target as HTMLElement).querySelector('input')?.click()}
	>
		<svg class="mb-3 h-8 w-8 text-sand-300" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
			<path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5m-13.5-9L12 3m0 0 4.5 4.5M12 3v13.5" />
		</svg>
		<p class="text-sm font-medium text-sand-600">
			{loading ? 'Loading...' : 'Drop a file here'}
		</p>
		<p class="mt-1 text-xs text-sand-400">or click to browse</p>
		<p class="mt-2 text-xs text-sand-300">CSV · Parquet · JSON · JSONL</p>
		<input type="file" accept=".csv,.parquet,.json,.jsonl,.ndjson" class="hidden" onchange={handleFileInput} />
	</label>

	<!-- Divider -->
	<div class="flex items-center gap-3">
		<div class="h-px flex-1 bg-sand-200"></div>
		<span class="text-xs text-sand-400">or paste a URL</span>
		<div class="h-px flex-1 bg-sand-200"></div>
	</div>

	<!-- URL input -->
	<div class="flex gap-2">
		<div class="flex flex-1 items-center rounded-lg border border-sand-200 bg-white px-3 py-2 shadow-sm transition-colors focus-within:border-sage-400 focus-within:ring-2 focus-within:ring-sage-200">
			<svg class="mr-2 h-4 w-4 shrink-0 text-sand-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
				<path stroke-linecap="round" stroke-linejoin="round" d="M13.19 8.688a4.5 4.5 0 0 1 1.242 7.244l-4.5 4.5a4.5 4.5 0 0 1-6.364-6.364l1.757-1.757m13.35-.622 1.757-1.757a4.5 4.5 0 0 0-6.364-6.364l-4.5 4.5a4.5 4.5 0 0 0 1.242 7.244" />
			</svg>
			<input
				type="url"
				bind:value={urlInput}
				placeholder="https://example.com/data.csv"
				class="flex-1 bg-transparent text-sm text-sand-800 outline-none placeholder:text-sand-300"
				onkeydown={(e) => e.key === 'Enter' && handleUrlSubmit()}
			/>
		</div>
		<button
			onclick={handleUrlSubmit}
			disabled={!urlInput.trim() || loading}
			class="shrink-0 rounded-lg bg-copper-400 px-4 py-2 text-sm font-medium text-white shadow-sm transition-all hover:bg-copper-500 disabled:cursor-not-allowed disabled:opacity-40"
		>
			Go
		</button>
	</div>
</div>
