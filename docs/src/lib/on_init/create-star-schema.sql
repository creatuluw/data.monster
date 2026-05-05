-- ============================================================================
-- Star Schema Creation from Sales Data
-- ============================================================================
-- This script transforms the flat sales_data and sales_history tables into 
-- a proper star schema with:
-- - 1 UNIFIED Fact Table: fact_sales (with ONLY keys and measures)
-- - 3 Dimension Tables: dim_customers, dim_products, dim_time
--
-- Strategy: First union all facts, then build dimensions, then add keys to facts
-- ============================================================================

-- ============================================================================
-- Step 1: Create UNIFIED Fact Table with Surrogate Keys
-- ============================================================================
-- Combine ALL sales data and generate keys using the same hash functions as dimensions
CREATE TABLE fact_sales AS
WITH all_sales AS (
    SELECT 
        transaction_id,
        customer_name,
        product_name,
        order_date,
        ship_date,
        sales_person,
        order_status,
        payment_method,
        shipping_method,
        quantity,
        unit_price,
        unit_cost,
        discount_percent,
        'current' AS data_source
    FROM sales_data
    
    UNION ALL
    
    SELECT 
        transaction_id,
        customer_name,
        product_name,
        order_date,
        ship_date,
        sales_person,
        order_status,
        payment_method,
        shipping_method,
        quantity,
        unit_price,
        unit_cost,
        discount_percent,
        'history' AS data_source
    FROM sales_history
)
SELECT 
    transaction_id,
    
    -- Foreign Keys (using same hash functions as dimensions)
    auto_id(customer_name) AS customer_key,
    auto_id(product_name) AS product_key,
    auto_id(order_date::VARCHAR) AS time_key,
    
    -- Degenerate Dimensions (transaction-specific attributes that stay in fact)
    ship_date,
    sales_person,
    order_status,
    payment_method,
    shipping_method,
    data_source,
    
    -- Measures (numeric facts)
    quantity,
    unit_price,
    unit_cost,
    discount_percent,
    
    -- Calculated Measures using UDFs
    revenue(quantity, unit_price) AS gross_revenue,
    revenue(quantity, unit_price) * (1 - discount_percent / 100) AS net_revenue,
    revenue(quantity, unit_cost) AS total_cost,
    revenue(quantity, unit_price) * (1 - discount_percent / 100) - revenue(quantity, unit_cost) AS profit,
    profit_margin(
        revenue(quantity, unit_price) * (1 - discount_percent / 100),
        revenue(quantity, unit_cost)
    ) AS profit_margin_pct,
    discount_price(unit_price, discount_percent) AS discounted_unit_price,
    CASE 
        WHEN order_status = 'Completed' THEN revenue(quantity, unit_price) * (1 - discount_percent / 100)
        ELSE 0 
    END AS completed_revenue
FROM all_sales;

-- ============================================================================
-- Step 2: Create Dimension Table - dim_customers
-- ============================================================================
-- Build customer dimension from source tables
CREATE TABLE dim_customers AS
WITH all_customers AS (
    SELECT DISTINCT customer_name, customer_segment, region, city, transaction_id, order_date
    FROM sales_data
    UNION ALL
    SELECT DISTINCT customer_name, customer_segment, region, city, transaction_id, order_date
    FROM sales_history
)
SELECT 
    auto_id(customer_name) AS customer_key,
    customer_name,
    customer_segment,
    region,
    city,
    COUNT(DISTINCT transaction_id) AS total_orders,
    MIN(order_date) AS first_order_date,
    MAX(order_date) AS last_order_date
FROM all_customers
GROUP BY customer_name, customer_segment, region, city;

-- ============================================================================
-- Step 3: Create Dimension Table - dim_products
-- ============================================================================
-- Build product dimension from source tables
CREATE TABLE dim_products AS
WITH all_products AS (
    SELECT DISTINCT product_name, category, sub_category, unit_cost, unit_price
    FROM sales_data
    UNION ALL
    SELECT DISTINCT product_name, category, sub_category, unit_cost, unit_price
    FROM sales_history
)
SELECT 
    auto_id(product_name) AS product_key,
    product_name,
    category,
    sub_category,
    AVG(unit_cost) AS avg_unit_cost,
    AVG(unit_price) AS avg_unit_price,
    profit_margin(AVG(unit_price), AVG(unit_cost)) AS avg_profit_margin_pct
FROM all_products
GROUP BY product_name, category, sub_category;

