#!/usr/bin/env python3
"""
Generate comprehensive sales history data with 1,234,567 records
This script creates realistic historical sales transaction data for testing performance
with large datasets and for long-term trend analysis.
"""

import csv
import random
from datetime import datetime, timedelta
import sys

# Configuration
OUTPUT_PATH = "src/lib/on_init/sales_history.csv"
NUM_RECORDS = 1_234_567
BATCH_SIZE = 10_000  # Write in batches to manage memory

# Define data arrays for realistic generation
products = [
    {"name": "Laptop Pro 15", "category": "Electronics", "sub_category": "Computers", "cost": 800, "price": 1299},
    {"name": "Wireless Mouse", "category": "Electronics", "sub_category": "Accessories", "cost": 15, "price": 29.99},
    {"name": "USB-C Hub", "category": "Electronics", "sub_category": "Accessories", "cost": 20, "price": 49.99},
    {"name": "Office Chair Executive", "category": "Furniture", "sub_category": "Seating", "cost": 150, "price": 299.99},
    {"name": "Standing Desk", "category": "Furniture", "sub_category": "Desks", "cost": 200, "price": 449.99},
    {"name": "Monitor 27inch 4K", "category": "Electronics", "sub_category": "Displays", "cost": 250, "price": 499.99},
    {"name": "Keyboard Mechanical RGB", "category": "Electronics", "sub_category": "Accessories", "cost": 50, "price": 129.99},
    {"name": "Webcam HD Pro", "category": "Electronics", "sub_category": "Accessories", "cost": 40, "price": 89.99},
    {"name": "Desk Lamp LED", "category": "Furniture", "sub_category": "Lighting", "cost": 18, "price": 39.99},
    {"name": "Notebook A4 Pack", "category": "Office Supplies", "sub_category": "Paper", "cost": 5, "price": 12.99},
    {"name": "Pen Set Premium", "category": "Office Supplies", "sub_category": "Writing", "cost": 8, "price": 19.99},
    {"name": "File Cabinet Metal", "category": "Furniture", "sub_category": "Storage", "cost": 120, "price": 249.99},
    {"name": "Printer All-in-One", "category": "Electronics", "sub_category": "Printers", "cost": 180, "price": 349.99},
    {"name": "Tablet 10inch", "category": "Electronics", "sub_category": "Computers", "cost": 300, "price": 599.99},
    {"name": "Headphones Wireless", "category": "Electronics", "sub_category": "Audio", "cost": 60, "price": 149.99},
    {"name": "Desk Organizer Wood", "category": "Furniture", "sub_category": "Organization", "cost": 12, "price": 29.99},
    {"name": "Whiteboard Large", "category": "Office Supplies", "sub_category": "Boards", "cost": 40, "price": 89.99},
    {"name": "Ergonomic Footrest", "category": "Furniture", "sub_category": "Accessories", "cost": 25, "price": 49.99},
    {"name": "Cable Management Kit", "category": "Electronics", "sub_category": "Accessories", "cost": 10, "price": 24.99},
    {"name": "Bookshelf 5-Tier", "category": "Furniture", "sub_category": "Storage", "cost": 80, "price": 179.99}
]

customers = [
    {"name": "Global Tech Solutions", "segment": "Enterprise", "region": "North America", "city": "New York"},
    {"name": "SmallBiz Inc", "segment": "Small Business", "region": "North America", "city": "Chicago"},
    {"name": "Acme Corp", "segment": "Corporate", "region": "North America", "city": "Los Angeles"},
    {"name": "TechStart LLC", "segment": "Small Business", "region": "North America", "city": "Austin"},
    {"name": "Mega Industries", "segment": "Enterprise", "region": "Europe", "city": "London"},
    {"name": "Euro Office GmbH", "segment": "Corporate", "region": "Europe", "city": "Berlin"},
    {"name": "UK Small Co", "segment": "Small Business", "region": "Europe", "city": "Manchester"},
    {"name": "Nordic Solutions", "segment": "Corporate", "region": "Europe", "city": "Stockholm"},
    {"name": "Asia Pacific Ltd", "segment": "Enterprise", "region": "Asia Pacific", "city": "Singapore"},
    {"name": "Tokyo Business", "segment": "Corporate", "region": "Asia Pacific", "city": "Tokyo"},
    {"name": "Sydney Enterprises", "segment": "Small Business", "region": "Asia Pacific", "city": "Sydney"},
    {"name": "Shanghai Corp", "segment": "Enterprise", "region": "Asia Pacific", "city": "Shanghai"},
    {"name": "Latin Trade SA", "segment": "Corporate", "region": "Latin America", "city": "Mexico City"},
    {"name": "Brasil Office", "segment": "Small Business", "region": "Latin America", "city": "Sao Paulo"},
    {"name": "Argentina Biz", "segment": "Small Business", "region": "Latin America", "city": "Buenos Aires"}
]

