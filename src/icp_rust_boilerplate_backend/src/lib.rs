#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Define type aliases for better readability
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define LandTitle struct
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct LandTitle {
    id: u64,
    landowner_id: u64, // Refers to the landowner's ID
    location: String,
    size: f64, // Size of the land (in acres/hectares)
    description: String,
    registered_at: u64, // Timestamp when the land title was registered
    price: u64, // Price of the land
    is_available: bool, // Whether the land is available for sale
}

impl Storable for LandTitle {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for LandTitle {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define LandTransfer struct
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct LandTransfer {
    id: u64,
    land_title_id: u64, // Reference to the land title being transferred
    buyer_id: u64, // Reference to the buyer
    landowner_id: u64, // Reference to the seller (landowner)
    agreed_price: u64, // Price agreed upon
    transfer_status: String, // e.g., "Pending", "Completed"
    initiated_at: u64, // Timestamp when transfer was initiated
    approved_at: Option<u64>, // Timestamp when transfer was approved
}

impl Storable for LandTransfer {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for LandTransfer {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define Landowner struct
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Landowner {
    id: u64,
    landowneridentity_id: String, // Unique digital identity for landowner
    name: String,
    email: String,
    phone: String,
    land_titles: Vec<u64>, // List of land title IDs owned by the landowner
    is_verified: bool,
}

impl Storable for Landowner {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Landowner {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define Buyer struct
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Buyer {
    id: u64,
    buyeridentity_id: String, // Unique digital identity for buyer
    name: String,
    email: String,
    phone: String,
    purchased_land_titles: Vec<u64>, // List of purchased land title IDs
}

impl Storable for Buyer {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Buyer {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define GovernmentOfficial struct
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct GovernmentOfficial {
    id: u64,
    officialidentity_id: String, // Unique digital identity for government official
    name: String,
    department: String,
    email: String,
    phone: String,
    is_active: bool,
}

impl Storable for GovernmentOfficial {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for GovernmentOfficial {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define Arbitrator struct
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Arbitrator {
    id: u64,
    arbitratoridentity_id: String, // Unique digital identity for arbitrator
    name: String,
    expertise: String, // Expertise in arbitration (e.g., land law)
    email: String,
    phone: String,
    is_active: bool,
}

impl Storable for Arbitrator {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Arbitrator {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define Dispute struct
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Dispute {
    id: u64,
    land_title_id: u64, // Reference to the land title in dispute
    raised_by: u64, // ID of the user who raised the dispute (landowner, buyer)
    comment: String, // Explanation of the dispute
    created_at: u64, // Timestamp when dispute was created
    resolved_at: Option<u64>, // Timestamp when dispute was resolved
}

impl Storable for Dispute {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Dispute {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define TransactionRecord struct
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct TransactionRecord {
    id: u64,
    land_title_id: u64, // Land title associated with the transaction
    transaction_type: String, // "Transfer", "Registration", etc.
    timestamp: u64, // Time of transaction
    details: String, // Additional details about the transaction
}

impl Storable for TransactionRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for TransactionRecord {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define BuyerIdentity struct
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct BuyerIdentity {
    id: u64,
    principal: String, // Principal associated with the buyer
}

// Implementation for BuyerIdentity
impl Storable for BuyerIdentity {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for BuyerIdentity {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define LandownerIdentity struct
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct LandownerIdentity {
    id: u64,
    principal: String, // Principal associated with the landowner
}

// Implementation for LandownerIdentity
impl Storable for LandownerIdentity {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for LandownerIdentity {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define OfficialIdentity struct
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct OfficialIdentity {
    id: u64,
    principal: String, // Principal associated with the official
}

// Implementation for OfficialIdentity
impl Storable for OfficialIdentity {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for OfficialIdentity {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define ArbitratorIdentity struct
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct ArbitratorIdentity {
    id: u64,
    principal: String, // Principal associated with the arbitrator
}

// Implementation for ArbitratorIdentity
impl Storable for ArbitratorIdentity {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for ArbitratorIdentity {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Error handling
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
    Unauthorized { msg: String },
    TransferConflict { msg: String },
    AlreadyExists { msg: String },
}

// Thread-local variables for managing memory and stable storage.
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static LAND_TITLE_STORAGE: RefCell<StableBTreeMap<u64, LandTitle, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static TRANSFER_STORAGE: RefCell<StableBTreeMap<u64, LandTransfer, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static LANDOWNER_STORAGE: RefCell<StableBTreeMap<u64, Landowner, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
    
