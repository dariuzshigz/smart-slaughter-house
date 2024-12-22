#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::collections::HashMap;
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Slaughterhouse struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Slaughterhouse {
    id: u64,
    name: String,
    location: String,
    contact: String,
    email: String,
    capacity: u64, // Maximum number of animals handled per day
    created_at: u64,
}

// Animal struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Animal {
    id: u64,
    slaughterhouse_id: u64,
    tag_number: String,
    species: String, // e.g., cow, sheep, goat, pig
    weight: f64,     // in kilograms
    arrival_time: u64,
    status: String, // "received", "processed", "disposed"
}

// MeatProduct struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct MeatProduct {
    id: u64,
    animal_id: u64,
    slaughterhouse_id: u64,
    product_type: String, // e.g., steak, ribs, minced meat
    weight: f64,          // in kilograms
    price_per_kg: f64,
    total_price: f64,
    status: String, // "in-stock", "sold", "disposed"
    created_at: u64,
}

// Expense struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Expense {
    id: u64,
    slaughterhouse_id: u64,
    date: u64,
    category: String,
    amount: f64,
    description: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct QualityInspection {
    id: u64,
    animal_id: u64,
    inspector_name: String,
    inspection_date: u64,
    temperature: f64,
    ph_level: f64,
    visual_inspection: String,
    passed: bool,
    notes: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Employee {
    id: u64,
    slaughterhouse_id: u64,
    name: String,
    role: String,
    certification: String,
    hire_date: u64,
    contact: String,
    status: String, // active, inactive, suspended
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct MaintenanceRecord {
    id: u64,
    slaughterhouse_id: u64,
    equipment_name: String,
    maintenance_type: String,
    cost: f64,
    date: u64,
    next_maintenance_date: u64,
    performed_by: String,
    status: String, // scheduled, in-progress, completed
    notes: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Supplier {
    id: u64,
    name: String,
    contact: String,
    email: String,
    supplier_type: String,
    rating: u8,
    active_since: u64,
    last_supply_date: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Shipment {
    id: u64,
    slaughterhouse_id: u64,
    product_ids: Vec<u64>,
    destination: String,
    shipping_date: u64,
    expected_delivery: u64,
    temperature_log: Vec<f64>,
    status: String,
    tracking_number: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct WasteRecord {
    id: u64,
    slaughterhouse_id: u64,
    waste_type: String,
    quantity: f64,
    disposal_method: String,
    disposal_date: u64,
    handled_by: String,
    cost: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct FinancialMetrics {
    total_revenue: f64,
    total_expenses: f64,
    profit_margin: f64,
    operating_costs: f64,
    maintenance_costs: f64,
    labor_costs: f64,
    waste_management_costs: f64,
    revenue_by_product: HashMap<String, f64>,
    expenses_by_category: HashMap<String, f64>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct QualityMetrics {
    total_inspections: u32,
    passed_inspections: u32,
    failure_rate: f64,
    average_temperature: f64,
    average_ph_level: f64,
    inspections: Vec<QualityInspection>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct MaintenanceAnalytics {
    total_maintenance_cost: f64,
    maintenance_by_type: HashMap<String, u32>,
    equipment_reliability: HashMap<String, f64>,
    pending_maintenance: Vec<MaintenanceRecord>,
    equipment_history: HashMap<String, Vec<MaintenanceRecord>>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct InventoryAnalytics {
    product_counts: HashMap<String, u32>,
    total_inventory_value: f64,
    products_by_status: HashMap<String, Vec<MeatProduct>>,
    low_stock_items: Vec<MeatProduct>,
}

// Payload structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateSlaughterhousePayload {
    name: String,
    location: String,
    contact: String,
    email: String,
    capacity: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RegisterAnimalPayload {
    slaughterhouse_id: u64,
    tag_number: String,
    species: String,
    weight: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateMeatProductPayload {
    animal_id: u64,
    slaughterhouse_id: u64,
    product_type: String,
    weight: f64,
    price_per_kg: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RecordExpensePayload {
    slaughterhouse_id: u64,
    category: String,
    amount: f64,
    description: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct QualityInspectionPayload {
    animal_id: u64,
    inspector_name: String,
    temperature: f64,
    ph_level: f64,
    visual_inspection: String,
    passed: bool,
    notes: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EmployeePayload {
    slaughterhouse_id: u64,
    name: String,
    role: String,
    certification: String,
    contact: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct MaintenancePayload {
    slaughterhouse_id: u64,
    equipment_name: String,
    maintenance_type: String,
    scheduled_date: u64,
    estimated_cost: f64,
    notes: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ShipmentPayload {
    slaughterhouse_id: u64,
    product_ids: Vec<u64>,
    destination: String,
    expected_delivery: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

// Implementing Storable for Slaughterhouse
impl Storable for Slaughterhouse {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Slaughterhouse {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Animal
impl Storable for Animal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Animal {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for MeatProduct
impl Storable for MeatProduct {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for MeatProduct {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Expense
impl Storable for Expense {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Expense {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for QualityInspection
impl Storable for QualityInspection {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for QualityInspection {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Employee
impl Storable for Employee {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Employee {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for MaintenanceRecord
impl Storable for MaintenanceRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for MaintenanceRecord {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Supplier
impl Storable for Supplier {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Supplier {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Shipment
impl Storable for Shipment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Shipment {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for WasteRecord
impl Storable for WasteRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for WasteRecord {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for FinancialMetrics
impl Storable for FinancialMetrics {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FinancialMetrics {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Memory management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static SLAUGHTERHOUSES: RefCell<StableBTreeMap<u64, Slaughterhouse, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(10)))
        ));

    static ANIMALS: RefCell<StableBTreeMap<u64, Animal, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(11)))
        ));

    static MEAT_PRODUCTS: RefCell<StableBTreeMap<u64, MeatProduct, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(12)))
        ));

    static EXPENSES: RefCell<StableBTreeMap<u64, Expense, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(13)))
        ));

    static QUALITY_INSPECTIONS: RefCell<StableBTreeMap<u64, QualityInspection, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(14)))
        ));

    static EMPLOYEES: RefCell<StableBTreeMap<u64, Employee, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(15)))
        ));

    static MAINTENANCE_RECORDS: RefCell<StableBTreeMap<u64, MaintenanceRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(16)))
        ));

    static SUPPLIERS: RefCell<StableBTreeMap<u64, Supplier, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(17)))
        ));

    static SHIPMENTS: RefCell<StableBTreeMap<u64, Shipment, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(18)))
        ));

    static WASTE_RECORDS: RefCell<StableBTreeMap<u64, WasteRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(19)))
        ));

    static FINANCIAL_METRICS: RefCell<StableBTreeMap<u64, FinancialMetrics, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(20)))
        ));


}

