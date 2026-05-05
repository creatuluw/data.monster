import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export type Theme = 'light' | 'dark' | 'system';

// Get initial theme from localStorage or default to 'system'
function getInitialTheme(): Theme {
	if (!browser) return 'system';
	
	const stored = localStorage.getItem('theme');
	if (stored === 'light' || stored === 'dark' || stored === 'system') {
		return stored;
	}
	return 'system';
}

// Create the store
export const theme = writable<Theme>(getInitialTheme());

// Apply theme to the document
function applyTheme(selectedTheme: Theme) {
	if (!browser) return;
	
	const root = document.documentElement;
	
	if (selectedTheme === 'system') {
		// Check system preference
		const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
		if (prefersDark) {
			root.classList.add('dark');
		} else {
			root.classList.remove('dark');
		}
	} else if (selectedTheme === 'dark') {
		root.classList.add('dark');
	} else {
		root.classList.remove('dark');
	}
	
	// Store preference
	localStorage.setItem('theme', selectedTheme);
}

// Subscribe to theme changes and apply them
if (browser) {
	theme.subscribe(applyTheme);
	
	// Listen for system theme changes when theme is set to 'system'
	const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
	mediaQuery.addEventListener('change', () => {
		theme.update(current => {
			if (current === 'system') {
				applyTheme('system');
			}
			return current;
		});
	});
	
	// Apply initial theme on load
	const initialTheme = getInitialTheme();
	applyTheme(initialTheme);
}

// Helper to get the currently applied theme (actual light/dark, not 'system')
export function getAppliedTheme(): 'light' | 'dark' {
	if (!browser) return 'light';
	return document.documentElement.classList.contains('dark') ? 'dark' : 'light';
}