    static BUYER_STORAGE: RefCell<StableBTreeMap<u64, Buyer, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
    
    static GOV_OFFICIAL_STORAGE: RefCell<StableBTreeMap<u64, GovernmentOfficial, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));
    
    static ARBITRATOR_STORAGE: RefCell<StableBTreeMap<u64, Arbitrator, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));
    
    static DISPUTE_STORAGE: RefCell<StableBTreeMap<u64, Dispute, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7)))
    ));
    
    static TRANSACTION_STORAGE: RefCell<StableBTreeMap<u64, TransactionRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(8)))
    ));
    static BUYER_IDENTITY_STORAGE: RefCell<StableBTreeMap<u64, BuyerIdentity, Memory>> =
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(10)))
    ));  
    static LANDOWNER_IDENTITY_STORAGE: RefCell<StableBTreeMap<u64, LandownerIdentity, Memory>> =
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(11)))
    ));
    static OFFICIAL_IDENTITY_STORAGE: RefCell<StableBTreeMap<u64, OfficialIdentity, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(12)))
    ));
    static ARBITRATOR_IDENTITY_STORAGE: RefCell<StableBTreeMap<u64, ArbitratorIdentity, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(13)))
    ));
}

// CRUD operations for Land Titles
#[ic_cdk::update]
fn add_land_title(landowner_id: u64, location: String, size: f64, description: String, price: u64) -> Result<LandTitle, Error> {
    // Validate input data
    if location.is_empty() || size <= 0.0 || description.is_empty() || price <= 0 {
        return Err(Error::InvalidInput {
            msg: "All fields must be valid".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let land_title = LandTitle {
        id,
        landowner_id,
        location,
        size,
        description,
        registered_at: ic_cdk::api::time(),
        price,
        is_available: true,
    };

    LAND_TITLE_STORAGE.with(|service| service.borrow_mut().insert(id, land_title.clone()));
    Ok(land_title)
}

#[ic_cdk::query]
fn get_land_title(id: u64) -> Result<LandTitle, Error> {
    match LAND_TITLE_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(land_title) => Ok(land_title.clone()),
        None => Err(Error::NotFound {
            msg: format!("Land title with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn update_land_title(id: u64, location: Option<String>, size: Option<f64>, description: Option<String>, price: Option<u64>, is_available: Option<bool>) -> Result<LandTitle, Error> {
    let mut land_title = match LAND_TITLE_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        Some(title) => title.clone(),
        None => return Err(Error::NotFound {
            msg: format!("Land title with id={} not found", id),
        }),
    };

    if let Some(loc) = location {
        land_title.location = loc;
    }
    if let Some(s) = size {
        land_title.size = s;
    }
    if let Some(desc) = description {
        land_title.description = desc;
    }
    if let Some(p) = price {
        land_title.price = p;
    }
    if let Some(avail) = is_available {
        land_title.is_available = avail;
    }

    LAND_TITLE_STORAGE.with(|service| service.borrow_mut().insert(id, land_title.clone()));
    Ok(land_title)
}

#[ic_cdk::update]
fn delete_land_title(id: u64) -> Result<(), Error> {
    match LAND_TITLE_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Land title with id={} not found", id),
        }),
    }
}

// CRUD operations for Land Transfers
#[ic_cdk::update]
fn add_land_transfer(land_title_id: u64, buyer_id: u64, landowner_id: u64, agreed_price: u64) -> Result<LandTransfer, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let land_transfer = LandTransfer {
        id,
        land_title_id,
        buyer_id,
        landowner_id,
        agreed_price,
        transfer_status: "Pending".to_string(),
        initiated_at: ic_cdk::api::time(),
        approved_at: None,
    };

    TRANSFER_STORAGE.with(|service| service.borrow_mut().insert(id, land_transfer.clone()));
    Ok(land_transfer)
}

#[ic_cdk::query]
fn get_land_transfer(id: u64) -> Result<LandTransfer, Error> {
    match TRANSFER_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(transfer) => Ok(transfer.clone()),
        None => Err(Error::NotFound {
            msg: format!("Land transfer with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn update_land_transfer(id: u64, buyer_id: Option<u64>, landowner_id: Option<u64>, agreed_price: Option<u64>, transfer_status: Option<String>, approved_at: Option<u64>) -> Result<LandTransfer, Error> {
    let mut land_transfer = match TRANSFER_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        Some(transfer) => transfer.clone(),
        None => return Err(Error::NotFound {
            msg: format!("Land transfer with id={} not found", id),
        }),
    };

    if let Some(b) = buyer_id {
        land_transfer.buyer_id = b;
    }
    if let Some(o) = landowner_id {
        land_transfer.landowner_id = o;
    }
    if let Some(p) = agreed_price {
        land_transfer.agreed_price = p;
    }
    if let Some(s) = transfer_status {
        land_transfer.transfer_status = s;
    }
    if let Some(a) = approved_at {
        land_transfer.approved_at = Some(a);
    }

    TRANSFER_STORAGE.with(|service| service.borrow_mut().insert(id, land_transfer.clone()));
    Ok(land_transfer)
}

#[ic_cdk::update]
fn delete_land_transfer(id: u64) -> Result<(), Error> {
    match TRANSFER_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Land transfer with id={} not found", id),
        }),
    }
}

