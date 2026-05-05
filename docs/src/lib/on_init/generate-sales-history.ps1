# Generate comprehensive sales history data with 1,234,567 records
# This script creates realistic historical sales transaction data for testing performance
# with large datasets and for long-term trend analysis.

$OutputPath = "src/lib/on_init/sales_history.csv"
$NumRecords = 1234567
$BatchSize = 10000

Write-Host "Generating $($NumRecords.ToString('N0')) sales history records..." -ForegroundColor Cyan
Write-Host "Date range: 2019-01-01 to 2024-10-28"
Write-Host "Writing to: $OutputPath"
Write-Host ""

# Define data arrays
$products = @(
    @{name="Laptop Pro 15"; category="Electronics"; sub_category="Computers"; cost=800; price=1299},
    @{name="Wireless Mouse"; category="Electronics"; sub_category="Accessories"; cost=15; price=29.99},
    @{name="USB-C Hub"; category="Electronics"; sub_category="Accessories"; cost=20; price=49.99},
    @{name="Office Chair Executive"; category="Furniture"; sub_category="Seating"; cost=150; price=299.99},
    @{name="Standing Desk"; category="Furniture"; sub_category="Desks"; cost=200; price=449.99},
    @{name="Monitor 27inch 4K"; category="Electronics"; sub_category="Displays"; cost=250; price=499.99},
    @{name="Keyboard Mechanical RGB"; category="Electronics"; sub_category="Accessories"; cost=50; price=129.99},
    @{name="Webcam HD Pro"; category="Electronics"; sub_category="Accessories"; cost=40; price=89.99},
    @{name="Desk Lamp LED"; category="Furniture"; sub_category="Lighting"; cost=18; price=39.99},
    @{name="Notebook A4 Pack"; category="Office Supplies"; sub_category="Paper"; cost=5; price=12.99},
    @{name="Pen Set Premium"; category="Office Supplies"; sub_category="Writing"; cost=8; price=19.99},
    @{name="File Cabinet Metal"; category="Furniture"; sub_category="Storage"; cost=120; price=249.99},
    @{name="Printer All-in-One"; category="Electronics"; sub_category="Printers"; cost=180; price=349.99},
    @{name="Tablet 10inch"; category="Electronics"; sub_category="Computers"; cost=300; price=599.99},
    @{name="Headphones Wireless"; category="Electronics"; sub_category="Audio"; cost=60; price=149.99},
    @{name="Desk Organizer Wood"; category="Furniture"; sub_category="Organization"; cost=12; price=29.99},
    @{name="Whiteboard Large"; category="Office Supplies"; sub_category="Boards"; cost=40; price=89.99},
    @{name="Ergonomic Footrest"; category="Furniture"; sub_category="Accessories"; cost=25; price=49.99},
    @{name="Cable Management Kit"; category="Electronics"; sub_category="Accessories"; cost=10; price=24.99},
    @{name="Bookshelf 5-Tier"; category="Furniture"; sub_category="Storage"; cost=80; price=179.99}
)

$customers = @(
    @{name="Global Tech Solutions"; segment="Enterprise"; region="North America"; city="New York"},
    @{name="SmallBiz Inc"; segment="Small Business"; region="North America"; city="Chicago"},
    @{name="Acme Corp"; segment="Corporate"; region="North America"; city="Los Angeles"},
    @{name="TechStart LLC"; segment="Small Business"; region="North America"; city="Austin"},
    @{name="Mega Industries"; segment="Enterprise"; region="Europe"; city="London"},
    @{name="Euro Office GmbH"; segment="Corporate"; region="Europe"; city="Berlin"},
    @{name="UK Small Co"; segment="Small Business"; region="Europe"; city="Manchester"},
    @{name="Nordic Solutions"; segment="Corporate"; region="Europe"; city="Stockholm"},
    @{name="Asia Pacific Ltd"; segment="Enterprise"; region="Asia Pacific"; city="Singapore"},
    @{name="Tokyo Business"; segment="Corporate"; region="Asia Pacific"; city="Tokyo"},
    @{name="Sydney Enterprises"; segment="Small Business"; region="Asia Pacific"; city="Sydney"},
    @{name="Shanghai Corp"; segment="Enterprise"; region="Asia Pacific"; city="Shanghai"},
    @{name="Latin Trade SA"; segment="Corporate"; region="Latin America"; city="Mexico City"},
    @{name="Brasil Office"; segment="Small Business"; region="Latin America"; city="Sao Paulo"},
    @{name="Argentina Biz"; segment="Small Business"; region="Latin America"; city="Buenos Aires"}
)

