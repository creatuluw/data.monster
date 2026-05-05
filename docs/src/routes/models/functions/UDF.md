# User-Defined Functions (UDFs) Guide

## Overview

User-Defined Functions (UDFs) in warphead.dev allow you to create reusable SQL logic using DuckDB's MACRO functionality. These functions are stored persistently in the database and automatically reloaded when the application starts.

## How UDFs Work

### Storage

All UDFs are stored in a special system table called `_warphead_udfs` with the following schema:

```sql
CREATE TABLE _warphead_udfs (
    function_name TEXT PRIMARY KEY,        -- Unique function identifier
    parameters TEXT NOT NULL,              -- Parameter definitions (e.g., "x INTEGER, y INTEGER")
    return_type TEXT NOT NULL,             -- Expected return type (e.g., "VARCHAR", "INTEGER")
    function_body TEXT NOT NULL,           -- SQL expression that defines the function
    description TEXT,                      -- Optional human-readable description
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- When the function was created
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP   -- When the function was last modified
);
```

### DuckDB Macros

Under the hood, UDFs are implemented as **DuckDB MACRO** functions. When you create a UDF:

1. The function metadata is saved to `_warphead_udfs` table
2. A DuckDB MACRO is created using `CREATE OR REPLACE MACRO` syntax
3. The MACRO persists for the database session

Example DuckDB command generated:
```sql
CREATE OR REPLACE MACRO full_name(first VARCHAR, last VARCHAR) AS 
    UPPER(first) || ' ' || UPPER(last);
```

### Automatic Reload

When the application initializes the DuckDB connection:
1. All functions are loaded from `_warphead_udfs` table
2. Each function is recreated as a MACRO
3. Functions are immediately available for use in queries

This happens in the `initialize_duckdb` function via the `reload_udfs_internal` helper.

## Creating Functions

### Via UI

1. Navigate to **Models → Functions**
2. Click **New Function**
3. Fill in the form:
   - **Function Name**: Unique identifier (letters, numbers, underscores only)
   - **Parameters**: Comma-separated list with types (e.g., `x INTEGER, y INTEGER`)
   - **Return Type**: Expected output type
   - **Function Body**: SQL expression that defines the logic
   - **Description**: Optional explanation of what the function does
4. Click **Create Function**

### Via API

Use the `create_udf` Tauri command:

```javascript
import { invoke } from '@tauri-apps/api/core';

await invoke('create_udf', {
    functionName: 'calculate_discount',
    parameters: 'price DOUBLE, discount_pct DOUBLE',
    returnType: 'DOUBLE',
    functionBody: 'price * (1 - discount_pct / 100.0)',
    description: 'Calculate final price after applying discount percentage'
});
```

## Example Functions

### String Manipulation

**Full Name Formatter**
```
Name: full_name
Parameters: first_name VARCHAR, last_name VARCHAR
Return Type: VARCHAR
Body: UPPER(first_name) || ' ' || UPPER(last_name)
Description: Formats first and last name into uppercase full name
```

**Email Domain Extractor**
```
Name: extract_domain
Parameters: email VARCHAR
Return Type: VARCHAR
Body: REGEXP_EXTRACT(email, '@(.+)$', 1)
Description: Extracts domain from email address
```

### Numeric Calculations

**Percentage Calculator**
```
Name: calc_percentage
Parameters: part DOUBLE, total DOUBLE
Return Type: DOUBLE
Body: ROUND((part / NULLIF(total, 0)) * 100, 2)
Description: Calculates percentage with 2 decimal places, handles division by zero
```

**Discount Price**
```
Name: apply_discount
Parameters: price DOUBLE, discount_rate DOUBLE
Return Type: DOUBLE
Body: price * (1 - discount_rate)
Description: Applies discount rate (0.0-1.0) to price
```

### Date Functions

**Days Since Date**
```
Name: days_since
Parameters: past_date DATE
Return Type: INTEGER
Body: DATE_DIFF('day', past_date, CURRENT_DATE)
Description: Calculate number of days from past date to today
```

**Quarter Year**
```
Name: quarter_year
Parameters: date_val DATE
Return Type: VARCHAR
Body: 'Q' || QUARTER(date_val) || ' ' || YEAR(date_val)
Description: Format date as "Q1 2024" style string
```

