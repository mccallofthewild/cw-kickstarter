use cosmwasm_std::{coin, Addr, Deps, Env, Order, StdResult, Uint128};
use cw_storage_plus::Bound;

use crate::{
    msg::{
        GetConfigResponse, GetFundersResponse, GetSharesResponse, GetTotalFundsResponse,
        QueryResponseWrapper,
    },
    state::State,
};

pub fn get_config(deps: Deps, _env: Env) -> StdResult<QueryResponseWrapper> {
    let state = State::default();
    let config = state.config.load(deps.storage)?;
    Ok(QueryResponseWrapper::GetConfigResponse(GetConfigResponse {
        goal: coin(config.goal.u128(), config.denom),
        deadline: config.deadline,
        name: config.name,
        description: config.description,
    }))
}

pub fn get_shares(deps: Deps, _env: Env, address: String) -> StdResult<QueryResponseWrapper> {
    let state = State::default();
    let addr = deps.api.addr_validate(&address)?;
    let shares = state.shares.load(deps.storage, addr)?;
    Ok(QueryResponseWrapper::GetSharesResponse(GetSharesResponse {
        shares,
        address,
    }))
}

pub fn get_funders(
    deps: Deps,
    _env: Env,
    limit: Uint128,
    start_after: Option<String>,
) -> StdResult<QueryResponseWrapper> {
    let state = State::default();
    let start = start_after
        .map(|s| deps.api.addr_validate(&s))
        .transpose()?
        .map(|addr| Bound::InclusiveRaw::<Addr>(addr.as_bytes().to_vec()));
    let funders = state
        .shares
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit.u128() as usize)
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|(addr, shares)| (addr.to_string(), *shares))
        .collect::<Vec<(String, Uint128)>>();
    Ok(QueryResponseWrapper::GetFundersResponse(
        GetFundersResponse { funders },
    ))
}

pub fn get_funds(deps: Deps, _env: Env) -> StdResult<QueryResponseWrapper> {
    let state = State::default();
    let funds = state.total_shares.load(deps.storage)?;
    let config = state.config.load(deps.storage)?;
    Ok(QueryResponseWrapper::GetTotalFundsResponse(
        GetTotalFundsResponse {
            total_funds: coin(funds.u128(), config.denom),
        },
    ))
}
