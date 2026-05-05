# Initialization Assets for warphead.dev

This folder contains pre-built assets to initialize the warphead.dev application with sample data, a complete star schema data model, and pre-configured metrics and dimensions.

## 📦 What's Included

### 1. **sales_data.csv** (973 records)
A comprehensive sales transaction dataset with realistic business data including:
- **Transaction Details**: Order and ship dates, transaction IDs, status
- **Customer Information**: Names, segments (Enterprise/Corporate/Small Business), regions, cities
- **Product Information**: Names, categories, sub-categories, costs, and prices
- **Sales Details**: Quantities, discounts, payment methods, shipping methods
- **Sales Representatives**: 12 different sales people across transactions
- **Time Range**: January 2023 - October 2024

**File Size**: ~164 KB  
**Use Case**: Recent sales data for building analytics and data models  
**Auto-loaded**: ✅ Automatically copied to `data/main/` folder on first app launch for persistence and easy access

---

### 2. **sales_history.csv** (1,234,567 records)
A large-scale historical sales dataset for testing performance with realistic data volumes:
- **Transaction Details**: Same structure as sales_data.csv
- **Customer Information**: Same 15 customers across all transactions
- **Product Information**: Same 20 products with consistent pricing
- **Sales Details**: Realistic distributions of quantities and discounts
- **Sales Representatives**: Same 12 sales people
- **Time Range**: January 2019 - October 2024 (5 years of data)

**File Size**: ~200 MB  
**Use Case**: Historical trend analysis, performance testing with large datasets, time-series analysis  
**Auto-loaded**: ✅ Automatically fetched from remote URL (`http://patrick.te9.nl/data/sales_history.csv`) and saved to `data/remote/init_data/` on first app launch  
**Performance**: ~1.2M records provide realistic query performance testing scenarios  
**Loading**: Downloaded from remote server during initialization to avoid bundling large file with the app

---

### 3. **create-star-schema.sql**
A comprehensive SQL script that transforms both `sales_data` and `sales_history` tables into a proper star schema with:

#### Dimension Tables (3):
- **dim_customers**: Combined customer master data from both sources with segments, regions, and order history
- **dim_products**: Combined product catalog from both sources with categories, costs, prices, and profit margins  
- **dim_time**: Combined time dimension from both sources (2019-2024) with year, quarter, month, day, and weekend indicators

#### Fact Tables (2):
- **fact_sales**: Recent sales fact table (973 records from sales_data)
  - Pre-calculated measures: gross_revenue, net_revenue, total_cost, profit, profit_margin_pct
  - Uses UDFs: `revenue()`, `profit_margin()`, `discount_price()`
  - Foreign keys to all dimension tables
- **fact_sales_history**: Historical sales fact table (1.2M+ records from sales_history)
  - Same structure and measures as fact_sales
  - Enables long-term trend analysis and performance testing
  - Properly registered as 'model' type tables in `_warphead_table_metadata`
  - Relationships defined in `_warphead_relationships`

**Key Features**:
- Uses available UDFs like `autonumber()`, `profit_margin()`, `quarter_year()`, `is_weekend()`
- Automatically registers tables as 'model' type for data model canvas visibility
- Pre-defines relationships for automatic JOIN path resolution
- Includes verification queries

---

### 4. **metrics.json** (6 metrics)
Pre-defined business metrics ready to load:

| Metric | Formula | Source Table | Description |
|--------|---------|--------------|-------------|
| **Total Revenue** | `SUM(net_revenue)` | fact_sales | Net revenue after discounts |
| **Total Profit** | `SUM(profit)` | fact_sales | Profit after COGS and discounts |
| **Average Order Value** | `safe_divide(SUM(net_revenue), COUNT(DISTINCT transaction_id))` | fact_sales | Average revenue per transaction |
| **Total Units Sold** | `SUM(quantity)` | fact_sales | Total quantity of products sold |
| **Average Profit Margin %** | `safe_percentage(SUM(profit), SUM(net_revenue))` | fact_sales | Overall profit margin percentage |
| **Order Count** | `COUNT(DISTINCT transaction_id)` | fact_sales | Total number of orders |

**UDFs Used**: `safe_divide()`, `safe_percentage()`

---

### 5. **dimensions.json** (7 dimensions)
Pre-defined dimensions for flexible analysis:

| Dimension | Field | Source Table | Use Case |
|-----------|-------|--------------|----------|
| **Customer Name** | customer_name | dim_customers | Customer-level analysis |
| **Customer Segment** | customer_segment | dim_customers | Segment performance |
| **Region** | region | dim_customers | Geographic analysis |
| **Product Name** | product_name | dim_products | Product performance |
| **Product Category** | category | dim_products | Category-level trends |
| **Order Year** | year | dim_time | Year-over-year comparison |
| **Quarter Year** | quarter_year | dim_time | Quarterly performance |

