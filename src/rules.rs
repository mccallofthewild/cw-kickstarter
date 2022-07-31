use cosmwasm_std::{DepsMut, Env, MessageInfo, StdError, Uint128};

use crate::state::State;

type Rule =
    fn(deps: &DepsMut, env: &Env, info: &MessageInfo, state: &State) -> Result<(), StdError>;

pub const HAS_STARTED: Rule = |deps, env, _info, state| {
    if state.config.load(deps.storage)?.start >= env.block.time {
        return Err(StdError::generic_err(
            "project has not started yet".to_string(),
        ));
    }
    Ok(())
};

pub const NOT_CLOSED: Rule = |deps, env, _info, state| {
    if state.config.load(deps.storage)?.deadline <= env.block.time {
        return Err(StdError::generic_err("Project is closed"));
    }
    Ok(())
};

pub const SENT_FUNDS: Rule = |deps, _env, info, state| {
    let denom = state.config.load(deps.storage)?.denom;
    if info
        .funds
        .iter()
        .find_map(|v| {
            if v.denom == denom {
                Some(v.amount)
            } else {
                None
            }
        })
        .unwrap_or_else(Uint128::zero)
        .is_zero()
    {
        return Err(StdError::generic_err("Amount must be positive"));
    }
    Ok(())
};

pub const FULLY_FUNDED: Rule = |deps, _env, _info, state| {
    let config = state.config.load(deps.storage)?;
    let goal = config.goal;
    let _denom = config.denom;
    let total_shares = state.total_shares.load(deps.storage)?;
    if total_shares < goal {
        return Err(StdError::generic_err(format!(
            "Project must be fully funded: {} < {}",
            total_shares, goal
        )));
    }
    Ok(())
};

pub const IS_CLOSED: Rule = |deps, env, _info, state| {
    if state.config.load(deps.storage)?.deadline > env.block.time {
        return Err(StdError::generic_err("Project is open"));
    }
    Ok(())
};

pub const NOT_FULLY_FUNDED: Rule = |deps, _env, _info, state| {
    let config = state.config.load(deps.storage)?;
    let goal = config.goal;
    let total_shares = state.total_shares.load(deps.storage)?;
    if total_shares >= goal {
        return Err(StdError::generic_err(format!(
            "Project must not be fully funded: {} >= {}",
            total_shares, goal
        )));
    }
    Ok(())
};
