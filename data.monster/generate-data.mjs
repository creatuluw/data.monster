import { writeFileSync } from 'fs';

const products = [
	{ name: 'Wireless Headphones', category: 'Electronics', basePrice: 79.99 },
	{ name: 'Bluetooth Speaker', category: 'Electronics', basePrice: 49.99 },
	{ name: 'USB-C Hub', category: 'Electronics', basePrice: 34.99 },
	{ name: 'Mechanical Keyboard', category: 'Electronics', basePrice: 129.99 },
	{ name: 'Gaming Mouse', category: 'Electronics', basePrice: 59.99 },
	{ name: 'Standing Desk', category: 'Furniture', basePrice: 499.99 },
	{ name: 'Ergonomic Chair', category: 'Furniture', basePrice: 349.99 },
	{ name: 'Monitor Arm', category: 'Furniture', basePrice: 89.99 },
	{ name: 'Desk Lamp', category: 'Furniture', basePrice: 44.99 },
	{ name: 'Bookshelf', category: 'Furniture', basePrice: 199.99 },
	{ name: 'Running Shoes', category: 'Sports', basePrice: 119.99 },
	{ name: 'Yoga Mat', category: 'Sports', basePrice: 29.99 },
	{ name: 'Dumbbell Set', category: 'Sports', basePrice: 149.99 },
	{ name: 'Resistance Bands', category: 'Sports', basePrice: 19.99 },
	{ name: 'Water Bottle', category: 'Sports', basePrice: 24.99 },
	{ name: 'Coffee Maker', category: 'Kitchen', basePrice: 89.99 },
	{ name: 'Blender', category: 'Kitchen', basePrice: 69.99 },
	{ name: 'Air Fryer', category: 'Kitchen', basePrice: 99.99 },
	{ name: 'Knife Set', category: 'Kitchen', basePrice: 59.99 },
	{ name: 'Toaster Oven', category: 'Kitchen', basePrice: 79.99 }
];

const regions = ['North', 'South', 'East', 'West', 'Central'];
const segments = ['Enterprise', 'SMB', 'Consumer', 'Government', 'Startup'];
const channels = ['Online', 'Retail', 'Wholesale', 'Direct', 'Partner'];
const salespeople = [
	'Alice Johnson', 'Bob Smith', 'Carol Williams', 'David Brown', 'Eve Davis',
	'Frank Miller', 'Grace Wilson', 'Henry Moore', 'Ivy Taylor', 'Jack Anderson'
];
const statuses = ['Completed', 'Pending', 'Refunded', 'Cancelled'];
const paymentMethods = ['Credit Card', 'PayPal', 'Bank Transfer', 'Invoice', 'Crypto'];

function randInt(min, max) {
	return Math.floor(Math.random() * (max - min + 1)) + min;
}

function randFloat(min, max, decimals = 2) {
	return parseFloat((Math.random() * (max - min) + min).toFixed(decimals));
}

function pick(arr) {
	return arr[randInt(0, arr.length - 1)];
}

function randomDate(start, end) {
	return new Date(start.getTime() + Math.random() * (end.getTime() - start.getTime()));
}

const rows = [];

for (let i = 1; i <= 10000; i++) {
	const product = pick(products);
	const quantity = randInt(1, 50);
	const discountPct = randFloat(0, 30, 1);
	const unitPrice = product.basePrice * randFloat(0.9, 1.15);
	const grossAmount = parseFloat((unitPrice * quantity).toFixed(2));
	const discountAmount = parseFloat((grossAmount * discountPct / 100).toFixed(2));
	const netAmount = parseFloat((grossAmount - discountAmount).toFixed(2));
	const date = randomDate(new Date('2024-01-01'), new Date('2025-12-31'));

	rows.push({
		id: i,
		date: date.toISOString().split('T')[0],
		product: product.name,
		category: product.category,
		region: pick(regions),
		segment: pick(segments),
		channel: pick(channels),
		salesperson: pick(salespeople),
		quantity,
		unitPrice: parseFloat(unitPrice.toFixed(2)),
		discountPct,
		netAmount,
		status: pick(statuses),
		paymentMethod: pick(paymentMethods),
		rating: randInt(1, 5)
	});
}

writeFileSync('src/lib/data.json', JSON.stringify(rows));
console.log(`Generated ${rows.length} rows`);
