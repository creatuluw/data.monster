# Generate comprehensive sales data with 973 records
# This script creates realistic sales transaction data for demo/init purposes

$outputPath = "src/lib/on_init/sales_data.csv"

# Define data arrays for realistic generation
$products = @(
    @{Name="Laptop Pro 15"; Category="Electronics"; SubCategory="Computers"; Cost=800; Price=1299},
    @{Name="Wireless Mouse"; Category="Electronics"; SubCategory="Accessories"; Cost=15; Price=29.99},
    @{Name="USB-C Hub"; Category="Electronics"; SubCategory="Accessories"; Cost=20; Price=49.99},
    @{Name="Office Chair Executive"; Category="Furniture"; SubCategory="Seating"; Cost=150; Price=299.99},
    @{Name="Standing Desk"; Category="Furniture"; SubCategory="Desks"; Cost=200; Price=449.99},
    @{Name="Monitor 27inch 4K"; Category="Electronics"; SubCategory="Displays"; Cost=250; Price=499.99},
    @{Name="Keyboard Mechanical RGB"; Category="Electronics"; SubCategory="Accessories"; Cost=50; Price=129.99},
    @{Name="Webcam HD Pro"; Category="Electronics"; SubCategory="Accessories"; Cost=40; Price=89.99},
    @{Name="Desk Lamp LED"; Category="Furniture"; SubCategory="Lighting"; Cost=18; Price=39.99},
    @{Name="Notebook A4 Pack"; Category="Office Supplies"; SubCategory="Paper"; Cost=5; Price=12.99},
    @{Name="Pen Set Premium"; Category="Office Supplies"; SubCategory="Writing"; Cost=8; Price=19.99},
    @{Name="File Cabinet Metal"; Category="Furniture"; SubCategory="Storage"; Cost=120; Price=249.99},
    @{Name="Printer All-in-One"; Category="Electronics"; SubCategory="Printers"; Cost=180; Price=349.99},
    @{Name="Tablet 10inch"; Category="Electronics"; SubCategory="Computers"; Cost=300; Price=599.99},
    @{Name="Headphones Wireless"; Category="Electronics"; SubCategory="Audio"; Cost=60; Price=149.99},
    @{Name="Desk Organizer Wood"; Category="Furniture"; SubCategory="Organization"; Cost=12; Price=29.99},
    @{Name="Whiteboard Large"; Category="Office Supplies"; SubCategory="Boards"; Cost=40; Price=89.99},
    @{Name="Ergonomic Footrest"; Category="Furniture"; SubCategory="Accessories"; Cost=25; Price=49.99},
    @{Name="Cable Management Kit"; Category="Electronics"; SubCategory="Accessories"; Cost=10; Price=24.99},
    @{Name="Bookshelf 5-Tier"; Category="Furniture"; SubCategory="Storage"; Cost=80; Price=179.99}
)

$customers = @(
    @{Name="Global Tech Solutions"; Segment="Enterprise"; Region="North America"; City="New York"},
    @{Name="SmallBiz Inc"; Segment="Small Business"; Region="North America"; City="Chicago"},
    @{Name="Acme Corp"; Segment="Corporate"; Region="North America"; City="Los Angeles"},
    @{Name="TechStart LLC"; Segment="Small Business"; Region="North America"; City="Austin"},
    @{Name="Mega Industries"; Segment="Enterprise"; Region="Europe"; City="London"},
    @{Name="Euro Office GmbH"; Segment="Corporate"; Region="Europe"; City="Berlin"},
    @{Name="UK Small Co"; Segment="Small Business"; Region="Europe"; City="Manchester"},
    @{Name="Nordic Solutions"; Segment="Corporate"; Region="Europe"; City="Stockholm"},
    @{Name="Asia Pacific Ltd"; Segment="Enterprise"; Region="Asia Pacific"; City="Singapore"},
    @{Name="Tokyo Business"; Segment="Corporate"; Region="Asia Pacific"; City="Tokyo"},
    @{Name="Sydney Enterprises"; Segment="Small Business"; Region="Asia Pacific"; City="Sydney"},
    @{Name="Shanghai Corp"; Segment="Enterprise"; Region="Asia Pacific"; City="Shanghai"},
    @{Name="Latin Trade SA"; Segment="Corporate"; Region="Latin America"; City="Mexico City"},
    @{Name="Brasil Office"; Segment="Small Business"; Region="Latin America"; City="Sao Paulo"},
    @{Name="Argentina Biz"; Segment="Small Business"; Region="Latin America"; City="Buenos Aires"}
)

