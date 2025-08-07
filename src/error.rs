use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.

    #[error("Error setting treasury fee, ensure fee is within range 1-10_000 basis point")]
    IncorrectFee {},

    #[error("Cannot predict twice")]
    CannotPredictTwice {},

    #[error("Event with id: {event_id} has ended")]
    EventEnded { event_id: u64 },

    #[error("Event with id: {event_id} has been resolved")]
    EventResolved { event_id: u64},

    #[error("Event not found")]
    NotFound {},

    #[error("Event has ended and result resolved, event id: {event_id}")]
    EventEndedAndResolved { event_id: u64 },

    #[error("Event with id: {event_id} already executed")]
    EventExecuted  { event_id: u64 },

    #[error("Event not executed(starrted)")]
    EventNotStarted {},

    #[error("No bet amount found")]
    NoBetFound {}
    
}
