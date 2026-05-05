# Sales History Implementation Summary

## Overview
This document describes the implementation of the `sales_history.csv` dataset containing 1,234,567 records for historical sales data analysis and performance testing.

## Files Created/Modified

### New Files Created

1. **`generate-sales-history.py`**
   - Python script to generate 1.2M+ sales records
   - Covers 5 years of data (2019-2024)
   - Uses batched writing for memory efficiency
   - Estimated runtime: 2-5 minutes
   - Output file size: ~200 MB

2. **`generate-sales-history.ps1`**
   - PowerShell equivalent of the Python script
   - Same functionality for Windows environments
   - Batched processing for performance

### Modified Files

1. **`create-star-schema.sql`**
   - Updated to process both `sales_data` and `sales_history` tables
   - Creates combined dimension tables from both sources
   - Creates two fact tables: `fact_sales` and `fact_sales_history`
   - Registers both fact tables in metadata
   - Defines relationships for both fact tables

2. **`README.md`**
   - Added documentation for `sales_history.csv`
   - Updated sample queries to show historical analysis
   - Added comparison queries between recent and historical data
   - Updated expected results section

## Data Structure

### sales_history.csv Schema
```
transaction_id, order_date, ship_date, customer_name, customer_segment,
region, city, sales_person, product_name, category, sub_category,
quantity, unit_price, unit_cost, discount_percent, order_status,
payment_method, shipping_method
```

### Key Characteristics
- **Records**: 1,234,567
- **Date Range**: 2019-01-01 to 2024-10-28 (5 years)
- **Customers**: Same 15 customers as sales_data
- **Products**: Same 20 products as sales_data
- **Sales Reps**: Same 12 sales representatives
- **Distributions**: Same realistic distributions of statuses, quantities, discounts

## Star Schema Changes

### Original Schema (sales_data only)
```
fact_sales (973 records)
  ├─ dim_customers (15 records)
  ├─ dim_products (20 records)
  └─ dim_time (~650 records)
```

### Enhanced Schema (both sources)
```
fact_sales (973 records) ────┐
                             ├─ dim_customers (15 records)
fact_sales_history (1.2M+)───┤
                             ├─ dim_products (20 records)
                             └─ dim_time (~2100 records)
```

### Benefits
- **Combined Dimensions**: Dimensions are built from both sources, ensuring complete coverage
- **Separate Fact Tables**: Allows easy comparison between recent and historical data
- **Performance Testing**: Large dataset enables realistic query performance testing
- **Trend Analysis**: 5 years of data enables meaningful year-over-year analysis

## Use Cases

### 1. Historical Trend Analysis
Query 5 years of data to identify long-term patterns:
```sql
SELECT 
    t.year,
    c.region,
    SUM(fh.net_revenue) AS total_revenue
FROM fact_sales_history fh
JOIN dim_customers c ON fh.customer_key = c.customer_key
JOIN dim_time t ON fh.time_key = t.time_key
GROUP BY t.year, c.region;
```

### 2. Performance Testing
Test query performance with 1.2M+ records:
```sql
SELECT 
    p.category,
    COUNT(*) AS order_count,
    AVG(fh.profit_margin_pct) AS avg_margin
FROM fact_sales_history fh
JOIN dim_products p ON fh.product_key = p.product_key
GROUP BY p.category;
```

### 3. Year-over-Year Comparison
Compare current performance to historical:
```sql
WITH current_year AS (
    SELECT region, SUM(net_revenue) AS revenue
    FROM fact_sales f
    JOIN dim_customers c ON f.customer_key = c.customer_key
    WHERE order_status = 'Completed'
    GROUP BY region
),
last_year AS (
    SELECT region, SUM(net_revenue) AS revenue
    FROM fact_sales_history fh
    JOIN dim_customers c ON fh.customer_key = c.customer_key
    JOIN dim_time t ON fh.time_key = t.time_key
    WHERE order_status = 'Completed' AND t.year = 2023
    GROUP BY region
)
SELECT 
    c.region,
    c.revenue AS current_revenue,
    l.revenue AS last_year_revenue,
    (c.revenue - l.revenue) / l.revenue * 100 AS pct_change
FROM current_year c
LEFT JOIN last_year l ON c.region = l.region;
```