$salesPeople = @(
    "Sarah Johnson", "Michael Chen", "Emma Williams", "David Rodriguez",
    "Lisa Anderson", "James Thompson", "Maria Garcia", "Robert Kim",
    "Jennifer Martinez", "William Brown", "Amanda Davis", "Christopher Lee"
)

$statuses = @("Completed", "Completed", "Completed", "Completed", "Completed", "Completed", "Shipped", "Pending", "Cancelled")
$paymentMethods = @("Credit Card", "Credit Card", "Credit Card", "Bank Transfer", "PayPal", "Invoice")
$shipMethods = @("Standard", "Standard", "Express", "Overnight", "Economy")

# Generate header
$csv = "transaction_id,order_date,ship_date,customer_name,customer_segment,region,city,sales_person,product_name,category,sub_category,quantity,unit_price,unit_cost,discount_percent,order_status,payment_method,shipping_method`n"

# Generate 973 records
$random = New-Object System.Random
$startDate = Get-Date "2023-01-01"
$endDate = Get-Date "2024-10-28"
$daySpan = ($endDate - $startDate).Days

for ($i = 1; $i -le 973; $i++) {
    # Generate random order date
    $orderDate = $startDate.AddDays($random.Next(0, $daySpan))
    $orderDateStr = $orderDate.ToString("yyyy-MM-dd")
    
    # Ship date is 1-7 days after order (or null for pending/cancelled)
    $status = $statuses[$random.Next(0, $statuses.Length)]
    $shipDateStr = ""
    if ($status -eq "Completed" -or $status -eq "Shipped") {
        $shipDate = $orderDate.AddDays($random.Next(1, 8))
        $shipDateStr = $shipDate.ToString("yyyy-MM-dd")
    }
    
    # Select random customer
    $customer = $customers[$random.Next(0, $customers.Length)]
    
    # Select random product
    $product = $products[$random.Next(0, $products.Length)]
    
    # Select random salesperson
    $salesPerson = $salesPeople[$random.Next(0, $salesPeople.Length)]
    
    # Quantity: mostly 1-5, occasionally higher
    $quantity = if ($random.Next(0, 10) -lt 8) {
        $random.Next(1, 6)
    } else {
        $random.Next(6, 21)
    }
    
    # Discount: 0-30% (weighted toward lower discounts)
    $discountPercent = $random.Next(0, 31) * (if ($random.Next(0, 10) -lt 7) { 0.5 } else { 1.0 })
    $discountPercent = [Math]::Round($discountPercent, 2)
    
    # Payment and shipping
    $paymentMethod = $paymentMethods[$random.Next(0, $paymentMethods.Length)]
    $shipMethod = $shipMethods[$random.Next(0, $shipMethods.Length)]
    
    # Format CSV line
    $line = "$i,$orderDateStr,$shipDateStr,`"$($customer.Name)`",$($customer.Segment),$($customer.Region),$($customer.City),`"$salesPerson`",`"$($product.Name)`",$($product.Category),$($product.SubCategory),$quantity,$($product.Price),$($product.Cost),$discountPercent,$status,$paymentMethod,$shipMethod`n"
    
    $csv += $line
    
    # Progress indicator
    if ($i % 100 -eq 0) {
        Write-Host "Generated $i records..."
    }
}

# Write to file
$csv | Out-File -FilePath $outputPath -Encoding UTF8 -NoNewline

Write-Host "`n✓ Successfully generated $outputPath with 973 records!"
$fileSize = [Math]::Round((Get-Item $outputPath).Length / 1KB, 2)
Write-Host "  File size: $fileSize KB"