// Functions

// Create Slaughterhouse
#[ic_cdk::update]
fn create_slaughterhouse(payload: CreateSlaughterhousePayload) -> Result<Slaughterhouse, Message> {
    if payload.name.is_empty() || payload.contact.is_empty() || payload.email.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    let slaughterhouse_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let slaughterhouse = Slaughterhouse {
        id: slaughterhouse_id,
        name: payload.name,
        location: payload.location,
        contact: payload.contact,
        email: payload.email,
        capacity: payload.capacity,
        created_at: time(),
    };

    SLAUGHTERHOUSES.with(|houses| {
        houses
            .borrow_mut()
            .insert(slaughterhouse_id, slaughterhouse.clone());
    });

    Ok(slaughterhouse)
}

// Register Animal
#[ic_cdk::update]
fn register_animal(payload: RegisterAnimalPayload) -> Result<Animal, Message> {
    if payload.tag_number.is_empty() || payload.species.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    let slaughterhouse_exists =
        SLAUGHTERHOUSES.with(|houses| houses.borrow().contains_key(&payload.slaughterhouse_id));
    if !slaughterhouse_exists {
        return Err(Message::NotFound("Slaughterhouse not found".to_string()));
    }

    let animal_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let animal = Animal {
        id: animal_id,
        slaughterhouse_id: payload.slaughterhouse_id,
        tag_number: payload.tag_number,
        species: payload.species,
        weight: payload.weight,
        arrival_time: time(),
        status: "received".to_string(),
    };

    ANIMALS.with(|animals| {
        animals.borrow_mut().insert(animal_id, animal.clone());
    });

    Ok(animal)
}

