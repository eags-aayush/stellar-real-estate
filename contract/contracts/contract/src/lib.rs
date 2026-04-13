#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

// We use an enum for data keys to keep storage organized and avoid collisions.
#[contracttype]
pub enum DataKey {
    Property(u32),
}

#[contract]
pub struct RealEstateRegistry;

#[contractimpl]
impl RealEstateRegistry {
    
    /// Registers a new property to an owner.
    /// Fails if the property ID already exists.
    pub fn register(env: Env, property_id: u32, owner: Address) {
        let key = DataKey::Property(property_id);
        
        if env.storage().persistent().has(&key) {
            panic!("Property is already registered");
        }
        
        env.storage().persistent().set(&key, &owner);
    }

    /// Transfers ownership of a property.
    /// Requires authorization from the current owner.
    pub fn transfer(env: Env, property_id: u32, current_owner: Address, new_owner: Address) {
        // Enforce that the current owner has cryptographically signed this transaction
        current_owner.require_auth();
        
        let key = DataKey::Property(property_id);
        
        // Retrieve the current owner from storage, panic if it doesn't exist
        let stored_owner: Address = env.storage().persistent().get(&key).expect("Property not found");
        
        // Ensure the person trying to transfer is the actual on-chain owner
        if current_owner != stored_owner {
            panic!("Caller is not the property owner");
        }

        // Update the storage with the new owner
        env.storage().persistent().set(&key, &new_owner);
    }

    /// Read-only function to get the current owner of a property.
    pub fn get_owner(env: Env, property_id: u32) -> Address {
        let key = DataKey::Property(property_id);
        env.storage().persistent().get(&key).expect("Property not found")
    }
}