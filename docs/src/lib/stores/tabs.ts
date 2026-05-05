import { writable } from 'svelte/store';
import { goto } from '$app/navigation';

export interface Tab {
	id: string;
	title: string;
	url: string;
	isActive: boolean;
}

function createTabStore() {
	const { subscribe, set, update } = writable<Tab[]>([
		{
			id: 'home',
			title: 'Home',
			url: '/',
			isActive: true
		}
	]);

	return {
		subscribe,
		addTab: (title: string, url: string) => {
			console.log('🆕 addTab called:', { title, url });
			const newTabId = `tab_${Date.now()}`;
			update(tabs => {
				console.log('📝 Current tabs before adding:', tabs);
				// Deactivate all other tabs
				const updatedTabs = tabs.map(tab => ({ ...tab, isActive: false }));
				// Add new tab as active
				const newTabs = [...updatedTabs, { id: newTabId, title, url, isActive: true }];
				console.log('📝 New tabs after adding:', newTabs);
				return newTabs;
			});
			// Navigate to the new tab's URL
			console.log('🔗 Navigating to:', url);
			goto(url);
		},
		setActiveTab: (id: string) => {
			update(tabs => {
				const targetTab = tabs.find(tab => tab.id === id);
				if (targetTab) {
					goto(targetTab.url);
				}
				return tabs.map(tab => ({ ...tab, isActive: tab.id === id }));
			});
		},
		closeTab: (id: string) => {
			update(tabs => {
				const filtered = tabs.filter(tab => tab.id !== id);
				// If we closed the active tab, activate the last tab
				const hasActiveTab = filtered.some(tab => tab.isActive);
				if (!hasActiveTab && filtered.length > 0) {
					filtered[filtered.length - 1].isActive = true;
					goto(filtered[filtered.length - 1].url);
				}
				return filtered;
			});
		},
		reset: () => set([{ id: 'home', title: 'Home', url: '/', isActive: true }])
	};
}

export const tabs = createTabStore();

