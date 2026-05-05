<script lang="ts">
	import { Home, Menu, FilePlus2, MessageCircleWarning, Star, ChevronLeft, ChevronRight, Command, CassetteTape, DatabaseBackup } from 'lucide-svelte';
	import Button from '$lib/components/Button.svelte';
	import Tooltip from '$lib/components/Tooltip.svelte';

	interface Props {
		onMenuClick: () => void;
		onUploadClick: () => void;
		onMessagesClick: () => void;
		onCommandClick: () => void;
		messageCount?: number;
	}

	let { onMenuClick, onUploadClick, onMessagesClick, onCommandClick, messageCount = 0 }: Props = $props();

	function goBack() {
		window.history.back();
	}

	function goForward() {
		window.history.forward();
	}
</script>

<header class="h-[60px] bg-[#F3E6FF] dark:bg-purple-800 backdrop-blur-sm border-b border-white/5 dark:border-purple-700/50">
	<div class="flex items-center justify-between h-full px-4">
		<!-- Left side buttons -->
		<div class="flex gap-2">
			<Tooltip text="Open menu" position="bottom">
				<Button
					onclick={onMenuClick}
					variant="ghost"
					size="sm"
					class="shadow-[0_1px_2px_0_rgba(0,0,0,0.03)] px-3.5 py-2"
				>
					<Menu class="w-4.5 h-4.5" />
				</Button>
			</Tooltip>
			<Tooltip text="Go back" position="bottom">
				<Button
					onclick={goBack}
					variant="ghost"
					size="sm"
					class="shadow-[0_1px_2px_0_rgba(0,0,0,0.03)] px-3.5 py-2"
				>
					<ChevronLeft class="w-4.5 h-4.5" />
				</Button>
			</Tooltip>
			<Tooltip text="Go forward" position="bottom">
				<Button
					onclick={goForward}
					variant="ghost"
					size="sm"
					class="shadow-[0_1px_2px_0_rgba(0,0,0,0.03)] px-3.5 py-2"
				>
					<ChevronRight class="w-4.5 h-4.5" />
				</Button>
			</Tooltip>
			<Tooltip text="Go to home page" position="bottom">
				<Button
					href="/"
					variant="ghost"
					size="sm"
					class="shadow-[0_1px_2px_0_rgba(0,0,0,0.03)] px-3.5 py-2"
				>
					<Home class="w-4.5 h-4.5" />
				</Button>
			</Tooltip>
			<Tooltip text="Quick upload file" position="bottom">
				<Button
					onclick={onUploadClick}
					variant="ghost"
					size="sm"
					class="shadow-[0_1px_2px_0_rgba(0,0,0,0.03)] px-3.5 py-2"
				>
					<FilePlus2 class="w-4.5 h-4.5" />
				</Button>
			</Tooltip>
			<Tooltip text="Load data from sources" position="bottom">
				<Button
					href="/data-load"
					variant="ghost"
					size="sm"
					class="shadow-[0_1px_2px_0_rgba(0,0,0,0.03)] px-3.5 py-2"
				>
					<DatabaseBackup class="w-4.5 h-4.5" />
				</Button>
			</Tooltip>
			<Tooltip text="Bookmarks" position="bottom">
				<Button
					href="/bookmarks"
					variant="ghost"
					size="sm"
					class="shadow-[0_1px_2px_0_rgba(0,0,0,0.03)] px-3.5 py-2"
				>
					<Star class="w-4.5 h-4.5" />
				</Button>
			</Tooltip>
			<Tooltip text="Command palette (Ctrl+K)" position="bottom">
				<Button
					onclick={onCommandClick}
					variant="ghost"
					size="sm"
					class="shadow-[0_1px_2px_0_rgba(0,0,0,0.03)] px-3.5 py-2"
				>
					<Command class="w-4.5 h-4.5" />
				</Button>
			</Tooltip>
		</div>

		<!-- Right side buttons -->
		<div class="flex gap-2">
			<Tooltip text="Warphead internals" position="bottom">
				<Button
					href="/warphead"
					variant="ghost"
					size="sm"
					class="shadow-[0_1px_2px_0_rgba(0,0,0,0.03)] px-3.5 py-2"
				>
					<CassetteTape class="w-4.5 h-4.5" />
				</Button>
			</Tooltip>
			<Tooltip text="View system messages" position="bottom">
				<Button
					variant="ghost"
					size="sm"
					onclick={onMessagesClick}
					class="relative shadow-[0_1px_2px_0_rgba(0,0,0,0.03)] px-3.5 py-2"
				>
					<MessageCircleWarning class="w-4.5 h-4.5" />
					{#if messageCount > 0}
						<span class="absolute -top-1 -right-1 flex h-5 w-5 items-center justify-center rounded-full bg-[#9B00FF] text-xs font-bold text-white">
							{messageCount > 9 ? '9+' : messageCount}
						</span>
					{/if}
				</Button>
			</Tooltip>
		</div>
	</div>
</header>