// CRUD operations for Landowners
#[ic_cdk::update]
fn add_landowner(landowneridentity_id: String, name: String, email: String, phone: String) -> Result<Landowner, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let landowner = Landowner {
        id,
        landowneridentity_id,
        name,
        email,
        phone,
        land_titles: vec![],
        is_verified: false,
    };

    LANDOWNER_STORAGE.with(|service| service.borrow_mut().insert(id, landowner.clone()));
    Ok(landowner)
}

// Get Landowner by landowner_identity_id
#[ic_cdk::query]
fn get_landowner(landowner_identity_id: u64) -> Result<Landowner, Error> {
    match _get_landowner(&landowner_identity_id) {
        Some(landowner) => Ok(landowner),
        None => Err(Error::NotFound {
            msg: format!("Landowner with landowner_identity_id={} not found", landowner_identity_id),
        }),
    }
}

#[ic_cdk::update]
fn update_landowner(id: u64, name: Option<String>, email: Option<String>, phone: Option<String>, is_verified: Option<bool>) -> Result<Landowner, Error> {
    let mut landowner = match LANDOWNER_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        Some(owner) => owner.clone(),
        None => return Err(Error::NotFound {
            msg: format!("Landowner with id={} not found", id),
        }),
    };

    if let Some(n) = name {
        landowner.name = n;
    }
    if let Some(e) = email {
        landowner.email = e;
    }
    if let Some(p) = phone {
        landowner.phone = p;
    }
    if let Some(v) = is_verified {
        landowner.is_verified = v;
    }

    LANDOWNER_STORAGE.with(|service| service.borrow_mut().insert(id, landowner.clone()));
    Ok(landowner)
}

#[ic_cdk::update]
fn delete_landowner(id: u64) -> Result<(), Error> {
    match LANDOWNER_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Landowner with id={} not found", id),
        }),
    }
}

// CRUD operations for Buyers
#[ic_cdk::update]
fn add_buyer(buyeridentity_id: String, name: String, email: String, phone: String) -> Result<Buyer, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let buyer = Buyer {
        id,
        buyeridentity_id,
        name,
        email,
        phone,
        purchased_land_titles: vec![],
    };

    BUYER_STORAGE.with(|service| service.borrow_mut().insert(id, buyer.clone()));
    Ok(buyer)
}

// Get Buyer by buyer_identity_id
#[ic_cdk::query]
fn get_buyer(buyer_identity_id: u64) -> Result<Buyer, Error> {
    match _get_buyer(&buyer_identity_id) {
        Some(buyer) => Ok(buyer),
        None => Err(Error::NotFound {
            msg: format!("Buyer with buyer_identity_id={} not found", buyer_identity_id),
        }),
    }
}