-- ============================================================================
-- Step 4: Create Dimension Table - dim_time
-- ============================================================================
-- Build time dimension from source tables
CREATE TABLE dim_time AS
WITH all_dates AS (
    SELECT DISTINCT order_date FROM sales_data WHERE order_date IS NOT NULL
    UNION
    SELECT DISTINCT order_date FROM sales_history WHERE order_date IS NOT NULL
)
SELECT DISTINCT
    auto_id(order_date::VARCHAR) AS time_key,
    order_date AS date,
    YEAR(order_date) AS year,
    QUARTER(order_date) AS quarter,
    quarter_year(order_date) AS quarter_year,
    MONTH(order_date) AS month,
    MONTHNAME(order_date) AS month_name,
    DAY(order_date) AS day,
    DAYOFWEEK(order_date) AS day_of_week,
    DAYNAME(order_date) AS day_name,
    is_weekend(order_date) AS is_weekend,
    WEEKOFYEAR(order_date) AS week_of_year
FROM all_dates
ORDER BY order_date;

-- ============================================================================
-- Step 5: Keys Already Added!
-- ============================================================================
-- Foreign keys were created inline using the same auto_id() functions
-- No ALTER TABLE needed - keys are already in the fact table

-- ============================================================================
-- Step 6: Register Model Tables in Metadata
-- ============================================================================