// Create Meat Product
#[ic_cdk::update]
fn create_meat_product(payload: CreateMeatProductPayload) -> Result<MeatProduct, Message> {
    if payload.product_type.is_empty() || payload.weight <= 0.0 || payload.price_per_kg <= 0.0 {
        return Err(Message::InvalidPayload("Invalid product data".to_string()));
    }

    let animal_exists = ANIMALS.with(|animals| animals.borrow().contains_key(&payload.animal_id));
    if !animal_exists {
        return Err(Message::NotFound("Animal not found".to_string()));
    }

    let slaughterhouse_exists =
        SLAUGHTERHOUSES.with(|houses| houses.borrow().contains_key(&payload.slaughterhouse_id));
    if !slaughterhouse_exists {
        return Err(Message::NotFound("Slaughterhouse not found".to_string()));
    }

    let meat_product_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let total_price = payload.weight * payload.price_per_kg;

    let meat_product = MeatProduct {
        id: meat_product_id,
        animal_id: payload.animal_id,
        slaughterhouse_id: payload.slaughterhouse_id,
        product_type: payload.product_type,
        weight: payload.weight,
        price_per_kg: payload.price_per_kg,
        total_price,
        status: "in-stock".to_string(),
        created_at: time(),
    };

    MEAT_PRODUCTS.with(|products| {
        products
            .borrow_mut()
            .insert(meat_product_id, meat_product.clone());
    });

    Ok(meat_product)
}

// Record Expense
#[ic_cdk::update]
fn record_expense(payload: RecordExpensePayload) -> Result<Expense, Message> {
    if payload.amount <= 0.0 {
        return Err(Message::InvalidPayload(
            "Invalid expense amount".to_string(),
        ));
    }

    let slaughterhouse_exists =
        SLAUGHTERHOUSES.with(|houses| houses.borrow().contains_key(&payload.slaughterhouse_id));
    if !slaughterhouse_exists {
        return Err(Message::NotFound("Slaughterhouse not found".to_string()));
    }

    let expense_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let expense = Expense {
        id: expense_id,
        slaughterhouse_id: payload.slaughterhouse_id,
        date: time(),
        category: payload.category,
        amount: payload.amount,
        description: payload.description,
    };

    EXPENSES.with(|expenses| {
        expenses.borrow_mut().insert(expense_id, expense.clone());
    });

    Ok(expense)
}

// Calculate Total Revenue
#[ic_cdk::query]
fn calculate_total_revenue(slaughterhouse_id: u64) -> Result<f64, Message> {
    let slaughterhouse_exists =
        SLAUGHTERHOUSES.with(|houses| houses.borrow().contains_key(&slaughterhouse_id));
    if !slaughterhouse_exists {
        return Err(Message::NotFound("Slaughterhouse not found".to_string()));
    }

    let total_revenue: f64 = MEAT_PRODUCTS.with(|products| {
        products
            .borrow()
            .iter()
            .filter(|(_, product)| product.slaughterhouse_id == slaughterhouse_id)
            .map(|(_, product)| product.total_price)
            .sum()
    });

    Ok(total_revenue)
}

// Calculate Total Expenses
#[ic_cdk::query]
fn calculate_total_expenses(slaughterhouse_id: u64) -> Result<f64, Message> {
    let slaughterhouse_exists =
        SLAUGHTERHOUSES.with(|houses| houses.borrow().contains_key(&slaughterhouse_id));
    if !slaughterhouse_exists {
        return Err(Message::NotFound("Slaughterhouse not found".to_string()));
    }

    let total_expenses: f64 = EXPENSES.with(|expenses| {
        expenses
            .borrow()
            .iter()
            .filter(|(_, expense)| expense.slaughterhouse_id == slaughterhouse_id)
            .map(|(_, expense)| expense.amount)
            .sum()
    });

    Ok(total_expenses)
}

