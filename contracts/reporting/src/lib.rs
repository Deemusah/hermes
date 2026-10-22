//! Reporting contract with admin-gated pause/resume mechanism.
//!
//! Provides a reporting subsystem that allows authorised reporters to submit
//! reports, admins to verify/dispute/resolve them, and an emergency pause
//! switch that halts all state-changing operations while preserving read
//! access.
//!
//! # Pause / Resume
//!
//! The pause mechanism is implemented in the [`pause`] module. When reporting
//! is paused, all state-changing entrypoints revert with
//! [`ReportingError::ReportingPaused`]. The read-only [`ReportingContract::is_reporting_paused`]
//! view is unaffected.
//!
//! # Auth Matrix
//!
//! | Function | Required Role |
//! |---|---|
//! | `initialize` | Admin (bootstrapper) |
//! | `submit_report` | Reporter |
//! | `verify_report` | Admin |
//! | `dispute_report` | Reporter |
//! | `resolve_dispute` | Admin |
//! | `update_report_status` | Admin |
//! | `delete_report` | Admin |
//! | `pause_reporting` | Admin |
//! | `unpause_reporting` | Admin |
//! | `transfer_ownership` | Admin |
//! | `is_reporting_paused` | Anyone (read-only) |
//! | `admin` | Anyone (read-only) |

#![no_std]

mod err;
mod events;
mod pause;
mod types;

pub use err::ReportingError;
pub use types::{DataKey, Report};

use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

/// The Reporting contract.
#[contract]
pub struct ReportingContract;

#[contractimpl]
impl ReportingContract {
    // -----------------------------------------------------------------------
    // Initialisation
    // -----------------------------------------------------------------------

    /// Initialise the contract, recording `admin` as the sole administrator.
    ///
    /// # Parameters
    ///
    /// * `admin` — The address that will hold admin privileges. Must sign the
    ///   transaction (`require_auth` is enforced).
    ///
    /// # Errors
    ///
    /// * [`ReportingError::AlreadyInitialized`] — Contract has already been
    ///   initialised. Call is idempotent-safe: clients should check before
    ///   calling rather than relying on the error.
    ///
    /// # Auth
    ///
    /// Requires a valid signature from `admin`.
    pub fn initialize(env: Env, admin: Address) -> Result<(), ReportingError> {
        admin.require_auth();

        if env.storage().instance().has(&DataKey::Initialized) {
            return Err(ReportingError::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Initialized, &true);

        // The reporting contract currently does not implement a dispute workflow of its own.
        // Disputes are handled by the dedicated contracts/disputes subsystem.
        env.storage().persistent().set(&DataKey::NextReportId, &1u32);

        Ok(())
    }

    // ... remaining file unchanged ...
}