-- fact_sales metadata
INSERT INTO _warphead_table_metadata (table_name, table_type, created_at, creation_query)
VALUES ('fact_sales', 'model', now(),
'-- Unified Fact Table: All Sales Transactions
-- Contains ONLY keys and measures (clean star schema)
WITH all_sales AS (
  SELECT 
    transaction_id, customer_name, product_name, order_date,
    ship_date, sales_person, order_status, payment_method,
    shipping_method, quantity, unit_price, unit_cost, discount_percent,
    ''current'' AS data_source
  FROM sales_data
  UNION ALL
  SELECT 
    transaction_id, customer_name, product_name, order_date,
    ship_date, sales_person, order_status, payment_method,
    shipping_method, quantity, unit_price, unit_cost, discount_percent,
    ''history'' AS data_source
  FROM sales_history
)
SELECT 
  transaction_id,
  auto_id(customer_name) AS customer_key,
  auto_id(product_name) AS product_key,
  auto_id(order_date::VARCHAR) AS time_key,
  ship_date,
  sales_person,
  order_status,
  payment_method,
  shipping_method,
  data_source,
  quantity,
  unit_price,
  unit_cost,
  discount_percent,
  revenue(quantity, unit_price) AS gross_revenue,
  revenue(quantity, unit_price) * (1 - discount_percent / 100) AS net_revenue,
  revenue(quantity, unit_cost) AS total_cost,
  revenue(quantity, unit_price) * (1 - discount_percent / 100) - revenue(quantity, unit_cost) AS profit,
  profit_margin(
    revenue(quantity, unit_price) * (1 - discount_percent / 100),
    revenue(quantity, unit_cost)
  ) AS profit_margin_pct,
  discount_price(unit_price, discount_percent) AS discounted_unit_price,
  CASE 
    WHEN order_status = ''Completed'' 
    THEN revenue(quantity, unit_price) * (1 - discount_percent / 100)
    ELSE 0 
  END AS completed_revenue
FROM all_sales')
ON CONFLICT (table_name) DO UPDATE SET
    table_type = EXCLUDED.table_type,
    creation_query = EXCLUDED.creation_query;

-- dim_customers metadata
INSERT INTO _warphead_table_metadata (table_name, table_type, created_at, creation_query)
VALUES ('dim_customers', 'model', now(), 
'-- Customer Dimension Table
WITH all_customers AS (
  SELECT DISTINCT customer_name, customer_segment, region, city, transaction_id, order_date
  FROM sales_data
  UNION ALL
  SELECT DISTINCT customer_name, customer_segment, region, city, transaction_id, order_date
  FROM sales_history
)
SELECT 
  auto_id(customer_name) AS customer_key,
  customer_name,
  customer_segment,
  region,
  city,
  COUNT(DISTINCT transaction_id) AS total_orders,
  MIN(order_date) AS first_order_date,
  MAX(order_date) AS last_order_date
FROM all_customers
GROUP BY customer_name, customer_segment, region, city')
ON CONFLICT (table_name) DO UPDATE SET
    table_type = EXCLUDED.table_type,
    creation_query = EXCLUDED.creation_query;

-- dim_products metadata
INSERT INTO _warphead_table_metadata (table_name, table_type, created_at, creation_query)
VALUES ('dim_products', 'model', now(),
'-- Product Dimension Table
WITH all_products AS (
  SELECT DISTINCT product_name, category, sub_category, unit_cost, unit_price
  FROM sales_data
  UNION ALL
  SELECT DISTINCT product_name, category, sub_category, unit_cost, unit_price
  FROM sales_history
)
SELECT 
  auto_id(product_name) AS product_key,
  product_name,
  category,
  sub_category,
  AVG(unit_cost) AS avg_unit_cost,
  AVG(unit_price) AS avg_unit_price,
  profit_margin(AVG(unit_price), AVG(unit_cost)) AS avg_profit_margin_pct
FROM all_products
GROUP BY product_name, category, sub_category')
ON CONFLICT (table_name) DO UPDATE SET
    table_type = EXCLUDED.table_type,
    creation_query = EXCLUDED.creation_query;

-- dim_time metadata
INSERT INTO _warphead_table_metadata (table_name, table_type, created_at, creation_query)
VALUES ('dim_time', 'model', now(),
'-- Time Dimension Table
WITH all_dates AS (
  SELECT DISTINCT order_date FROM sales_data WHERE order_date IS NOT NULL
  UNION
  SELECT DISTINCT order_date FROM sales_history WHERE order_date IS NOT NULL
)
SELECT DISTINCT
  auto_id(order_date::VARCHAR) AS time_key,
  order_date AS date,
  YEAR(order_date) AS year,
  QUARTER(order_date) AS quarter,
  quarter_year(order_date) AS quarter_year,
  MONTH(order_date) AS month,
  MONTHNAME(order_date) AS month_name,
  DAY(order_date) AS day,
  DAYOFWEEK(order_date) AS day_of_week,
  DAYNAME(order_date) AS day_name,
  is_weekend(order_date) AS is_weekend,
  WEEKOFYEAR(order_date) AS week_of_year
FROM all_dates
ORDER BY order_date')
ON CONFLICT (table_name) DO UPDATE SET
    table_type = EXCLUDED.table_type,
    creation_query = EXCLUDED.creation_query;

-- ============================================================================
-- Step 7: Dimensions for Analysis (Now Loaded from JSON)
-- ============================================================================
-- 
-- Dimensions are now automatically loaded from src/lib/on_init/dimensions.json
-- during database initialization, not from this SQL file.
-- 
-- This provides:
-- - Consistency with metrics and UDFs (all loaded from JSON)
-- - Easier maintenance and updates
-- - Version control of dimension definitions
-- - No need to modify SQL when adding/changing dimensions
--
-- See: initialize_builtin_dimensions() in src-tauri/src/commands/database.rs
-- ============================================================================

-- ============================================================================
-- Step 8: Add Relationships for Star Schema
-- ============================================================================

INSERT INTO _warphead_relationships (from_table, from_column, to_table, to_column, relationship_type, created_at)
VALUES 
    ('fact_sales', 'customer_key', 'dim_customers', 'customer_key', 'inferred', now()),
    ('fact_sales', 'product_key', 'dim_products', 'product_key', 'inferred', now()),
    ('fact_sales', 'time_key', 'dim_time', 'time_key', 'inferred', now())
ON CONFLICT (from_table, from_column, to_table, to_column) DO NOTHING;

-- ============================================================================
-- Verification Queries
-- ============================================================================

-- Check row counts
SELECT 'dim_customers' AS table_name, COUNT(*) AS row_count FROM dim_customers
UNION ALL
SELECT 'dim_products', COUNT(*) FROM dim_products
UNION ALL
SELECT 'dim_time', COUNT(*) FROM dim_time
UNION ALL
SELECT 'fact_sales', COUNT(*) FROM fact_sales
ORDER BY row_count DESC;

-- Sample star schema query
SELECT 
    c.customer_name,
    c.region,
    p.product_name,
    p.category,
    t.year,
    t.quarter_year,
    f.data_source,
    SUM(f.net_revenue) AS total_revenue,
    SUM(f.profit) AS total_profit,
    SUM(f.quantity) AS total_quantity
FROM fact_sales f
JOIN dim_customers c ON f.customer_key = c.customer_key
JOIN dim_products p ON f.product_key = p.product_key
JOIN dim_time t ON f.time_key = t.time_key
WHERE f.order_status = 'Completed'
GROUP BY c.customer_name, c.region, p.product_name, p.category, t.year, t.quarter_year, f.data_source
ORDER BY total_revenue DESC
LIMIT 10;