### Conditional Logic

**Status Color**
```
Name: status_color
Parameters: value DOUBLE, threshold DOUBLE
Return Type: VARCHAR
Body: CASE WHEN value >= threshold THEN 'green' WHEN value >= threshold * 0.5 THEN 'yellow' ELSE 'red' END
Description: Returns color based on threshold comparison
```

**Grade Calculator**
```
Name: calculate_grade
Parameters: score INTEGER
Return Type: VARCHAR
Body: CASE WHEN score >= 90 THEN 'A' WHEN score >= 80 THEN 'B' WHEN score >= 70 THEN 'C' WHEN score >= 60 THEN 'D' ELSE 'F' END
Description: Convert numeric score to letter grade
```

## Using Functions

Once created, functions can be used in any SQL query throughout the application:

### In Table Builder

```sql
SELECT 
  first_name,
  last_name,
  full_name(first_name, last_name) AS formatted_name,
  email,
  extract_domain(email) AS email_domain
FROM employees
LIMIT 100;
```

### In Metrics

```sql
SELECT 
  product_category,
  SUM(revenue) AS total_revenue,
  AVG(calc_percentage(revenue, total_sales)) AS avg_percentage
FROM sales
GROUP BY product_category;
```

### In Data Models

```sql
SELECT 
  order_date,
  quarter_year(order_date) AS quarter,
  days_since(order_date) AS days_old,
  order_amount,
  apply_discount(order_amount, 0.15) AS discounted_amount
FROM orders
WHERE days_since(order_date) <= 90;
```

## Testing Functions

The UI provides a built-in test panel for each function:

1. Click the **Play** icon on any function card
2. A test query is auto-generated based on the function signature
3. Modify the query to test different inputs
4. Click **Run Test** to see results
5. Results display in a table format

Example test queries:

```sql
-- Test string function
SELECT full_name('john', 'doe') AS result;

-- Test numeric function with multiple rows
SELECT 
  price,
  apply_discount(price, 0.10) AS discounted,
  apply_discount(price, 0.25) AS sale_price
FROM (VALUES (100.0), (250.0), (500.0)) AS t(price);

-- Test with actual table data
SELECT 
  employee_name,
  days_since(hire_date) AS days_employed
FROM employees
LIMIT 5;
```

## Managing Functions

### Editing Functions

1. Click the **Edit** icon on a function card
2. Modify parameters, body, or description
3. Click **Update Function**
4. Function is immediately recreated with new definition

**Note**: Function name cannot be changed when editing. To rename, delete and recreate.

### Deleting Functions

1. Click the **Trash** icon on a function card
2. Confirm deletion by clicking the checkmark
3. Function is removed from both `_warphead_udfs` table and DuckDB session

### Copying Function Bodies

Click the **Copy** icon on any function card to copy the function body to your clipboard for reuse.

## Backend Implementation

### Tauri Commands

**`create_udf`**
- Creates or updates a UDF
- Parameters: `function_name`, `parameters`, `return_type`, `function_body`, `description`
- Validates function name (alphanumeric + underscores only)
- Stores in `_warphead_udfs` table
- Creates DuckDB MACRO

**`list_udfs`**
- Returns all functions from `_warphead_udfs` table
- Sorted by function name
- Returns JSON array of function objects

**`delete_udf`**
- Removes function from database
- Parameters: `function_name`
- Drops MACRO with `DROP MACRO IF EXISTS`
- Deletes from `_warphead_udfs` table

**`test_udf`**
- Executes a test query using the function
- Parameters: `function_name`, `test_query`
- Returns query results as JSON
- Catches and reports errors

**`reload_udfs`**
- Reloads all functions from `_warphead_udfs` table
- Recreates all MACROs in current session
- Called automatically during database initialization

### Helper Functions

**`reload_udfs_internal`**
- Internal helper that takes a `Connection` reference
- Used during database initialization
- Gracefully handles missing `_warphead_udfs` table

## Best Practices

### Naming Conventions

- Use descriptive names: `calculate_discount` not `cd`
- Use snake_case: `full_name` not `fullName`
- Prefix related functions: `date_to_quarter`, `date_to_fiscal_year`
- Avoid reserved SQL keywords

### Parameter Types

