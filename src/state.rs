use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, CosmosMsg, StdError, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub denom: String,
    pub goal: Uint128,
    pub start: Timestamp,
    pub deadline: Timestamp,
    pub name: String,
    pub description: String,
}

impl Config {
    pub fn validate(&self) -> Result<(), StdError> {
        if self.goal <= Uint128::zero() {
            return Err(StdError::generic_err(
                "goal must be greater than 0".to_string(),
            ));
        }
        if self.start >= self.deadline {
            return Err(StdError::generic_err(
                "start must be before deadline".to_string(),
            ));
        }
        // description must be less than 256 characters
        if self.description.len() > 256 {
            return Err(StdError::generic_err(
                "description must be less than 256 characters".to_string(),
            ));
        }
        // title must be less than 32 characters
        if self.name.len() > 32 {
            return Err(StdError::generic_err(
                "title must be less than 32 characters".to_string(),
            ));
        }
        Ok(())
    }
}

pub struct State<'a> {
    pub config: Item<'a, Config>,
    pub shares: Map<'a, Addr, Uint128>,
    pub total_shares: Item<'a, Uint128>,
    pub execute_msg: Item<'a, Option<CosmosMsg>>,
}

impl Default for State<'_> {
    fn default() -> Self {
        State {
            config: Item::new("config"),
            shares: Map::new("shares"),
            total_shares: Item::new("total_shares"),
            execute_msg: Item::new("execute_msg"),
        }
    }
}

pub const STATE: Item<State> = Item::new("state");