sales_people = [
    "Sarah Johnson", "Michael Chen", "Emma Williams", "David Rodriguez",
    "Lisa Anderson", "James Thompson", "Maria Garcia", "Robert Kim",
    "Jennifer Martinez", "William Brown", "Amanda Davis", "Christopher Lee"
]

statuses = ["Completed"] * 6 + ["Shipped", "Pending", "Cancelled"]
payment_methods = ["Credit Card"] * 3 + ["Bank Transfer", "PayPal", "Invoice"]
ship_methods = ["Standard"] * 2 + ["Express", "Overnight", "Economy"]

# Date range: 5 years of historical data
start_date = datetime(2019, 1, 1)
end_date = datetime(2024, 10, 28)
day_span = (end_date - start_date).days

print(f"Generating {NUM_RECORDS:,} sales history records...")
print(f"Date range: {start_date.strftime('%Y-%m-%d')} to {end_date.strftime('%Y-%m-%d')}")
print(f"Writing to: {OUTPUT_PATH}")
print()

# Open file and write header
with open(OUTPUT_PATH, 'w', newline='', encoding='utf-8') as f:
    writer = csv.writer(f)
    
    # Write header
    writer.writerow([
        "transaction_id", "order_date", "ship_date", "customer_name",
        "customer_segment", "region", "city", "sales_person", "product_name",
        "category", "sub_category", "quantity", "unit_price", "unit_cost",
        "discount_percent", "order_status", "payment_method", "shipping_method"
    ])
    
    # Generate and write data in batches
    batch = []
    for i in range(1, NUM_RECORDS + 1):
        # Generate random order date
        order_date = start_date + timedelta(days=random.randint(0, day_span))
        order_date_str = order_date.strftime("%Y-%m-%d")
        
        # Ship date is 1-7 days after order (or empty for pending/cancelled)
        status = random.choice(statuses)
        ship_date_str = ""
        if status in ["Completed", "Shipped"]:
            ship_date = order_date + timedelta(days=random.randint(1, 8))
            ship_date_str = ship_date.strftime("%Y-%m-%d")
        
        # Select random data
        customer = random.choice(customers)
        product = random.choice(products)
        sales_person = random.choice(sales_people)
        
        # Quantity: mostly 1-5, occasionally higher
        quantity = random.randint(1, 6) if random.random() < 0.8 else random.randint(6, 21)
        
        # Discount: 0-30% (weighted toward lower discounts)
        discount_percent = round(random.random() * 30 * (0.5 if random.random() < 0.7 else 1.0), 2)
        
        # Payment and shipping
        payment_method = random.choice(payment_methods)
        ship_method = random.choice(ship_methods)
        
        # Create record
        record = [
            i,  # transaction_id
            order_date_str,
            ship_date_str,
            customer["name"],
            customer["segment"],
            customer["region"],
            customer["city"],
            sales_person,
            product["name"],
            product["category"],
            product["sub_category"],
            quantity,
            product["price"],
            product["cost"],
            discount_percent,
            status,
            payment_method,
            ship_method
        ]
        batch.append(record)
        
        # Write batch to file
        if len(batch) >= BATCH_SIZE:
            writer.writerows(batch)
            batch = []
            print(f"  Generated {i:,} records... ({i/NUM_RECORDS*100:.1f}%)", end='\r')
    
    # Write remaining records
    if batch:
        writer.writerows(batch)
    
    print()  # New line after progress

print(f"\n✓ Successfully generated {OUTPUT_PATH} with {NUM_RECORDS:,} records!")

# Calculate and display file size
import os
file_size = os.path.getsize(OUTPUT_PATH)
file_size_mb = file_size / (1024 * 1024)
print(f"  File size: {file_size_mb:.2f} MB ({file_size:,} bytes)")
print(f"  Average record size: {file_size/NUM_RECORDS:.0f} bytes")