#[ic_cdk::update]
fn perform_quality_inspection(
    payload: QualityInspectionPayload,
) -> Result<QualityInspection, Message> {
    // Validate animal exists
    let animal_exists = ANIMALS.with(|animals| animals.borrow().contains_key(&payload.animal_id));
    if !animal_exists {
        return Err(Message::NotFound("Animal not found".to_string()));
    }

    let inspection_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Counter increment failed");
        current_value
    });

    let inspection = QualityInspection {
        id: inspection_id,
        animal_id: payload.animal_id,
        inspector_name: payload.inspector_name,
        inspection_date: time(),
        temperature: payload.temperature,
        ph_level: payload.ph_level,
        visual_inspection: payload.visual_inspection,
        passed: payload.passed,
        notes: payload.notes,
    };

    QUALITY_INSPECTIONS.with(|inspections| {
        inspections
            .borrow_mut()
            .insert(inspection_id, inspection.clone());
    });

    Ok(inspection)
}

#[ic_cdk::update]
fn register_employee(payload: EmployeePayload) -> Result<Employee, Message> {
    if payload.name.is_empty() || payload.role.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    let slaughterhouse_exists =
        SLAUGHTERHOUSES.with(|houses| houses.borrow().contains_key(&payload.slaughterhouse_id));
    if !slaughterhouse_exists {
        return Err(Message::NotFound("Slaughterhouse not found".to_string()));
    }

    let employee_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Counter increment failed");
        current_value
    });

    let employee = Employee {
        id: employee_id,
        slaughterhouse_id: payload.slaughterhouse_id,
        name: payload.name,
        role: payload.role,
        certification: payload.certification,
        hire_date: time(),
        contact: payload.contact,
        status: "active".to_string(),
    };

    EMPLOYEES.with(|employees| {
        employees.borrow_mut().insert(employee_id, employee.clone());
    });

    Ok(employee)
}

#[ic_cdk::update]
fn schedule_maintenance(payload: MaintenancePayload) -> Result<MaintenanceRecord, Message> {
    if payload.equipment_name.is_empty() || payload.maintenance_type.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    let maintenance_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Counter increment failed");
        current_value
    });

    let record = MaintenanceRecord {
        id: maintenance_id,
        slaughterhouse_id: payload.slaughterhouse_id,
        equipment_name: payload.equipment_name,
        maintenance_type: payload.maintenance_type,
        cost: payload.estimated_cost,
        date: payload.scheduled_date,
        next_maintenance_date: payload.scheduled_date + 7_884_000, // Default to 3 months
        performed_by: "".to_string(),
        status: "scheduled".to_string(),
        notes: payload.notes,
    };

    MAINTENANCE_RECORDS.with(|records| {
        records.borrow_mut().insert(maintenance_id, record.clone());
    });

    Ok(record)
}

#[ic_cdk::update]
fn create_shipment(payload: ShipmentPayload) -> Result<Shipment, Message> {
    // Validate all products exist
    for product_id in &payload.product_ids {
        let product_exists =
            MEAT_PRODUCTS.with(|products| products.borrow().contains_key(product_id));
        if !product_exists {
            return Err(Message::NotFound(format!(
                "Product {} not found",
                product_id
            )));
        }
    }

    let shipment_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Counter increment failed");
        current_value
    });

    let tracking_number = format!("SH{:0>6}", shipment_id);

    let shipment = Shipment {
        id: shipment_id,
        slaughterhouse_id: payload.slaughterhouse_id,
        product_ids: payload.product_ids,
        destination: payload.destination,
        shipping_date: time(),
        expected_delivery: payload.expected_delivery,
        temperature_log: Vec::new(),
        status: "preparing".to_string(),
        tracking_number,
    };

    SHIPMENTS.with(|shipments| {
        shipments.borrow_mut().insert(shipment_id, shipment.clone());
    });

    Ok(shipment)
}

