/**
 * Dynamic Chart System
 * 
 * Allows users to create chart templates in the UI and use them as components
 * 
 * Example:
 * 1. Create a template called "SalesBarChart" in the Chart Builder
 * 2. Use it in your analysis views:
 *    <Chart component_name="SalesBarChart" data={salesData} x_dimension="product" y_metric="revenue" />
 */

export { default as Chart } from '../components/Chart.svelte';
export * from './chart-template-runtime';