---

## 🚀 How to Use

### Automatic Initialization (Recommended)

The initialization happens **automatically** when you first launch the application:

1. **sales_data.csv** is automatically copied to your `data/main/` folder
2. **sales_history.csv** is automatically downloaded from remote URL and saved to `data/remote/init_data/` folder
3. The `sales_data` and `sales_history` tables are created in DuckDB from the CSVs
4. The star schema (5 tables total) is automatically generated:
   - 3 dimension tables: dim_customers, dim_products, dim_time (combined from both sources)
   - 2 fact tables: fact_sales (recent), fact_sales_history (historical)
5. Pre-configured metrics (6) and dimensions (7) are loaded

**All this happens seamlessly on first launch - no manual intervention needed!**

> **Note**: If the remote download fails (network issues), the app will continue with `sales_data` only and create a working star schema with 4 tables instead of 5.

### Manual Re-initialization (Optional)

If you need to manually reset or re-initialize the data:

### Step 1: Upload the Sales Data CSVs (Manual Only)

1. Open warphead.dev
2. Navigate to **Data > Upload** (`/data/upload`)
3. Select a folder (or create a new one like "sample_data")
4. Upload `sales_data.csv` and `sales_history.csv`
5. The app will automatically create `sales_data` and `sales_history` tables (type: 'source')

### Step 2: Create the Star Schema

1. Navigate to **SQL Studio** (`/sql-studio`)
2. Copy and paste the contents of `create-star-schema.sql`
3. Execute the script
4. This will create:
   - 3 dimension tables (dim_customers, dim_products, dim_time) - combined from both sources
   - 2 fact tables (fact_sales, fact_sales_history)
   - Proper metadata registration
   - Relationship definitions

### Step 3: Verify the Data Model

1. Navigate to **Models > Data Model** (`/models/datamodel`)
2. You should see all 5 model tables in the canvas:
   - `fact_sales` (center - recent data)
   - `fact_sales_history` (center - historical data)
   - `dim_customers` (linked to both fact tables)
   - `dim_products` (linked to both fact tables)
   - `dim_time` (linked to both fact tables)
3. Click "Scan Relationships" to visualize the connections

### Step 4: Load Metrics (Optional - Manual)

Currently, metrics need to be added manually through the UI:

1. Navigate to **Models > Metrics** (`/models/metrics`)
2. Click "Create New Metric"
3. Use the definitions from `metrics.json`:
   - Copy the `metric_name`, `formula`, `source_table`, `description`, and `tags`
   - The system will auto-generate unique slugs

### Step 5: Load Dimensions (Optional - Manual)

1. Navigate to **Models > Dimensions** (`/models/dimensions`)
2. Click "Create New Dimension"
3. Use the definitions from `dimensions.json`:
   - Copy `dimension_name`, `field_name`, `source_table`, `description`, and `tags`

---

## 📊 Sample Queries

### Query 1: 5-Year Revenue Trends by Region
```sql
SELECT 
    t.year,
    c.region,
    COUNT(DISTINCT fh.transaction_id) AS order_count,
    SUM(fh.net_revenue) AS total_revenue,
    SUM(fh.profit) AS total_profit,
    AVG(fh.profit_margin_pct) AS avg_margin_pct
FROM fact_sales_history fh
JOIN dim_customers c ON fh.customer_key = c.customer_key
JOIN dim_time t ON fh.time_key = t.time_key
WHERE fh.order_status = 'Completed'
GROUP BY t.year, c.region
ORDER BY t.year DESC, total_revenue DESC;
```

### Query 2: Top Products by Profit Margin
```sql
SELECT 
    p.product_name,
    p.category,
    p.profit_margin_pct,
    SUM(f.quantity) AS units_sold,
    SUM(f.profit) AS total_profit
FROM fact_sales f
JOIN dim_products p ON f.product_key = p.product_key
WHERE f.order_status = 'Completed'
GROUP BY p.product_name, p.category, p.profit_margin_pct
ORDER BY total_profit DESC
LIMIT 10;
```

### Query 3: Customer Segment Performance (Historical)
```sql
SELECT 
    c.customer_segment,
    t.year,
    COUNT(DISTINCT fh.transaction_id) AS order_count,
    SUM(fh.net_revenue) AS total_revenue,
    safe_divide(SUM(fh.net_revenue), COUNT(DISTINCT fh.transaction_id)) AS avg_order_value,
    safe_percentage(SUM(fh.profit), SUM(fh.net_revenue)) AS avg_margin_pct
FROM fact_sales_history fh
JOIN dim_customers c ON fh.customer_key = c.customer_key
JOIN dim_time t ON fh.time_key = t.time_key
WHERE fh.order_status = 'Completed'
GROUP BY c.customer_segment, t.year
ORDER BY t.year DESC, total_revenue DESC;
```

