# Smart Slaughterhouse Management System

A comprehensive Internet Computer (IC) canister-based system for managing slaughterhouse operations, quality control, and supply chain tracking.

## Features

### Core Operations
- Animal tracking and management
- Quality inspection system
- Employee management
- Maintenance scheduling
- Shipment tracking
- Waste management
- Financial analytics

### Data Management
- Secure and transparent record-keeping
- Real-time analytics and reporting
- Supply chain traceability
- Quality control metrics
- Equipment maintenance tracking

## System Components

### Animal Management
- Track individual animals
- Record animal health data
- Monitor processing status
- Maintain compliance records

### Quality Control
- Conduct quality inspections
- Record temperature and pH levels
- Visual inspection documentation
- Pass/fail tracking
- Historical inspection data

### Employee Management
- Employee registration and tracking
- Role-based access control
- Certification management
- Performance monitoring
- Status tracking (active/inactive/suspended)

### Maintenance Management
- Equipment maintenance scheduling
- Cost tracking
- Maintenance history
- Performance analytics
- Preventive maintenance planning

### Shipment Tracking
- Product shipment management
- Temperature logging
- Delivery status tracking
- Destination management
- Product batch tracking

### Waste Management
- Waste disposal tracking
- Cost monitoring
- Disposal method documentation
- Compliance reporting
- Environmental impact tracking

### Financial Analytics
- Revenue tracking
- Expense monitoring
- Profit margin calculation
- Cost analysis by category
- Product-specific revenue analysis

## Technical Implementation

### Data Structures

#### Core Structs
```rust
QualityInspection
Employee
MaintenanceRecord
Supplier
Shipment
WasteRecord
FinancialMetrics
```

#### Payload Structs
```rust
QualityInspectionPayload
EmployeePayload
MaintenancePayload
ShipmentPayload
```

### Analytics

#### Quality Metrics
- Total inspection count
- Pass/fail rates
- Average temperature and pH levels
- Inspection history

#### Maintenance Analytics
- Cost analysis
- Equipment reliability
- Pending maintenance
- Historical maintenance records

#### Inventory Analytics
- Product counts
- Inventory value
- Status tracking
- Low stock alerts

## API Reference

### Update Methods

#### Quality Control
```rust
fn perform_quality_inspection(payload: QualityInspectionPayload) -> Result<QualityInspection, Message>
```

#### Employee Management
```rust
fn register_employee(payload: EmployeePayload) -> Result<Employee, Message>
```

#### Maintenance
```rust
fn schedule_maintenance(payload: MaintenancePayload) -> Result<MaintenanceRecord, Message>
```

#### Shipment
```rust
fn create_shipment(payload: ShipmentPayload) -> Result<Shipment, Message>
```

#### Waste Management
```rust
fn manage_waste_disposal(
    slaughterhouse_id: u64,
    waste_type: String,
    quantity: f64,
    disposal_method: String,
    cost: f64,
    handled_by: String,
) -> Result<WasteRecord, Message>
```

### Query Methods

#### Analytics
```rust
fn generate_financial_analytics(slaughterhouse_id: u64) -> Result<FinancialMetrics, Message>
fn get_quality_metrics(slaughterhouse_id: u64, start_date: u64, end_date: u64) -> Result<QualityMetrics, Message>
fn get_maintenance_analytics(slaughterhouse_id: u64, start_date: u64, end_date: u64) -> Result<MaintenanceAnalytics, Message>
fn get_inventory_analytics(slaughterhouse_id: u64) -> Result<InventoryAnalytics, Message>
```

## Installation

1. Install the DFINITY Canister SDK
2. Clone the repository
3. Start the local replica
4. Deploy the canister:
```bash
dfx deploy
```

## Usage

### Initialize Slaughterhouse
```bash
dfx canister call slaughterhouse_management init_slaughterhouse '(record { name = "Main Facility"; location = "123 Processing St"; capacity = 1000 })'
```

### Register Employee
```bash
dfx canister call slaughterhouse_management register_employee '(record { slaughterhouse_id = 1; name = "John Doe"; role = "Inspector"; certification = "FDA-123"; contact = "john@example.com" })'
```

### Schedule Maintenance
```bash
dfx canister call slaughterhouse_management schedule_maintenance '(record { slaughterhouse_id = 1; equipment_name = "Conveyor Belt"; maintenance_type = "Preventive"; scheduled_date = 1678900000; estimated_cost = 500.00; notes = "Regular maintenance" })'
```

## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown targetz
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```