$salesPeople = @(
    "Sarah Johnson", "Michael Chen", "Emma Williams", "David Rodriguez",
    "Lisa Anderson", "James Thompson", "Maria Garcia", "Robert Kim",
    "Jennifer Martinez", "William Brown", "Amanda Davis", "Christopher Lee"
)

$statuses = @("Completed","Completed","Completed","Completed","Completed","Completed","Shipped","Pending","Cancelled")
$paymentMethods = @("Credit Card","Credit Card","Credit Card","Bank Transfer","PayPal","Invoice")
$shipMethods = @("Standard","Standard","Express","Overnight","Economy")

# Date range
$startDate = Get-Date "2019-01-01"
$endDate = Get-Date "2024-10-28"
$daySpan = ($endDate - $startDate).Days

# Create CSV file
$header = "transaction_id,order_date,ship_date,customer_name,customer_segment,region,city,sales_person,product_name,category,sub_category,quantity,unit_price,unit_cost,discount_percent,order_status,payment_method,shipping_method"
Set-Content -Path $OutputPath -Value $header

# Generate records in batches
$batch = @()
for ($i = 1; $i -le $NumRecords; $i++) {
    # Random order date
    $orderDate = $startDate.AddDays((Get-Random -Maximum $daySpan))
    $orderDateStr = $orderDate.ToString("yyyy-MM-dd")
    
    # Status and ship date
    $status = $statuses | Get-Random
    $shipDateStr = ""
    if ($status -in @("Completed", "Shipped")) {
        $shipDate = $orderDate.AddDays((Get-Random -Minimum 1 -Maximum 9))
        $shipDateStr = $shipDate.ToString("yyyy-MM-dd")
    }
    
    # Random selections
    $customer = $customers | Get-Random
    $product = $products | Get-Random
    $salesPerson = $salesPeople | Get-Random
    
    # Quantity
    $quantity = if ((Get-Random -Maximum 100) -lt 80) { 
        Get-Random -Minimum 1 -Maximum 7 
    } else { 
        Get-Random -Minimum 6 -Maximum 22 
    }
    
    # Discount
    $discountBase = (Get-Random) / [int]::MaxValue * 30
    $discountPercent = if ((Get-Random -Maximum 100) -lt 70) { 
        [Math]::Round($discountBase * 0.5, 2) 
    } else { 
        [Math]::Round($discountBase, 2) 
    }
    
    # Payment and shipping
    $paymentMethod = $paymentMethods | Get-Random
    $shipMethod = $shipMethods | Get-Random
    
    # Build CSV line
    $line = "$i,$orderDateStr,$shipDateStr,$($customer.name),$($customer.segment),$($customer.region),$($customer.city),$salesPerson,$($product.name),$($product.category),$($product.sub_category),$quantity,$($product.price),$($product.cost),$discountPercent,$status,$paymentMethod,$shipMethod"
    $batch += $line
    
    # Write batch
    if ($batch.Count -ge $BatchSize) {
        Add-Content -Path $OutputPath -Value $batch
        $batch = @()
        $percent = [Math]::Round(($i / $NumRecords) * 100, 1)
        Write-Host -NoNewline "`r  Generated $($i.ToString('N0')) records... ($percent%)"
    }
}

# Write remaining
if ($batch.Count -gt 0) {
    Add-Content -Path $OutputPath -Value $batch
}

Write-Host ""
Write-Host ""
Write-Host "✓ Successfully generated $OutputPath with $($NumRecords.ToString('N0')) records!" -ForegroundColor Green

# File size
$fileInfo = Get-Item $OutputPath
$fileSizeMB = [Math]::Round($fileInfo.Length / 1MB, 2)
Write-Host "  File size: $fileSizeMB MB ($($fileInfo.Length.ToString('N0')) bytes)"
$avgRecordSize = [Math]::Round($fileInfo.Length / $NumRecords, 0)
Write-Host "  Average record size: $avgRecordSize bytes"

