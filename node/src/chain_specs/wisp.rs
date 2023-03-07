//! Parachain Chain Specifications

use super::*;
use crate::command::WISP_PARACHAIN_ID;
use session_key_primitives::util::unchecked_account_id;
use wisp_runtime::{
    opaque::SessionKeys, CouncilConfig, DemocracyConfig, GenesisConfig, TechnicalCommitteeConfig,
};

/// Parachain Protocol Identifier
pub const WISP_PROTOCOL_ID: &str = "wisp";

/// Kusama Relaychain Local Network Identifier
pub const KUSAMA_RELAYCHAIN_LOCAL_NET: &str = "kusama-local";

/// Kusama Relaychain Development Network Identifier
pub const KUSAMA_RELAYCHAIN_DEV_NET: &str = "kusama-dev";

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = 2;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type WispChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Returns the [`Properties`] for the Wisp parachain.
pub fn wisp_properties() -> Properties {
    let mut p = Properties::new();
    p.insert("ss58format".into(), constants::WISP_SS58PREFIX.into());
    p.insert("tokenDecimals".into(), constants::WISP_DECIMAL.into());
    p.insert("tokenSymbol".into(), constants::WISP_TOKEN_SYMBOL.into());
    p
}

/// Returns the Wisp development chainspec.
pub fn wisp_development_config() -> WispChainSpec {
    WispChainSpec::from_genesis(
        "Wisp Parachain Development",
        "wisp_dev",
        ChainType::Local,
        move || {
            wisp_dev_genesis(
                vec![(
                    unchecked_account_id::<sr25519::Public>("Alice"),
                    SessionKeys::from_seed_unchecked("Alice"),
                )],
                unchecked_account_id::<sr25519::Public>("Alice"),
                vec![
                    unchecked_account_id::<sr25519::Public>("Alice"),
                    unchecked_account_id::<sr25519::Public>("Bob"),
                    unchecked_account_id::<sr25519::Public>("Alice//stash"),
                    unchecked_account_id::<sr25519::Public>("Bob//stash"),
                ],
            )
        },
        vec![],
        None,
        Some(WISP_PROTOCOL_ID),
        None,
        Some(wisp_properties()),
        Extensions {
            relay_chain: "".into(),
            para_id: WISP_PARACHAIN_ID,
        },
    )
}

/// Returns the Wisp local chainspec.
pub fn wisp_local_config(localdev: bool) -> WispChainSpec {
    let id = if localdev {
        "wisp_localdev"
    } else {
        "wisp_local"
    };
    WispChainSpec::from_genesis(
        "Wisp Parachain Local",
        id,
        ChainType::Local,
        move || {
            let invulnerables = if localdev {
                vec![(
                    unchecked_account_id::<sr25519::Public>("Alice"),
                    SessionKeys::from_seed_unchecked("Alice"),
                )]
            } else {
                vec![
                    (
                        unchecked_account_id::<sr25519::Public>("Alice"),
                        SessionKeys::from_seed_unchecked("Alice"),
                    ),
                    (
                        unchecked_account_id::<sr25519::Public>("Bob"),
                        SessionKeys::from_seed_unchecked("Bob"),
                    ),
                    (
                        unchecked_account_id::<sr25519::Public>("Charlie"),
                        SessionKeys::from_seed_unchecked("Charlie"),
                    ),
                    (
                        unchecked_account_id::<sr25519::Public>("Dave"),
                        SessionKeys::from_seed_unchecked("Dave"),
                    ),
                    (
                        unchecked_account_id::<sr25519::Public>("Eve"),
                        SessionKeys::from_seed_unchecked("Eve"),
                    ),
                ]
            };
            wisp_dev_genesis(
                invulnerables,
                unchecked_account_id::<sr25519::Public>("Alice"),
                vec![
                    unchecked_account_id::<sr25519::Public>("Alice"),
                    unchecked_account_id::<sr25519::Public>("Bob"),
                    unchecked_account_id::<sr25519::Public>("Charlie"),
                    unchecked_account_id::<sr25519::Public>("Dave"),
                    unchecked_account_id::<sr25519::Public>("Eve"),
                    unchecked_account_id::<sr25519::Public>("Alice//stash"),
                    unchecked_account_id::<sr25519::Public>("Bob//stash"),
                    unchecked_account_id::<sr25519::Public>("Charlie//stash"),
                    unchecked_account_id::<sr25519::Public>("Dave//stash"),
                    unchecked_account_id::<sr25519::Public>("Eve//stash"),
                ],
            )
        },
        vec![],
        None,
        Some(WISP_PROTOCOL_ID),
        None,
        Some(wisp_properties()),
        Extensions {
            relay_chain: "".into(),
            para_id: WISP_PARACHAIN_ID,
        },
    )
}

