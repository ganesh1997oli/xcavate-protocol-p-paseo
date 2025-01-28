use cumulus_primitives_core::ParaId;
use xcavate_parachain_runtime as runtime;
use runtime::{AccountId, AuraId, EXISTENTIAL_DEPOSIT};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::crypto::Ss58Codec;
use crate::constants::xcavate;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	#[serde(alias = "relayChain", alias = "RelayChain")]
	pub relay_chain: String,
	/// The id of the Parachain.
	#[serde(alias = "paraId", alias = "ParaId")]
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> runtime::SessionKeys {
	runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), xcavate::TOKEN_SYMBOL.into());
    properties.insert("tokenDecimals".into(), xcavate::TOKEN_DECIMALS.into());
    properties.insert("ss58Format".into(), xcavate::SS58_FORMAT.into());

	// richard
    let collator_0_account_id: AccountId =
        AccountId::from_ss58check("1xGrVAzbHfUfxFMRhUsDKWFcJNf1XMSNE2UqRjkYyFhrnqt").unwrap();
    let collator_0_aura_id: AuraId =
        AuraId::from_ss58check("1xGrVAzbHfUfxFMRhUsDKWFcJNf1XMSNE2UqRjkYyFhrnqt").unwrap();
    // alex
    let collator_1_account_id: AccountId =
        AccountId::from_ss58check("1uWNn87BVmATvKRHW6ptAdhTBaYgYBakeKhJnRYCCPJJGaY").unwrap();
    let collator_1_aura_id: AuraId =
        AuraId::from_ss58check("1uWNn87BVmATvKRHW6ptAdhTBaYgYBakeKhJnRYCCPJJGaY").unwrap();

	ChainSpec::builder(
		runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions {
            relay_chain: xcavate::RELAY_CHAIN.into(),
            // You MUST set this to the correct network!
            para_id: xcavate::PARACHAIN_ID,
		},
	)
	.with_name("Development")
	.with_id("dev")
	.with_chain_type(ChainType::Development)
	.with_genesis_config_patch(testnet_genesis(
		// initial collators.
        vec![
            // XCAVATE COLLATOR 0
            (collator_0_account_id, collator_0_aura_id),
            // XCAVATE COLLATOR 1
            (collator_1_account_id, collator_1_aura_id),
        ],
		get_endowed_accounts_with_balance(),
		get_root_account(),
		xcavate::PARACHAIN_ID.into(),
	))
	.with_protocol_id("template-dev")
    .with_properties(properties)
	.build()
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), xcavate::TOKEN_SYMBOL.into());
    properties.insert("tokenDecimals".into(), xcavate::TOKEN_DECIMALS.into());
    properties.insert("ss58Format".into(), xcavate::SS58_FORMAT.into());

	// richard
    let collator_0_account_id: AccountId =
        AccountId::from_ss58check("1xGrVAzbHfUfxFMRhUsDKWFcJNf1XMSNE2UqRjkYyFhrnqt").unwrap();
    let collator_0_aura_id: AuraId =
        AuraId::from_ss58check("1xGrVAzbHfUfxFMRhUsDKWFcJNf1XMSNE2UqRjkYyFhrnqt").unwrap();

    // alex
    let collator_1_account_id: AccountId =
        AccountId::from_ss58check("1uWNn87BVmATvKRHW6ptAdhTBaYgYBakeKhJnRYCCPJJGaY").unwrap();
    let collator_1_aura_id: AuraId =
        AuraId::from_ss58check("1uWNn87BVmATvKRHW6ptAdhTBaYgYBakeKhJnRYCCPJJGaY").unwrap();

	#[allow(deprecated)]
	ChainSpec::builder(
		runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions {
            relay_chain: xcavate::RELAY_CHAIN.into(),
            // You MUST set this to the correct network!
            para_id: xcavate::PARACHAIN_ID,
		},
	)
	.with_name("Local Testnet")
	.with_id("local_testnet")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_patch(testnet_genesis(
		// initial collators.
        vec![
            // XCAVATE COLLATOR 0
            (collator_0_account_id, collator_0_aura_id),
            // XCAVATE COLLATOR 1
            (collator_1_account_id, collator_1_aura_id),
        ],
		get_endowed_accounts_with_balance(),
		get_root_account(),
		xcavate::PARACHAIN_ID.into(),
	))
	.with_protocol_id("template-local")
	.with_properties(properties)
	.build()
}

fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	root: AccountId,
	id: ParaId,
) -> serde_json::Value {
	serde_json::json!({
		"balances": {
			"balances": endowed_accounts.iter().cloned().map(|k| (k, xcavate::ENDOWMENT)).collect::<Vec<_>>(),
		},
		"parachainInfo": {
			"parachainId": id,
		},
		"collatorSelection": {
			"invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
			"candidacyBond": EXISTENTIAL_DEPOSIT * 16,
		},
		"session": {
			"keys": invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						template_session_keys(aura), // session keys
					)
				})
			.collect::<Vec<_>>(),
		},
		"polkadotXcm": {
			"safeXcmVersion": Some(SAFE_XCM_VERSION),
		},
		"sudo": { "key": Some(root) }
	})
}

pub fn get_root_account() -> AccountId {
    let json_data = &include_bytes!("../../seed/balances.json")[..];
    let additional_accounts_with_balance: Vec<AccountId> =
        serde_json::from_slice(json_data).unwrap_or_default();

    additional_accounts_with_balance[0].clone()
}

pub fn get_endowed_accounts_with_balance() -> Vec<AccountId> {
    let json_data = &include_bytes!("../../seed/balances.json")[..];
    let additional_accounts_with_balance: Vec<AccountId> =
        serde_json::from_slice(json_data).unwrap_or_default();

    additional_accounts_with_balance
}