### Query 4: Recent vs Historical Comparison
```sql
-- Compare recent data (fact_sales) to same period last year (fact_sales_history)
WITH recent AS (
    SELECT 
        c.region,
        SUM(f.net_revenue) AS revenue
    FROM fact_sales f
    JOIN dim_customers c ON f.customer_key = c.customer_key
    JOIN dim_time t ON f.time_key = t.time_key
    WHERE f.order_status = 'Completed' AND t.year = 2024
    GROUP BY c.region
),
last_year AS (
    SELECT 
        c.region,
        SUM(fh.net_revenue) AS revenue
    FROM fact_sales_history fh
    JOIN dim_customers c ON fh.customer_key = c.customer_key
    JOIN dim_time t ON fh.time_key = t.time_key
    WHERE fh.order_status = 'Completed' AND t.year = 2023
    GROUP BY c.region
)
SELECT 
    r.region,
    r.revenue AS revenue_2024,
    l.revenue AS revenue_2023,
    r.revenue - l.revenue AS revenue_change,
    safe_percentage(r.revenue - l.revenue, l.revenue) AS pct_change
FROM recent r
LEFT JOIN last_year l ON r.region = l.region
ORDER BY revenue_change DESC;
```

---

## 🔧 Available UDFs (User-Defined Functions)

The following UDFs are available and used in this initialization:

### Business Functions
- `revenue(quantity, price)` - Calculate revenue
- `profit_margin(revenue, cost)` - Calculate profit margin percentage
- `discount_price(price, discount_pct)` - Calculate discounted price

### Math Functions
- `safe_divide(numerator, denominator)` - Division with zero protection
- `safe_percentage(part, total)` - Percentage with zero protection

### Date Functions
- `quarter_year(date)` - Format as "Q1 2024"
- `is_weekend(date)` - Boolean for Saturday/Sunday
- `days_since(past_date)` - Days from date to today

### Hash Functions
- `autonumber(text)` - Generate numeric ID from text
- `auto_id(text)` - Generate 8-char hex ID

### String Functions
- `full_name(first, last)` - Concatenate with space
- `clean_text(text)` - Normalize whitespace and lowercase
- `format_currency(amount)` - Format as $X,XXX.XX
- `format_percentage(value, decimals)` - Format as percentage

For the complete list, see `src/lib/udfs/library-functions.json`

---

## 📈 Expected Results

After loading all assets, you should have:

✅ **2 Source Tables**: `sales_data` (973 records), `sales_history` (1.2M+ records)  
✅ **5 Model Tables**: `fact_sales`, `fact_sales_history`, `dim_customers`, `dim_products`, `dim_time`  
✅ **6 Relationships**: Both fact tables connected to all dimension tables  
✅ **6 Metrics**: Ready for analysis and charting  
✅ **7 Dimensions**: Ready for grouping and filtering

---

## 🎯 Next Steps

1. **Create Charts**: Use the metrics and dimensions in the Analysis section
2. **Build Dashboards**: Combine multiple charts for executive views
3. **Explore Patterns**: Use the data model canvas to understand relationships
4. **Custom Metrics**: Create your own metrics using the formula builder
5. **Export Data**: Use the export features to share insights

---

## 📝 Notes

- The `sales_data` and `sales_history` tables are marked as **'source'** type (raw data)
- The star schema tables are marked as **'model'** type (transformed data)
- Only **'model'** type tables appear in the data model canvas
- The relationship scanner works automatically on model tables
- Metrics can use either `fact_sales` (recent) or `fact_sales_history` (historical) as their source
- Dimensions are combined from both sources for complete coverage
- Use `fact_sales` for current/recent analysis, `fact_sales_history` for trends and performance testing

---

## 🔄 Regenerating Data

To regenerate the sales data CSVs with different records:

### Generate sales_data.csv (973 records)
```bash
# Using Python
python src/lib/on_init/generate-sales-data.py

# Using PowerShell
powershell -ExecutionPolicy Bypass -File src/lib/on_init/generate-sales-data.ps1
```

### Generate sales_history.csv (1,234,567 records)
```bash
# Using Python
python src/lib/on_init/generate-sales-history.py

# Using PowerShell
powershell -ExecutionPolicy Bypass -File src/lib/on_init/generate-sales-history.ps1
```

**Note**: Generating `sales_history.csv` will take several minutes and create a ~200MB file.

You can modify the scripts to adjust:
- Number of records (`NUM_RECORDS` variable)
- Product catalog
- Customer list
- Date ranges
- Discount distributions
- Batch size (for memory management in large files)

---

## 🤝 Contributing

To add more initialization assets:
1. Add new metrics to `metrics.json`
2. Add new dimensions to `dimensions.json`
3. Extend the SQL script with additional derived tables
4. Update this README with usage instructions

---

**Created**: October 28, 2025  
**Version**: 1.0.0  
**Compatible with**: warphead.dev v1.x

