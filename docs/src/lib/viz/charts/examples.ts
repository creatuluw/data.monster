// Example: Using the BarChart component directly in your app

import { BarChart } from '$lib/viz/charts';

// Example 1: Basic Bar Chart
let salesData = [
	{ month: 'Jan', sales: 12000 },
	{ month: 'Feb', sales: 19000 },
	{ month: 'Mar', sales: 8500 },
	{ month: 'Apr', sales: 15000 },
	{ month: 'May', sales: 22000 }
];

// In your template:
// <BarChart data={salesData} x="month" y="sales" title="Monthly Sales" />

// Example 2: Horizontal Bar Chart with Custom Colors
let productData = [
	{ product: 'Product A', revenue: 45000 },
	{ product: 'Product B', revenue: 67000 },
	{ product: 'Product C', revenue: 34000 },
	{ product: 'Product D', revenue: 89000 }
];

// In your template:
// <BarChart 
//   data={productData} 
//   x="product" 
//   y="revenue" 
//   swapXY={true}
//   fillColor="#6366F1"
//   yFmt="usd0k"
//   title="Product Revenue"
// />

// Example 3: Multi-Series Stacked Bar Chart
let categoryData = [
	{ month: 'Jan', category: 'Electronics', sales: 12000 },
	{ month: 'Jan', category: 'Clothing', sales: 8000 },
	{ month: 'Feb', category: 'Electronics', sales: 15000 },
	{ month: 'Feb', category: 'Clothing', sales: 9500 }
];

// In your template:
// <BarChart 
//   data={categoryData} 
//   x="month" 
//   y="sales" 
//   series="category"
//   type="stacked"
//   colorPalette={['#CA3500', '#6366F1', '#10B981']}
//   title="Sales by Category"
// />

// Example 4: Minimal (Auto-detected columns)
let simpleData = [
	{ name: 'A', value: 100 },
	{ name: 'B', value: 200 },
	{ name: 'C', value: 150 }
];

// In your template:
// <BarChart data={simpleData} />
// x and y will be auto-detected from the first and second columns

export {};

