import { writable } from 'svelte/store';

export interface ToastNotification {
	id: string;
	message: string;
	type: 'success' | 'error' | 'warning' | 'info';
	duration?: number;
}

function createToastStore() {
	const { subscribe, update } = writable<ToastNotification[]>([]);
	
	const add = (message: string, type: ToastNotification['type'] = 'info', duration = 3000) => {
		const id = Math.random().toString(36).substr(2, 9);
		const toast: ToastNotification = { id, message, type, duration };
		
		update(toasts => [...toasts, toast]);
		
		return id;
	};
	
	const remove = (id: string) => {
		update(toasts => toasts.filter(t => t.id !== id));
	};
	
	return {
		subscribe,
		add,
		remove,
		success: (message: string, duration?: number) => add(message, 'success', duration),
		error: (message: string, duration?: number) => add(message, 'error', duration),
		warning: (message: string, duration?: number) => add(message, 'warning', duration),
		info: (message: string, duration?: number) => add(message, 'info', duration)
	};
}

export const toasts = createToastStore();