### 4. DuckDB Performance Testing
Test DuckDB's columnar storage and query optimization:
- Aggregations over millions of rows
- JOIN performance with large fact tables
- GROUP BY with multiple dimensions
- Time-series queries and window functions

## Generation Instructions

### To Generate the Data File

**Using Python** (recommended):
```bash
cd E:\warphead.dev
python src/lib/on_init/generate-sales-history.py
```

**Using PowerShell**:
```powershell
cd E:\warphead.dev
powershell -ExecutionPolicy Bypass -File src/lib/on_init/generate-sales-history.ps1
```

### Generation Process
1. Script initializes with configuration (1,234,567 records)
2. Generates records in batches of 10,000 for memory efficiency
3. Writes CSV header first
4. Generates and writes batches with progress updates
5. Displays final statistics (file size, average record size)

### Expected Output
```
Generating 1,234,567 sales history records...
Date range: 2019-01-01 to 2024-10-28
Writing to: src/lib/on_init/sales_history.csv

  Generated 10,000 records... (0.8%)
  Generated 20,000 records... (1.6%)
  ...
  Generated 1,234,567 records... (100.0%)

✓ Successfully generated sales_history.csv with 1,234,567 records!
  File size: 201.45 MB (211,234,567 bytes)
  Average record size: 171 bytes
```

## Integration with Init System

### Automatic Loading (on first launch)
1. Copy `sales_history.csv` to `data/main/` folder
2. Create `sales_history` table from CSV (marked as 'source')
3. Run `create-star-schema.sql` to build dimensions and fact tables
4. Register tables in metadata system
5. Define relationships

### Manual Loading
1. Upload `sales_history.csv` via Data > Upload interface
2. Run `create-star-schema.sql` in SQL Studio
3. Verify in Models > Data Model canvas

## Performance Considerations

### File Size
- CSV file: ~200 MB
- DuckDB storage: Compressed columnar format (typically 30-50% of CSV size)
- Memory usage during generation: <100 MB (batched processing)

### Query Performance
- DuckDB handles 1.2M rows efficiently with columnar storage
- Aggregations complete in milliseconds to low seconds
- JOINs with dimension tables are fast due to small dimension sizes
- Recommended to add indexes on frequently queried columns

### Storage
- Ensure adequate disk space (~300 MB for CSV + DuckDB database)
- Consider .gitignore for the CSV file due to size
- Git LFS recommended if version control needed for data files

## Future Enhancements

### Possible Extensions
1. **Parameterized Generation**: Allow custom record counts, date ranges
2. **Multiple Files**: Split into yearly files for easier management
3. **Data Variations**: Add seasonal patterns, growth trends
4. **Additional Tables**: Order line items, returns, customer events
5. **Real-time Data**: Streaming data generation for testing

### Customization
Modify the generation scripts to adjust:
- `NUM_RECORDS`: Change total record count
- Date range: Adjust `start_date` and `end_date`
- Data distributions: Modify product mix, customer segments
- Batch size: Tune `BATCH_SIZE` for memory/speed tradeoff

## Testing Checklist

- [x] Python generation script created
- [x] PowerShell generation script created
- [x] SQL schema updated to handle both sources
- [x] Dimension tables combine data from both sources
- [x] fact_sales_history table created
- [x] Relationships defined for both fact tables
- [x] Metadata registration updated
- [x] README documentation updated
- [x] Sample queries provided
- [ ] Generate actual sales_history.csv file
- [ ] Test initialization process
- [ ] Verify query performance
- [ ] Test data model canvas display

## Notes

- **Do not commit** the generated `sales_history.csv` file to git (too large)
- Add to `.gitignore` if not already present
- Users should generate locally using the provided scripts
- Consider providing download link for pre-generated file if needed
- DuckDB will handle the large dataset efficiently in memory or with disk spillover

---

**Created**: October 29, 2025  
**Author**: AI Assistant  
**Version**: 1.0.0

