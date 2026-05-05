# Library User-Defined Functions (UDFs)

This directory contains the library UDFs that ship with warphead.dev. These functions are automatically loaded into DuckDB when the application initializes.

## 📁 Files

- **`library-functions.json`** - JSON definition of all library UDFs that ship with the app

## 🔧 How It Works

1. **JSON Definition**: All library functions are defined in `library-functions.json`
2. **Runtime Loading (Development)**: When developing, the backend reads the JSON file from disk on each app init
3. **Embedded Fallback (Production)**: In production builds, the JSON is embedded using `include_str!()` macro
4. **Auto-loaded on Init**: When DuckDB initializes, the backend:
   - Parses the JSON file (from disk if available, otherwise from embedded version)
   - Creates the `_warphead_udfs` table if it doesn't exist
   - Inserts/updates each function using `ON CONFLICT DO UPDATE` (updates library functions when JSON changes)
   - Creates all functions as DuckDB MACROs
   - Reports how many functions were created vs updated

## 📝 JSON Schema

Each function in the JSON file has the following structure:

```json
{
  "function_name": "safe_divide",
  "parameters": "numerator DOUBLE, denominator DOUBLE",
  "return_type": "DOUBLE",
  "function_body": "CASE WHEN denominator = 0 THEN 0 ELSE numerator / denominator END",
  "description": "Prevents division by zero errors...",
  "category": "math",
  "examples": [
    "safe_divide(100, 5) → 20.0",
    "safe_divide(100, 0) → 0.0"
  ]
}
```

### Required Fields
- `function_name` - Unique identifier for the function
- `parameters` - Parameter list with types (e.g., `"x INTEGER, y VARCHAR"`)
- `return_type` - DuckDB return type (e.g., `VARCHAR`, `INTEGER`, `DOUBLE`)
- `function_body` - SQL expression that defines the function logic
- `description` - Human-readable description of what the function does

### Optional Fields
- `category` - Grouping category (e.g., `math`, `string`, `date`, `business`)
- `examples` - Array of usage examples showing input → output

## 📚 Current Categories

- **math** - Mathematical operations (`safe_divide`, `clamp`, `safe_percentage`)
- **business** - Business calculations (`revenue`, `profit_margin`, `discount_price`)
- **string** - String manipulation (`full_name`, `clean_text`, `initials`)
- **date** - Date operations (`quarter_year`, `days_since`, `is_weekend`)
- **hash** - Hashing functions (`autonumber`, `auto_id`)
- **categorization** - Data categorization (`age_group`, `status_label`)
- **formatting** - Output formatting (`format_percentage`, `format_currency`)

## ➕ Adding New Library Functions

To add a new library function:

1. **Edit `library-functions.json`**
   ```json
   {
     "function_name": "my_function",
     "parameters": "param1 VARCHAR",
     "return_type": "VARCHAR",
     "function_body": "UPPER(param1)",
     "description": "Converts text to uppercase",
     "category": "string",
     "examples": [
       "my_function('hello') → 'HELLO'"
     ]
   }
   ```

2. **Restart the app**
   - In development: Just restart the app - changes will be loaded automatically
   - In production: Rebuild the Tauri app to embed the new JSON
   - The function will be auto-loaded on next app start
   - Use the Functions page in the UI to test it

3. **Considerations**
   - Function names must be unique
   - Changes to library functions will be applied on app restart (using `ON CONFLICT DO UPDATE`)
   - Keep function bodies simple and efficient
   - Provide clear descriptions and examples

## 🔄 Library Function Updates

The system now updates library functions automatically:
- When the app initializes, it reads `library-functions.json`
- If a library function has changed, it will be updated in the database (using `ON CONFLICT DO UPDATE`)
- The DuckDB MACROs are recreated with the new definitions
- In development, simply restart the app to pick up changes to the JSON file
- In production builds, the JSON is embedded at compile time

**Note:** This replaces the old behavior where library functions were never overwritten. Now library functions will always sync with the JSON file on app initialization.

## 🚀 Special Functions

### autonumber(input_text VARCHAR) → BIGINT
Generates consistent positive numeric IDs from strings, similar to Qlik's `autonumber()`. The same input always produces the same number.

**Usage:**
```sql
SELECT autonumber('customer_123') as id;
-- Always returns the same numeric ID for 'customer_123'
```

### auto_id(input_text VARCHAR) → VARCHAR
Generates consistent 8-character hexadecimal IDs from strings. The same input always produces the same 8-character ID.

**Usage:**
```sql
SELECT auto_id('customer_123') as short_id;
-- Returns something like: '3e4f5a6b'
-- Always the same for 'customer_123'
```

## 🐛 Troubleshooting

### Functions not showing up
- Restart the app to reload the JSON file
- Check the console for error messages about JSON parsing
- Verify JSON syntax is valid using a JSON validator
- Make sure the file path `src/lib/udfs/library-functions.json` exists

### Function syntax errors
- Test the function body in SQL Studio first
- Ensure parameter names match what's used in the body
- Check that all quotes and parentheses are balanced

### Changes to library functions not appearing
- In development: Restart the app - it reads the JSON file on each initialization
- In production: Rebuild the Tauri app to embed the updated JSON
- Check the console output for "Loaded X library functions from JSON (Y created, Z updated)"

## 📖 Related Documentation

- [UDF.md](../../routes/models/functions/UDF.md) - Complete UDF system documentation
- [BUILTIN-VS-CUSTOM-FUNCTIONS.md](../../../docs/BUILTIN-VS-CUSTOM-FUNCTIONS.md) - Built-in vs custom functions guide

## 🎯 Best Practices

1. **Keep it simple** - Functions should do one thing well
2. **Handle edge cases** - Use CASE statements for null/zero handling
3. **Document thoroughly** - Good descriptions and examples are essential
4. **Test extensively** - Use the test panel before adding to library
5. **Categorize properly** - Use consistent categories for organization
6. **Name clearly** - Use descriptive snake_case names

---

**Need help?** Check the Functions page in the app for testing tools and examples!

