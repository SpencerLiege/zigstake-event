use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Timestamp, Uint128};

use crate::error::ContractError;
use crate::msg::{Choice, ExecuteMsg, Single};
use crate::state::{Bet, Config, Event, ALL_EVENTS, CONFIG, EVENT, USER_BET, USER_ROUNDS};

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // match the input message
    match msg {
        // Admin functions
        ExecuteMsg::AddEvent {
             name, 
             category,
             event_id, 
             options, 
             start_time,
             end_time 
        } => {
            execute_add_event(deps, env, info, name, event_id, category,options, start_time, end_time)
        },

        ExecuteMsg::StartEvent { event_id  } => {
            execute_start_event(deps, env, info, event_id)
        },

        ExecuteMsg::EndEvent { event_id, result } => {
            execute_end_event(deps, env, info, event_id, result)
        },

        ExecuteMsg::UpdateFee { new_fee } => {
            execute_update_fee(deps, env, info, new_fee)
        },
        
        ExecuteMsg::PlaceBet { event_id, choice, option } => {
            execute_place_bet(deps, env, info, event_id, choice, option)
        }
    }
}

// ADMIN EXECUTE FUNCTIONS

/*
@description: This function allows an admin to add new event
@access: private(ADMIN ONLY)
@params: 
    event_id: The ID to an event
    options: The available option for users to pick while placing bets
    start_time: The time where an event will be executed
    end_time: The time where an event will end and get resolved
*/
fn execute_add_event(
    deps:DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    event_id: u64,
    category: String,
    options: Vec<Single>,
    start_time: Timestamp,
    end_time: Timestamp
) -> Result<Response, ContractError> {
    // Ensure only admin calls this function
    let config: Config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {  });
    }

    // The event 
    let event: Event = Event { 
        event_id: event_id, 
        name: name.clone(),
        category: category.clone(), 
        start_time: start_time, 
        end_time: end_time, 
        resolved: false, 
        options: options, 
        total_pool: Uint128::zero(), 
        result: None, 
        executed: false, 
        participants: vec![]
    };

    // Save the event and also add it to list of events
    EVENT.save(deps.storage, event_id, &event)?;
    
    let mut  events: Vec<Event> = ALL_EVENTS.may_load(deps.storage)?
    .unwrap_or_default();

    events.push(event);

    ALL_EVENTS.save(deps.storage, &events )?;

    Ok(Response::new()
        .add_attribute("action", "add_event")
        .add_attribute("event_id", event_id.to_string())
        .add_attribute("event_name", &name)
    )

}

/*
@description: This function allows the admin to start/execute an event
@access: Private(ADMIN ONLY)
@params:
    event_id: The ID for an event
*/
fn execute_start_event(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    event_id: u64
) -> Result<Response, ContractError> {
    // Ensure access is from admin
    let config: Config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {  });
    }

    // Validate the round intended to start
    let mut event: Event = EVENT.load(deps.storage, event_id)?;

    if event.executed && event.resolved {
        return Err(ContractError::EventEndedAndResolved { event_id });
    }

    if event.executed {
        return Err(ContractError::EventExecuted { event_id });
    }

    event.executed = true;

    EVENT.save(deps.storage, event_id, &event)?;

    Ok(Response::new()
        .add_attribute("action", "start_event")
        .add_attribute("event_id", &event_id.to_string())
    )   
}

/*
@description: This function allows the admin to end and resolve an event
@access: Private(ADMIN ONLY)
@params:
    event_id: The ID for an event
*/
fn execute_end_event (
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    event_id:u64 ,
    result: Single
) -> Result<Response, ContractError> {
    // Check if access is by ADMIN
    let config: Config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {  });
    }

    EVENT.update(deps.storage, event_id, |event| ->Result<_, ContractError> {
        // Validate the round to ensure it has started or its not a round that has ended
        let mut event: Event = event.ok_or(ContractError::NotFound {  })?;

        if !event.executed {
            return Err(ContractError::EventNotStarted {  });
        }

        if event.resolved {
            return Err(ContractError::EventEnded { event_id });
        }

        event.resolved = true;
        event.result = Some(result.clone());

        // Send reward to all participants 

        Ok(event)
    })?;

    Ok(Response::new()
        .add_attribute("action", "end_event")
        .add_attribute("event_id", event_id.to_string())
        .add_attribute("result", result.name)
    )
}

/*
@description: This function allows the admin to update the treasury fee
@access: Private(ADMIN ONLY)
@params:
    new_fee: The new treasury fee
*/
fn execute_update_fee(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_fee: u64
) -> Result<Response, ContractError> {
    // Validate admin
    let mut config: Config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return  Err(ContractError::Unauthorized {  });
    }

    // Ensure the new fee is not <= 0 and < 10_000
    if new_fee <= 0 || new_fee > 10_000 {
        return Err(ContractError::IncorrectFee {  });
    }
    config.treasury_fee = new_fee;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_fee")
        .add_attribute("new_fee", new_fee.to_string()
    )
    )
}

// PUBLIC FUNCTIONS

/*
@description: This function allows a user to place a bet
@access: Private(ADMIN ONLY)
@params:
    event_id: The ID for an event
*/
fn execute_place_bet (
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    event_id: u64,
    choice: Choice,
    option: u64
) -> Result<Response, ContractError> {
    // Check if the round user is betting on exists
    if EVENT.may_load(deps.storage, event_id)?.is_none()  {
        return Err(ContractError::NotFound {  });
    }

    // Check if the round is bettable
    let mut event: Event = EVENT.load(deps.storage, event_id)?;

    if !event.executed {
        return Err(ContractError::EventNotStarted {  });
    }

    // Check if the user is already on the 
    if event.participants.contains(&info.sender) {
        return Err(ContractError::CannotPredictTwice {  });
    }

    // Check if the user sent the funds
    let bet_amount: Uint128 = info
        .funds
        .iter()
        .find(|c| c.denom == "uzig")
        .map(|c| c.amount)
        .unwrap_or_else(Uint128::zero);

    if bet_amount.is_zero() {
        return  Err(ContractError::NoBetFound {  });
    }

    // Let save user bet
    let user_bet: Bet = Bet { 
        option:  event.options[option as usize].clone(), 
        choice: choice.clone(), 
        amount: bet_amount.clone() 
    };

    USER_BET.save(deps.storage, (event_id, &info.sender), &user_bet)?;
    
    // Update the event details
    event.participants.push(info.sender.clone());
    event.total_pool += bet_amount;

    event.options[option as usize].total_pool += bet_amount;
    match choice {
        Choice::Yes => {
            event.options[option as usize].yes_pool += bet_amount;
        },
        Choice::No => {
            event.options[option as usize].no_pool += bet_amount;
        }
    }

    EVENT.save(deps.storage, event_id, &event)?;

    // Add the bet to the user bet history
    if USER_ROUNDS.may_load(deps.storage, &info.sender)?.is_none() {
        let mut new = USER_ROUNDS.load(deps.storage, &info.sender)?;
        new.push(user_bet);
    } else {
        let mut new = USER_ROUNDS.load(deps.storage, &info.sender)?;
        new.push(user_bet);
    }

    Ok(Response::new()
        .add_attribute("action", "place_bet")
        .add_attribute("event_id", event_id.to_string())
        .add_attribute("option", event.options[option as usize].name.clone())
        .add_attribute("choice", match choice {
            Choice::Yes => { "Yes" },
            Choice::No => { "No" }
        })
    )
}
