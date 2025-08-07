use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{ Addr, Timestamp, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub treasury_fee: u64,
    pub treasury: Addr
}

#[cw_serde]
pub enum ExecuteMsg {
    // Admin execute message
    StartEvent {  event_id: u64 },
    EndEvent {  event_id: u64, result: Single },
    AddEvent {
        name: String,
        event_id: u64,
        options: Vec<Single>,
        start_time: Timestamp,
        end_time: Timestamp
    },
    UpdateFee { new_fee: u64 },

    // User execute message
    PlaceBet {
        event_id: u64,
        choice: Choice,
        option: u64
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {

}

#[cw_serde]
pub enum Choice {
    Yes,
    No
}

#[cw_serde]
pub struct Single {
    pub name: String,
    pub total_pool: Uint128,
    pub yes_pool: Uint128,
    pub no_pool: Uint128
}