#[ic_cdk::update]
fn update_buyer(id: u64, name: Option<String>, email: Option<String>, phone: Option<String>) -> Result<Buyer, Error> {
    let mut buyer = match BUYER_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        Some(b) => b.clone(),
        None => return Err(Error::NotFound {
            msg: format!("Buyer with id={} not found", id),
        }),
    };

    if let Some(n) = name {
        buyer.name = n;
    }
    if let Some(e) = email {
        buyer.email = e;
    }
    if let Some(p) = phone {
        buyer.phone = p;
    }

    BUYER_STORAGE.with(|service| service.borrow_mut().insert(id, buyer.clone()));
    Ok(buyer)
}

#[ic_cdk::update]
fn delete_buyer(id: u64) -> Result<(), Error> {
    match BUYER_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Buyer with id={} not found", id),
        }),
    }
}

// CRUD operations for Government Officials
#[ic_cdk::update]
fn add_government_official(officialidentity_id: String, name: String, department: String, email: String, phone: String) -> Result<GovernmentOfficial, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let official = GovernmentOfficial {
        id,
        officialidentity_id,
        name,
        department,
        email,
        phone,
        is_active: true,
    };

    GOV_OFFICIAL_STORAGE.with(|service| service.borrow_mut().insert(id, official.clone()));
    Ok(official)
}

// Get Government Official by official_identity_id
#[ic_cdk::query]
fn get_official(official_identity_id: u64) -> Result<GovernmentOfficial, Error> {
    match _get_official(&official_identity_id) {
        Some(official) => Ok(official),
        None => Err(Error::NotFound {
            msg: format!("Government official with official_identity_id={} not found", official_identity_id),
        }),
    }
}

#[ic_cdk::update]
fn update_government_official(id: u64, name: Option<String>, department: Option<String>, email: Option<String>, phone: Option<String>, is_active: Option<bool>) -> Result<GovernmentOfficial, Error> {
    let mut official = match GOV_OFFICIAL_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        Some(o) => o.clone(),
        None => return Err(Error::NotFound {
            msg: format!("Government official with id={} not found", id),
        }),
    };

    if let Some(n) = name {
        official.name = n;
    }
    if let Some(d) = department {
        official.department = d;
    }
    if let Some(e) = email {
        official.email = e;
    }
    if let Some(p) = phone {
        official.phone = p;
    }
    if let Some(a) = is_active {
        official.is_active = a;
    }

    GOV_OFFICIAL_STORAGE.with(|service| service.borrow_mut().insert(id, official.clone()));
    Ok(official)
}

#[ic_cdk::update]
fn delete_government_official(id: u64) -> Result<(), Error> {
    match GOV_OFFICIAL_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Government official with id={} not found", id),
        }),
    }
}

// CRUD operations for Arbitrators
#[ic_cdk::update]
fn add_arbitrator(arbitratoridentity_id: String, name: String, expertise: String, email: String, phone: String) -> Result<Arbitrator, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let arbitrator = Arbitrator {
        id,
        arbitratoridentity_id,
        name,
        expertise,
        email,
        phone,
        is_active: true,
    };

    ARBITRATOR_STORAGE.with(|service| service.borrow_mut().insert(id, arbitrator.clone()));
    Ok(arbitrator)
}

// Get Arbitrator by arbitrator_identity_id
#[ic_cdk::query]
fn get_arbitrator(arbitrator_identity_id: u64) -> Result<Arbitrator, Error> {
    match _get_arbitrator(&arbitrator_identity_id) {
        Some(arbitrator) => Ok(arbitrator),
        None => Err(Error::NotFound {
            msg: format!("Arbitrator with arbitrator_identity_id={} not found", arbitrator_identity_id),
        }),
    }
}
#[ic_cdk::update]
fn update_arbitrator(id: u64, name: Option<String>, expertise: Option<String>, email: Option<String>, phone: Option<String>, is_active: Option<bool>) -> Result<Arbitrator, Error> {
    let mut arbitrator = match ARBITRATOR_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        Some(a) => a.clone(),
        None => return Err(Error::NotFound {
            msg: format!("Arbitrator with id={} not found", id),
        }),
    };

    if let Some(n) = name {
        arbitrator.name = n;
    }
    if let Some(e) = expertise {
        arbitrator.expertise = e;
    }
    if let Some(em) = email {
        arbitrator.email = em;
    }
    if let Some(p) = phone {
        arbitrator.phone = p;
    }
    if let Some(a) = is_active {
        arbitrator.is_active = a;
    }

    ARBITRATOR_STORAGE.with(|service| service.borrow_mut().insert(id, arbitrator.clone()));
    Ok(arbitrator)
}

