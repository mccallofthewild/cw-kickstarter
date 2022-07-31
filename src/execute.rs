use cosmwasm_std::{
    coins, BankMsg, CosmosMsg, DepsMut, Empty, Env, MessageInfo, Order, Response, StdError, Uint128,
};

use crate::{state::State, ContractError};

pub fn fund(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let state = State::default();
    let config = state.config.load(deps.storage)?;
    let sent_funds = info
        .funds
        .iter()
        .find_map(|v| {
            if v.denom == config.denom {
                Some(v.amount)
            } else {
                None
            }
        })
        .unwrap_or_else(Uint128::zero);
    state
        .shares
        .update::<_, StdError>(deps.storage, info.sender, |shares| {
            let mut shares = shares.unwrap_or_default();
            shares += sent_funds;
            Ok(shares)
        })?;
    state
        .total_shares
        .update::<_, StdError>(deps.storage, |total_shares| {
            let mut total_shares = total_shares;
            total_shares += sent_funds;
            Ok(total_shares)
        })?;
    Ok(Response::new())
}

pub fn execute(deps: DepsMut, _env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    let state = State::default();
    let execute_msg = state
        .execute_msg
        .load(deps.storage)?
        .ok_or_else(|| StdError::generic_err("execute_msg not set".to_string()))?;
    // execute can only run once ever.
    state.execute_msg.save(deps.storage, &None)?;
    Ok(Response::new().add_message(execute_msg))
}

pub fn refund(deps: DepsMut, env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    let state = State::default();
    let config = state.config.load(deps.storage)?;
    let contract_balance = deps
        .querier
        .query_balance(env.contract.address, config.denom.clone())?
        .amount;
    let total_shares = state.total_shares.load(deps.storage)?;
    let user_shares = state
        .shares
        .range(deps.storage, None, None, Order::Ascending)
        // batch execute 30 transfers at a time
        .take(30)
        .collect::<Result<Vec<_>, _>>()?;
    let mut next_shares = total_shares;
    let msgs: Vec<CosmosMsg> = vec![];
    for (addr, shares) in user_shares {
        let refund_amount = contract_balance.multiply_ratio(shares, total_shares);
        let _bank_transfer_msg = CosmosMsg::<Empty>::Bank(BankMsg::Send {
            to_address: addr.to_string(),
            amount: coins(refund_amount.u128(), config.denom.clone()),
        });
        state.shares.remove(deps.storage, addr);
        next_shares -= shares;
    }
    state.total_shares.save(deps.storage, &next_shares)?;
    Ok(Response::new().add_messages(msgs))
}

pub fn claim(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let state = State::default();
    let config = state.config.load(deps.storage)?;
    let contract_balance = deps
        .querier
        .query_balance(env.contract.address, config.denom.clone())?
        .amount;
    let total_shares = state.total_shares.load(deps.storage)?;
    let user_shares = state.shares.load(deps.storage, info.sender.clone())?;
    let mut next_total_shares = total_shares;
    let refund_amount = contract_balance.multiply_ratio(user_shares, total_shares);
    let bank_transfer_msg = CosmosMsg::<Empty>::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: coins(refund_amount.u128(), config.denom),
    });
    state.shares.remove(deps.storage, info.sender);
    next_total_shares -= user_shares;
    state.total_shares.save(deps.storage, &next_total_shares)?;
    Ok(Response::new().add_message(bank_transfer_msg))
}