fn wisp_dev_genesis(
    invulnerables: Vec<(AccountId, SessionKeys)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
    GenesisConfig {
        system: wisp_runtime::SystemConfig {
            code: wisp_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
        },
        balances: wisp_runtime::BalancesConfig {
            balances: endowed_accounts[..endowed_accounts.len() / 2]
                .iter()
                .map(|k| {
                    (
                        k.clone(),
                        100 * WISP_ENDOWMENT / ((endowed_accounts.len() / 2) as Balance),
                    )
                })
                .collect(),
        },
        // no need to pass anything to aura, in fact it will panic if we do. Session will take care
        // of this.
        aura: Default::default(),
        sudo: wisp_runtime::SudoConfig {
            key: Some(root_key),
        },
        parachain_info: wisp_runtime::ParachainInfoConfig {
            parachain_id: WISP_PARACHAIN_ID.into(),
        },
        collator_selection: wisp_runtime::CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: WSP * 1000, // How many tokens will be reserved as collator
            ..Default::default()
        },
        session: wisp_runtime::SessionConfig {
            keys: invulnerables
                .iter()
                .cloned()
                .map(|(acc, session_keys)| {
                    (
                        acc.clone(),  // account id
                        acc,          // validator id
                        session_keys, // collator session keys
                    )
                })
                .collect(),
        },
        democracy: DemocracyConfig::default(),
        council: CouncilConfig {
            members: endowed_accounts.iter().take(1).cloned().collect(),
            phantom: Default::default(),
        },
        technical_committee: TechnicalCommitteeConfig {
            members: endowed_accounts.iter().take(1).cloned().collect(),
            phantom: Default::default(),
        },
        asset_manager: Default::default(),
        council_membership: Default::default(),
        technical_membership: Default::default(),
        parachain_system: Default::default(),
        polkadot_xcm: wisp_runtime::PolkadotXcmConfig {
            safe_xcm_version: Some(SAFE_XCM_VERSION),
        },
    }
}

// /// Returns the Wisp testnet chainspec.
// pub fn wisp_testnet_config() -> Result<WispChainSpec, String> {
//     let mut spec = WispChainSpec::from_json_bytes(
//         &include_bytes!("../../../genesis/wisp-testnet-genesis.json")[..],
//     )?;
//     spec.extensions_mut().para_id = PARACHAIN_ID;
//     Ok(spec)
// }

// pub fn wisp_2085_config() -> Result<WispChainSpec, String> {
//     let mut spec = WispChainSpec::from_json_bytes(
//         &include_bytes!("../../../genesis/wisp-2085-genesis.json")[..],
//     )?;
//     spec.extensions_mut().para_id = WISP_ON_BAIKAL_PARACHAIN_ID;
//     Ok(spec)
// }

// /// Returns the Wisp V3 2085 staging chainspec.
// pub fn wisp_v3_2085_staging_config() -> Result<WispChainSpec, String> {
//     let mut spec = WispChainSpec::from_json_bytes(
//         &include_bytes!("../../../genesis/wisp-v3-2085-genesis.json")[..],
//     )?;
//     spec.extensions_mut().para_id = 9997;
//     Ok(spec)
// }
