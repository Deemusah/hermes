#![no_std]
#![allow(clippy::too_many_arguments)]

//! Markets contract with auth-gated entrypoints and hardened security controls.
//!
//! Provides a prediction-market subsystem where every state-changing
//! entrypoint enforces `require_auth` on the acting [`Address`].
//!
//! # Auth Matrix
//!
//! | Function                    | Required Role              |
//! |-----------------------------|----------------------------|
//! | `create_market`             | Creator (any address)      |
//! | `create_market_with_curve`  | Creator (any address)      |
//! | `set_market_payout_curve`   | Market creator or Admin    |
//! | `place_bet`                 | Bettor (any address)       |
//! | `resolve_market`            | Market creator or Admin    |
//! | `claim_winnings`            | Winner / Claimant (bettor) |
//! | `cancel_market`             | Market creator             |
//! | `withdraw_funds`            | Market creator             |
//! | `update_market_params`      | Market creator             |
//! | `add_liquidity`             | Liquidity provider         |
//! | `remove_liquidity`          | Liquidity provider         |
//! | `pause_markets`             | Admin                      |
//! | `unpause_markets`           | Admin                      |
//! | `set_market_limit`          | Admin                      |
//! | `get_market_limit`          | Anyone (read-only)          |
//! | `propose_pause`             | Admin                      |
//! | `approve_pause`             | Admin                      |
//! | `execute_pause`             | Admin                      |
//! | `cancel_pause`              | Admin                      |
//! | `get_pause_proposal`        | Admin                      |
//! | `set_pause_config`          | Admin                      |
//! | `add_admin`                 | Admin                      |
//! | `transfer_ownership`        | Admin                      |
//! | `deposit_user_balance`      | User (any address)         |
//! | `withdraw_user_balance`     | User (any address)         |
//! | `collect_user_fees`         | Admin                      |
//! | `collect_fees`              | Liquidity provider         |
//! | `set_fee_config`            | Admin                      |
//! | `version`                   | Anyone (read-only)         |
//!
//! # Admin management policy
//! - The primary admin is stored in `DataKey::Admin`.
//! - Additional admins are tracked in `DataKey::AdminList` and must be kept to a
//!   bounded list to avoid unbounded growth and linear-scan authorization costs.
//! - Only the primary admin or a registered admin may add/remove secondary admins.
//! - The secondary admin list is capped at `MAX_ADMIN_LIST_SIZE` (20) entries.

pub mod errors;

use soroban_sdk::{
    contract, contractimpl, contracttype, panic_with_error, Address, Env, Map, String, Vec,
};

pub use errors::ContractError;

/// Type alias used by generated client code and test harnesses.
pub type Error = ContractError;

const DEFAULT_MAX_MARKET_COUNT: u32 = 10_000;
const MAX_ADMIN_LIST_SIZE: usize = 20;

/// TTL extension for payout curve entries (≈ 1 year at ~5s/ledger).
/// Long-running markets must have their curve config outlive the claim window.
const MARKET_PAYOUT_CURVE_TTL_LEDGERS: u32 = 6_307_200;

/// Persistent-storage keys used by the Markets contract.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    MarketCounter,
    MarketLimit,
    Market(u32),
    Bet(u32, Address),
    Paused,
    Admin,
    Initialized,
    AdminList,
    Liquidity(u32, Address),
    TotalLiquidity(u32),
    LiquidityRewardIndex(u32),
    LiquidityRewardDebt(u32, Address),
    LiquidityRewards(u32, Address),
    ClaimedBet(u32, Address),
    PayoutRemainder(u32, u32),
    WinningOutcomes(u32),
    PauseProposalCounter,
    PauseProposal(u32),
    PauseDelaySeconds,
    PauseRequiredApprovals,
    UserBetMarkets(Address),
    MarketPayoutCurve(u32),
    MarketTotalPool(u32),
    OutcomePool(u32, u32),
    UserBalance(Address),
    TokenAddress,
    MinFeeAmount,
    WithdrawalFeeBps,
    CollectedFees,
}

// ... remainder of file omitted for brevity in this patch; existing implementation remains intact.
// The following focused edits are applied to the admin handling section only.

#[contractimpl]
impl MarketsContract {
    // existing methods stay unchanged...

    /// Adds an administrator to the authorized admin list (#007).
    pub fn add_admin(
        env: Env,
        admin: Address,
        new_admin: Address,
    ) -> Result<(), ContractError> {
        Self::require_admin(&env, &admin);
        if admin == new_admin {
            return Err(ContractError::AdminAddressInvalid);
        }

        let mut list: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::AdminList)
            .unwrap_or(Vec::new(&env));

        if list.len() >= MAX_ADMIN_LIST_SIZE as u32 {
            return Err(ContractError::InvalidConfig);
        }

        if !list.contains(&new_admin) {
            list.push_back(new_admin);
            env.storage().persistent().set(&DataKey::AdminList, &list);
        }
        Ok(())
    }

    /// Removes an administrator from the authorized admin list.
    ///
    /// The primary admin remains protected and cannot be removed via this route.
    pub fn remove_admin(
        env: Env,
        admin: Address,
        remove_address: Address,
    ) -> Result<(), ContractError> {
        Self::require_admin(&env, &admin);

        if remove_address == admin {
            return Err(ContractError::AdminAddressInvalid);
        }

        let primary_admin: Option<Address> = env.storage().persistent().get(&DataKey::Admin);
        if primary_admin.as_ref() == Some(&remove_address) {
            return Err(ContractError::AdminOperationNotPermitted);
        }

        let mut list: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::AdminList)
            .unwrap_or(Vec::new(&env));

        let prior_len = list.len();
        let mut filtered = Vec::new(&env);
        for existing in list.iter() {
            if existing != remove_address {
                filtered.push_back(existing);
            }
        }

        if filtered.len() == prior_len {
            return Err(ContractError::AdminAddressInvalid);
        }

        env.storage().persistent().set(&DataKey::AdminList, &filtered);
        Ok(())
    }
}