#[ic_cdk::query]
fn generate_financial_analytics(slaughterhouse_id: u64) -> Result<FinancialMetrics, Message> {
    let slaughterhouse_exists =
        SLAUGHTERHOUSES.with(|houses| houses.borrow().contains_key(&slaughterhouse_id));
    if !slaughterhouse_exists {
        return Err(Message::NotFound("Slaughterhouse not found".to_string()));
    }

    let total_revenue = calculate_total_revenue(slaughterhouse_id)?;
    let total_expenses = calculate_total_expenses(slaughterhouse_id)?;

    let maintenance_costs: f64 = MAINTENANCE_RECORDS.with(|records| {
        records
            .borrow()
            .iter()
            .filter(|(_, record)| record.slaughterhouse_id == slaughterhouse_id)
            .map(|(_, record)| record.cost)
            .sum()
    });

    let waste_management_costs: f64 = WASTE_RECORDS.with(|records| {
        records
            .borrow()
            .iter()
            .filter(|(_, record)| record.slaughterhouse_id == slaughterhouse_id)
            .map(|(_, record)| record.cost)
            .sum()
    });

    let mut revenue_by_product: HashMap<String, f64> = HashMap::new();
    MEAT_PRODUCTS.with(|products| {
        products
            .borrow()
            .iter()
            .filter(|(_, product)| product.slaughterhouse_id == slaughterhouse_id)
            .for_each(|(_, product)| {
                *revenue_by_product
                    .entry(product.product_type.clone())
                    .or_insert(0.0) += product.total_price;
            });
    });

    let metrics = FinancialMetrics {
        total_revenue,
        total_expenses,
        profit_margin: ((total_revenue - total_expenses) / total_revenue * 100.0).max(0.0),
        operating_costs: total_expenses - maintenance_costs - waste_management_costs,
        maintenance_costs,
        labor_costs: 0.0, // Would need to implement payroll system
        waste_management_costs,
        revenue_by_product,
        expenses_by_category: HashMap::new(), // Would need to implement expense categorization
    };

    Ok(metrics)
}

#[ic_cdk::update]
fn manage_waste_disposal(
    slaughterhouse_id: u64,
    waste_type: String,
    quantity: f64,
    disposal_method: String,
    cost: f64,
    handled_by: String,
) -> Result<WasteRecord, Message> {
    let waste_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Counter increment failed");
        current_value
    });

    let record = WasteRecord {
        id: waste_id,
        slaughterhouse_id,
        waste_type,
        quantity,
        disposal_method,
        disposal_date: time(),
        handled_by,
        cost,
    };

    WASTE_RECORDS.with(|records| {
        records.borrow_mut().insert(waste_id, record.clone());
    });

    Ok(record)
}

// Query functions for analytics and reporting

#[ic_cdk::query]
fn get_quality_metrics(
    slaughterhouse_id: u64,
    start_date: u64,
    end_date: u64,
) -> Result<QualityMetrics, Message> {
    let slaughterhouse_exists =
        SLAUGHTERHOUSES.with(|houses| houses.borrow().contains_key(&slaughterhouse_id));
    if !slaughterhouse_exists {
        return Err(Message::NotFound("Slaughterhouse not found".to_string()));
    }

    let mut total_inspections = 0;
    let mut passed_inspections = 0;
    let mut avg_temperature = 0.0;
    let mut avg_ph_level = 0.0;
    let mut inspections_in_range = Vec::new();

    QUALITY_INSPECTIONS.with(|inspections| {
        inspections
            .borrow()
            .iter()
            .filter(|(_, inspection)| {
                let animal = ANIMALS.with(|animals| {
                    animals
                        .borrow()
                        .get(&inspection.animal_id)
                        .map(|animal| animal.slaughterhouse_id == slaughterhouse_id)
                        .unwrap_or(false)
                });
                animal
                    && inspection.inspection_date >= start_date
                    && inspection.inspection_date <= end_date
            })
            .for_each(|(_, inspection)| {
                total_inspections += 1;
                if inspection.passed {
                    passed_inspections += 1;
                }
                avg_temperature += inspection.temperature;
                avg_ph_level += inspection.ph_level;
                inspections_in_range.push(inspection.clone());
            });
    });

    if total_inspections > 0 {
        avg_temperature /= total_inspections as f64;
        avg_ph_level /= total_inspections as f64;
    }

    Ok(QualityMetrics {
        total_inspections,
        passed_inspections,
        failure_rate: if total_inspections > 0 {
            ((total_inspections - passed_inspections) as f64 / total_inspections as f64 * 100.0)
                .round()
        } else {
            0.0
        },
        average_temperature: avg_temperature.round(),
        average_ph_level: avg_ph_level.round(),
        inspections: inspections_in_range,
    })
}

