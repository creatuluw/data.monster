/**
 * Table demo data — dummy rows, no workspace or DuckDB needed.
 */
import type { LibraryDemo } from '$lib/library/types';

export const tableDemo: LibraryDemo = {
	title: 'Recent orders',
	subtitle: 'Demo data — component library',
	dimensionAliases: [],
	measureAliases: ['amount'],
	columns: ['order', 'region', 'channel', 'amount', 'status'],
	rows: [
		{ order: 'SO-1041', region: 'EMEA', channel: 'Online', amount: 1250, status: 'Shipped' },
		{ order: 'SO-1042', region: 'APAC', channel: 'Retail', amount: 890, status: 'Pending' },
		{ order: 'SO-1043', region: 'AMER', channel: 'Online', amount: 2410, status: 'Shipped' },
		{ order: 'SO-1044', region: 'EMEA', channel: 'Partner', amount: 640, status: 'Cancelled' },
		{ order: 'SO-1045', region: 'AMER', channel: 'Online', amount: 1830, status: 'Shipped' },
		{ order: 'SO-1046', region: 'APAC', channel: 'Online', amount: 720, status: 'Pending' }
	]
};