#[ic_cdk::update]
fn delete_arbitrator(id: u64) -> Result<(), Error> {
    match ARBITRATOR_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Arbitrator with id={} not found", id),
        }),
    }
}

// CRUD operations for Disputes
#[ic_cdk::update]
fn add_dispute(land_title_id: u64, raised_by: u64, comment: String) -> Result<Dispute, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let dispute = Dispute {
        id,
        land_title_id,
        raised_by,
        comment,
        created_at: ic_cdk::api::time(),
        resolved_at: None,
    };

    DISPUTE_STORAGE.with(|service| service.borrow_mut().insert(id, dispute.clone()));
    Ok(dispute)
}

#[ic_cdk::query]
fn get_dispute(id: u64) -> Result<Dispute, Error> {
    match DISPUTE_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(dispute) => Ok(dispute.clone()),
        None => Err(Error::NotFound {
            msg: format!("Dispute with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn update_dispute(id: u64, comment: Option<String>, resolved_at: Option<u64>) -> Result<Dispute, Error> {
    let mut dispute = match DISPUTE_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        Some(d) => d.clone(),
        None => return Err(Error::NotFound {
            msg: format!("Dispute with id={} not found", id),
        }),
    };

    if let Some(c) = comment {
        dispute.comment = c;
    }
    if let Some(r) = resolved_at {
        dispute.resolved_at = Some(r);
    }

    DISPUTE_STORAGE.with(|service| service.borrow_mut().insert(id, dispute.clone()));
    Ok(dispute)
}

#[ic_cdk::update]
fn delete_dispute(id: u64) -> Result<(), Error> {
    match DISPUTE_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Dispute with id={} not found", id),
        }),
    }
}

// CRUD operations for Transaction Records
#[ic_cdk::update]
fn add_transaction_record(land_title_id: u64, transaction_type: String, details: String) -> Result<TransactionRecord, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let transaction_record = TransactionRecord {
        id,
        land_title_id,
        transaction_type,
        timestamp: ic_cdk::api::time(),
        details,
    };

    TRANSACTION_STORAGE.with(|service| service.borrow_mut().insert(id, transaction_record.clone()));
    Ok(transaction_record)
}

#[ic_cdk::query]
fn get_transaction_record(id: u64) -> Result<TransactionRecord, Error> {
    match TRANSACTION_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(record) => Ok(record.clone()),
        None => Err(Error::NotFound {
            msg: format!("Transaction record with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn update_transaction_record(id: u64, transaction_type: Option<String>, details: Option<String>) -> Result<TransactionRecord, Error> {
    let mut transaction_record = match TRANSACTION_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        Some(record) => record.clone(),
        None => return Err(Error::NotFound {
            msg: format!("Transaction record with id={} not found", id),
        }),
    };

    if let Some(t) = transaction_type {
        transaction_record.transaction_type = t;
    }
    if let Some(d) = details {
        transaction_record.details = d;
    }

    TRANSACTION_STORAGE.with(|service| service.borrow_mut().insert(id, transaction_record.clone()));
    Ok(transaction_record)
}

#[ic_cdk::update]
fn delete_transaction_record(id: u64) -> Result<(), Error> {
    match TRANSACTION_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Transaction record with id={} not found", id),
        }),
    }
}