#[ic_cdk::query]
fn get_maintenance_analytics(
    slaughterhouse_id: u64,
    start_date: u64,
    end_date: u64,
) -> Result<MaintenanceAnalytics, Message> {
    let mut total_maintenance_cost = 0.0;
    let mut maintenance_by_type: HashMap<String, u32> = HashMap::new();
    let mut equipment_history: HashMap<String, Vec<MaintenanceRecord>> = HashMap::new();
    let mut pending_maintenance = Vec::new();

    MAINTENANCE_RECORDS.with(|records| {
        records
            .borrow()
            .iter()
            .filter(|(_, record)| {
                record.slaughterhouse_id == slaughterhouse_id
                    && record.date >= start_date
                    && record.date <= end_date
            })
            .for_each(|(_, record)| {
                total_maintenance_cost += record.cost;
                *maintenance_by_type
                    .entry(record.maintenance_type.clone())
                    .or_insert(0) += 1;
                equipment_history
                    .entry(record.equipment_name.clone())
                    .or_insert_with(Vec::new)
                    .push(record.clone());

                if record.status == "scheduled" || record.status == "in-progress" {
                    pending_maintenance.push(record.clone());
                }
            });
    });

    let mut equipment_reliability: HashMap<String, f64> = HashMap::new();
    for (equipment, history) in &equipment_history {
        let total_records = history.len() as f64;
        let emergency_repairs = history
            .iter()
            .filter(|record| record.maintenance_type == "emergency")
            .count() as f64;
        let reliability = ((total_records - emergency_repairs) / total_records * 100.0).round();
        equipment_reliability.insert(equipment.clone(), reliability);
    }

    Ok(MaintenanceAnalytics {
        total_maintenance_cost,
        maintenance_by_type,
        equipment_reliability,
        pending_maintenance,
        equipment_history,
    })
}

#[ic_cdk::query]
fn get_inventory_analytics(slaughterhouse_id: u64) -> Result<InventoryAnalytics, Message> {
    let mut product_counts: HashMap<String, u32> = HashMap::new();
    let mut total_value = 0.0;
    let mut products_by_status: HashMap<String, Vec<MeatProduct>> = HashMap::new();
    let mut low_stock_items = Vec::new();

    MEAT_PRODUCTS.with(|products| {
        products
            .borrow()
            .iter()
            .filter(|(_, product)| product.slaughterhouse_id == slaughterhouse_id)
            .for_each(|(_, product)| {
                *product_counts
                    .entry(product.product_type.clone())
                    .or_insert(0) += 1;
                total_value += product.total_price;

                products_by_status
                    .entry(product.status.clone())
                    .or_insert_with(Vec::new)
                    .push(product.clone());

                // Consider products with quantity below 10 as low stock
                if product.weight < 10.0 {
                    low_stock_items.push(product.clone());
                }
            });
    });

    Ok(InventoryAnalytics {
        product_counts,
        total_inventory_value: total_value,
        products_by_status,
        low_stock_items,
    })
}

// Exporting the candid interface
ic_cdk::export_candid!();
