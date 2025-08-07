use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ Addr, Timestamp, Uint128};
use cw_storage_plus::{ Map, Item };
use crate::msg::{Choice, Single};

#[cw_serde]
pub struct  Config {
    pub admin: Addr,
    pub treasury_fee: u64,
    pub treasury: Addr
}

#[cw_serde]
pub struct Event {
    pub event_id: u64,
    pub name: String,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub resolved: bool,
    pub options: Vec<Single>,
    pub total_pool: Uint128,
    pub result: Option<Single>,
    pub executed: bool,
    pub participants: Vec<Addr>
}

#[cw_serde]
pub struct Bet {
    pub option: Single,
    pub choice: Choice,
    pub amount: Uint128
}

// Save the admin configuratuions
pub const CONFIG: Item<Config> = Item::new("config");

// Map an event to an event ID
pub const EVENT: Map<u64, Event> = Map::new("event");

// Add a list of all events
pub const ALL_EVENTS: Item<Vec<Event>> = Item::new("all_events");

// Map each user bet to an event
pub const USER_BET: Map<(u64, &Addr), Bet> = Map::new("user_bet");

// Map the user to list of rounds played]
pub const  USER_ROUNDS: Map<&Addr, Vec<Event>> = Map::new("user_rounds");

// Create the leaderboard