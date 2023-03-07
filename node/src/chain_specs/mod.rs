//! Chain Specification Definitions

// NOTE: Tolerate clippy warning originating in ChainSpecGroup, which is a dependency.
#![allow(clippy::derive_partial_eq_without_eq)]
// NOTE: Missing documentation on all `ChainSpecGroup` implementations.
#![allow(missing_docs)]

use common_primitives::{
    constants,
    types::{AccountId, Balance},
};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::{ChainType, Properties};
use serde::{Deserialize, Serialize};
use sp_core::sr25519;

pub mod wisp;

pub use self::wisp::*;
pub use wisp_runtime::currency::WSP;

/// Wisp Endowment: 10 endowment so that total supply is 10B
pub const WISP_ENDOWMENT: Balance = 1_000_000_000 * WSP;

/// Runtime Chain Spec
pub type ChainSpec = sc_service::GenericChainSpec<wisp_runtime::GenesisConfig, Extensions>;

/// The extensions for the [`ChainSpec`].
#[derive(
    ChainSpecExtension, ChainSpecGroup, Clone, Debug, Deserialize, Eq, PartialEq, Serialize,
)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}