// Function to add a buyer identity
#[ic_cdk::update]
fn add_buyer_identity(principal: String) -> Result<BuyerIdentity, Error> {
    // Validate input data
    if principal.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Principal cannot be empty".to_string(),
        });
    }

    // Check if the principal already exists
    let exists = BUYER_IDENTITY_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .any(|(_, buyeridentity)| buyeridentity.principal == principal)
    });

    if exists {
        return Err(Error::AlreadyExists {
            msg: "Principal already exists".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let buyeridentity = BuyerIdentity { id, principal };

    BUYER_IDENTITY_STORAGE.with(|service| service.borrow_mut().insert(id, buyeridentity.clone()));
    Ok(buyeridentity)
}

// Function to get a buyer identity by its ID
#[ic_cdk::query]
fn get_buyer_identity(buyeridentity_id: u64) -> Result<BuyerIdentity, Error> {
    match BUYER_IDENTITY_STORAGE.with(|storage| storage.borrow().get(&buyeridentity_id)) {
        Some(buyeridentity) => Ok(buyeridentity.clone()),
        None => Err(Error::NotFound {
            msg: format!("BuyerIdentity with id={} not found", buyeridentity_id),
        }),
    }
}

// Function to delete a buyer identity
#[ic_cdk::update]
fn delete_buyer_identity(buyeridentity_id: u64) -> Result<(), Error> {
    match BUYER_IDENTITY_STORAGE.with(|service| service.borrow_mut().remove(&buyeridentity_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("BuyerIdentity with id={} not found", buyeridentity_id),
        }),
    }
}

// Function to add a landowner identity
#[ic_cdk::update]
fn add_landowner_identity(principal: String) -> Result<LandownerIdentity, Error> {
    // Validate input data
    if principal.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Principal cannot be empty".to_string(),
        });
    }

    // Check if the principal already exists
    let exists = LANDOWNER_IDENTITY_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .any(|(_, landowneridentity)| landowneridentity.principal == principal)
    });

    if exists {
        return Err(Error::AlreadyExists {
            msg: "Principal already exists".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let landowneridentity = LandownerIdentity { id, principal };

    LANDOWNER_IDENTITY_STORAGE.with(|service| service.borrow_mut().insert(id, landowneridentity.clone()));
    Ok(landowneridentity)
}

// Function to get a landowner identity by its ID
#[ic_cdk::query]
fn get_landowner_identity(landowneridentity_id: u64) -> Result<LandownerIdentity, Error> {
    match LANDOWNER_IDENTITY_STORAGE.with(|storage| storage.borrow().get(&landowneridentity_id)) {
        Some(landowneridentity) => Ok(landowneridentity.clone()),
        None => Err(Error::NotFound {
            msg: format!("LandownerIdentity with id={} not found", landowneridentity_id),
        }),
    }
}

// Function to delete a landowner identity
#[ic_cdk::update]
fn delete_landowner_identity(landowneridentity_id: u64) -> Result<(), Error> {
    match LANDOWNER_IDENTITY_STORAGE.with(|service| service.borrow_mut().remove(&landowneridentity_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("LandownerIdentity with id={} not found", landowneridentity_id),
        }),
    }
}

// Function to add an official identity
#[ic_cdk::update]
fn add_official_identity(principal: String) -> Result<OfficialIdentity, Error> {
    // Validate input data
    if principal.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Principal cannot be empty".to_string(),
        });
    }

    // Check if the principal already exists
    let exists = OFFICIAL_IDENTITY_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .any(|(_, officialidentity)| officialidentity.principal == principal)
    });

    if exists {
        return Err(Error::AlreadyExists {
            msg: "Principal already exists".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let officialidentity = OfficialIdentity { id, principal };

    OFFICIAL_IDENTITY_STORAGE.with(|service| service.borrow_mut().insert(id, officialidentity.clone()));
    Ok(officialidentity)
}

// Function to get an official identity by its ID
#[ic_cdk::query]
fn get_official_identity(officialidentity_id: u64) -> Result<OfficialIdentity, Error> {
    match OFFICIAL_IDENTITY_STORAGE.with(|storage| storage.borrow().get(&officialidentity_id)) {
        Some(officialidentity) => Ok(officialidentity.clone()),
        None => Err(Error::NotFound {
            msg: format!("OfficialIdentity with id={} not found", officialidentity_id),
        }),
    }
}

// Function to delete an official identity
#[ic_cdk::update]
fn delete_official_identity(officialidentity_id: u64) -> Result<(), Error> {
    match OFFICIAL_IDENTITY_STORAGE.with(|service| service.borrow_mut().remove(&officialidentity_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("OfficialIdentity with id={} not found", officialidentity_id),
        }),
    }
}

// Function to add an arbitrator identity
#[ic_cdk::update]
fn add_arbitrator_identity(principal: String) -> Result<ArbitratorIdentity, Error> {
    // Validate input data
    if principal.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Principal cannot be empty".to_string(),
        });
    }

    // Check if the principal already exists
    let exists = ARBITRATOR_IDENTITY_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .any(|(_, arbitratoridentity)| arbitratoridentity.principal == principal)
    });

    if exists {
        return Err(Error::AlreadyExists {
            msg: "Principal already exists".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let arbitratoridentity = ArbitratorIdentity { id, principal };

    ARBITRATOR_IDENTITY_STORAGE.with(|service| service.borrow_mut().insert(id, arbitratoridentity.clone()));
    Ok(arbitratoridentity)
}

// Function to get an arbitrator identity by its ID
#[ic_cdk::query]
fn get_arbitrator_identity(arbitratoridentity_id: u64) -> Result<ArbitratorIdentity, Error> {
    match ARBITRATOR_IDENTITY_STORAGE.with(|storage| storage.borrow().get(&arbitratoridentity_id)) {
        Some(arbitratoridentity) => Ok(arbitratoridentity.clone()),
        None => Err(Error::NotFound {
            msg: format!("ArbitratorIdentity with id={} not found", arbitratoridentity_id),
        }),
    }
}

// Function to delete an arbitrator identity
#[ic_cdk::update]
fn delete_arbitrator_identity(arbitratoridentity_id: u64) -> Result<(), Error> {
    match ARBITRATOR_IDENTITY_STORAGE.with(|service| service.borrow_mut().remove(&arbitratoridentity_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("ArbitratorIdentity with id={} not found", arbitratoridentity_id),
        }),
    }
}

// Helper function to retrieve LandownerIdentity by ID
fn _get_landowner_identity(landowneridentity_id: &u64) -> Option<LandownerIdentity> {
    LANDOWNER_IDENTITY_STORAGE.with(|service| service.borrow().get(landowneridentity_id))
}

// Helper function to retrieve OfficialIdentity by ID
fn _get_official_identity(officialidentity_id: &u64) -> Option<OfficialIdentity> {
    OFFICIAL_IDENTITY_STORAGE.with(|service| service.borrow().get(officialidentity_id))
}

// Helper function to retrieve ArbitratorIdentity by ID
fn _get_arbitrator_identity(arbitratoridentity_id: &u64) -> Option<ArbitratorIdentity> {
    ARBITRATOR_IDENTITY_STORAGE.with(|service| service.borrow().get(arbitratoridentity_id))
}
// Helper function to retrieve BuyerIdentity by ID
fn _get_buyer_identity(buyeridentity_id: &u64) -> Option<BuyerIdentity> {
    BUYER_IDENTITY_STORAGE.with(|service| service.borrow().get(buyeridentity_id))
}
// Helper functions to get specific entries
fn _get_land_title(land_title_id: &u64) -> Option<LandTitle> {
    LAND_TITLE_STORAGE.with(|service| service.borrow().get(land_title_id))
}

fn _get_land_transfer(land_transfer_id: &u64) -> Option<LandTransfer> {
    TRANSFER_STORAGE.with(|service| service.borrow().get(land_transfer_id))
}

// Helper function to retrieve Landowner by landowner_identity_id
fn _get_landowner(landowner_identity_id: &u64) -> Option<Landowner> {
    LANDOWNER_STORAGE.with(|service| service.borrow().get(landowner_identity_id))
}
// Helper function to retrieve Buyer by buyer_identity_id
fn _get_buyer(buyer_identity_id: &u64) -> Option<Buyer> {
    BUYER_STORAGE.with(|service| service.borrow().get(buyer_identity_id))
}

// Helper function to retrieve GovernmentOfficial by official_identity_id
fn _get_official(official_identity_id: &u64) -> Option<GovernmentOfficial> {
    GOV_OFFICIAL_STORAGE.with(|service| service.borrow().get(official_identity_id))
}

// Helper function to retrieve Arbitrator by arbitrator_identity_id
fn _get_arbitrator(arbitrator_identity_id: &u64) -> Option<Arbitrator> {
    ARBITRATOR_STORAGE.with(|service| service.borrow().get(arbitrator_identity_id))
}

fn _get_dispute(dispute_id: &u64) -> Option<Dispute> {
    DISPUTE_STORAGE.with(|service| service.borrow().get(dispute_id))
}

fn _get_transaction_record(record_id: &u64) -> Option<TransactionRecord> {
    TRANSACTION_STORAGE.with(|service| service.borrow().get(record_id))
}

// Export Candid interface
ic_cdk::export_candid!();