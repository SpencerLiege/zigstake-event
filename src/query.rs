use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Env, StdResult, StdError};

use crate::state::{ ALL_EVENTS, EVENT, Event, Bet, USER_ROUNDS, USER_BET};
use crate::msg::{QueryMsg};

pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetEventDetails { event_id } => {
            to_json_binary(&query_event_details(deps, event_id)?)
        },
        QueryMsg::GetAllEventDetails {  } => {
            to_json_binary(&query_all_events(deps)?)
        },
        QueryMsg::GetBetDetails { user, event_id } => {
            to_json_binary(&query_bet_details(deps, user, event_id)?)
        },
        QueryMsg::GetAllUserBetDetails { user } => {
            to_json_binary(&query_all_user_bets(deps, user)?)
        }
    }
}

// PUBLIC VIEW FUNCTIONS
/*
@description: This function queries an event detail
@params:
    event_id: The ID to the event
*/
fn query_event_details (
    deps: Deps,
    event_id: u64
) -> StdResult<Event> {
    // Ensure the round to query is available
    if EVENT.may_load(deps.storage, event_id)?.is_none(){
        return Err(StdError::generic_err("Could not find event"));
    }

    let event: Event = EVENT.load(deps.storage, event_id)?;
    Ok(event)
}

/*
@description: This function fetches all the events(can be sort to included required)
@params:
    None
*/
fn query_all_events(
    deps: Deps
) -> StdResult<Vec<Event>> {
    let events = ALL_EVENTS.load(deps.storage)?;

    Ok(events)
}

// USER VIEW FUNCTIONS
/*
@description: This function fetches a user bet info
@params: 
    event_id: The event ID
    user: The user address
*/
fn query_bet_details (
    deps: Deps,
    user: String,
    event_id: u64
) -> StdResult<Bet> {
    // validate the user address
    let validated_addr = deps.api.addr_validate(&user).unwrap();
    // check if the bet is available
    let key = (event_id, &validated_addr);

    let bet: Bet = USER_BET.load(deps.storage, key)?;

    Ok(bet)
}

fn query_all_user_bets (
    deps: Deps,
    user: String
) -> StdResult<Vec<Bet>> {
    // Validate the user address
    let validated_add = deps.api.addr_validate(&user).unwrap();
    let bets = USER_ROUNDS.load(deps.storage, &validated_add )?;

    Ok(bets)
}