Common DuckDB types for parameters:
- `VARCHAR` / `TEXT` - String data
- `INTEGER` / `BIGINT` - Whole numbers
- `DOUBLE` / `FLOAT` - Decimal numbers
- `BOOLEAN` - True/false values
- `DATE` - Date values
- `TIMESTAMP` - Date and time values

### Function Body Guidelines

1. **Keep it simple**: Functions should do one thing well
2. **Handle nulls**: Use `COALESCE()` or `NULLIF()` for null safety
3. **Avoid division by zero**: Use `NULLIF(denominator, 0)`
4. **Use CASE for logic**: Prefer CASE WHEN over nested IFs
5. **Test thoroughly**: Use the test panel to verify edge cases

### Documentation

Always add a description explaining:
- What the function does
- Expected input format/range
- What the function returns
- Any special considerations (null handling, edge cases)

## Advanced Usage

### Nested Functions

Functions can call other functions:

```sql
-- Base function
CREATE MACRO clean_text(text VARCHAR) AS TRIM(LOWER(text));

-- Function using clean_text
CREATE MACRO match_text(text1 VARCHAR, text2 VARCHAR) AS 
    clean_text(text1) = clean_text(text2);
```

### Using Subqueries

Functions can include subquery expressions:

```sql
CREATE MACRO customer_lifetime_value(customer_id INTEGER) AS (
    SELECT SUM(order_total) 
    FROM orders 
    WHERE customer_id = customer_id
);
```

### Type Casting

Explicitly cast types when needed:

```sql
CREATE MACRO to_percentage_string(value DOUBLE) AS 
    CAST(ROUND(value * 100, 1) AS VARCHAR) || '%';
```

## Limitations

1. **DuckDB MACRO limitations apply**:
   - No recursive functions
   - No CREATE/DROP statements inside functions
   - Limited to single expression (no multi-statement)

2. **Session-based**: 
   - MACROs exist only for current database session
   - Automatically reloaded on app restart via metadata table

3. **Name conflicts**:
   - Cannot override built-in DuckDB functions
   - Function names must be unique

4. **No modification tracking**:
   - If MACRO is manually dropped in SQL, metadata remains
   - Reload functions to restore consistency

## Troubleshooting

### Function not found error

**Cause**: MACRO was dropped or database was reset
**Solution**: Click "New Function" → Save the function again, or restart the app to trigger reload

### Syntax error when creating function

**Cause**: Invalid SQL expression in function body
**Solution**: 
- Test the expression in a regular SELECT query first
- Check parameter names match what's used in body
- Verify all quotes and parentheses are balanced

### Function returns unexpected results

**Cause**: Logic error or null handling issue
**Solution**:
- Use the Test panel to debug with sample inputs
- Add COALESCE or NULLIF for null safety
- Check operator precedence (use parentheses)

### Cannot edit function name

**Cause**: Function names are immutable (used as primary key)
**Solution**: Delete the old function and create a new one with the desired name

## Performance Considerations

1. **MACROs are expanded inline**: DuckDB replaces function calls with the body expression at query time
2. **No performance penalty**: MACROs don't add function call overhead
3. **Optimize the body**: The function body should be efficient SQL
4. **Avoid expensive operations**: Subqueries in functions can impact performance on large datasets

## Integration with Data Models

UDFs integrate seamlessly with all modeling features:

- **Tables**: Use in SELECT, WHERE, ORDER BY clauses
- **Metrics**: Reference in aggregation calculations
- **Dimensions**: Apply for categorization logic
- **Relationships**: Use in JOIN conditions
- **SQL Models**: Call from any custom SQL transformation

## Future Enhancements

Potential improvements to the UDF system:

- [ ] Function categories/tagging
- [ ] Import/export function libraries
- [ ] Version history for functions
- [ ] Function usage analytics
- [ ] Built-in function templates
- [ ] Syntax highlighting in editor
- [ ] Auto-complete for function names
- [ ] Function dependencies graph
- [ ] Performance profiling for functions

## Resources

- [DuckDB MACRO Documentation](https://duckdb.org/docs/sql/statements/create_macro.html)
- [DuckDB SQL Functions](https://duckdb.org/docs/sql/functions/overview)
- [SQL Expression Syntax](https://duckdb.org/docs/sql/expressions/overview)

---

**Need help?** Check the test panel for debugging, or review the example functions above for common patterns.

