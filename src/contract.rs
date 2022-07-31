#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryResponseWrapper};
use crate::state::{Config, State};
use crate::{execute, queries, rules};

const CONTRACT_NAME: &str = "crates.io:cw-kickstarter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let state = State::default();
    let config = Config {
        owner: env.contract.address,
        denom: msg.denom,
        goal: msg.goal,
        start: msg.start.unwrap_or(env.block.time),
        deadline: msg.deadline,
        name: msg.name,
        description: msg.description,
    };
    config.validate()?;
    state.config.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "cw_kickstarter/instantiate")
        .add_attribute("name", config.name))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;
    match msg {
        Fund {} => {
            let state = State::default();
            rules::HAS_STARTED(&deps, &env, &info, &state)?;
            rules::NOT_CLOSED(&deps, &env, &info, &state)?;
            rules::SENT_FUNDS(&deps, &env, &info, &state)?;
            execute::fund(deps, env, info)
        }
        Execute {} => {
            let state = State::default();
            rules::IS_CLOSED(&deps, &env, &info, &state)?;
            rules::FULLY_FUNDED(&deps, &env, &info, &state)?;
            execute::execute(deps, env, info)
        }
        Claim {} => {
            let state = State::default();
            rules::IS_CLOSED(&deps, &env, &info, &state)?;
            rules::NOT_FULLY_FUNDED(&deps, &env, &info, &state)?;
            execute::claim(deps, env, info)
        }
        Refund {} => {
            let state = State::default();
            rules::IS_CLOSED(&deps, &env, &info, &state)?;
            rules::NOT_FULLY_FUNDED(&deps, &env, &info, &state)?;
            execute::refund(deps, env, info)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let output: StdResult<QueryResponseWrapper> = match msg {
        QueryMsg::GetConfig {} => queries::get_config(deps, env),
        QueryMsg::GetShares { user } => queries::get_shares(deps, env, user),
        QueryMsg::GetFunders { limit, start_after } => {
            queries::get_funders(deps, env, limit, start_after)
        }
        QueryMsg::GetTotalFunds {} => queries::get_funds(deps, env),
    };
    output?.to_binary()
}

#[cfg(test)]
mod tests {}
