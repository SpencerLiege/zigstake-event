use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};
use crate::msg::{InstantiateMsg};
use crate::state::{Config, CONFIG };
use crate::error::ContractError;

pub fn instantiate(
    deps: DepsMut,
    _env:Env,
    info: MessageInfo,
    msg: InstantiateMsg,
    address: Addr
) -> Result<Response, ContractError> {
    // Validate the treasury fee, ensure it is <10_000 
    if msg.treasury_fee > 10_000 || msg.treasury_fee <= 0 {
        return Err(ContractError::IncorrectFee {  });
    }

    let config: Config = Config{
        admin: info.sender.clone(),
        treasury_fee: msg.treasury_fee,
        treasury: address
    };

    CONFIG.save(deps.storage, &config)?;

    // log the event 
    Ok(Response::new()
        .add_attribute("action", "instantiate_contract")
        .add_attribute("admin", info.sender.clone())
        .add_attribute("treasury_fee", msg.treasury_fee.to_string())
    )
}