use sp_core::{Pair, Public, sr25519, ed25519};
use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature, WeHubConfig,
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::ChainType;
use hex_literal::hex;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
			],
			// Off-chain authorities
			vec![
				hex!["8b3b84c10aac8abf34906f5387b93ee126abc3c03da91da3aca030b868f97a02"].into(),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				(
					sr25519::Public::from_slice(&hex!("8d8dc4b6f620b6cc03382997a1b162f5daefea8508378274d1dd45843a8f0db6")).into(), // node-0
					ed25519::Public::from_slice(&hex!("8d8dc4b6f620b6cc03382997a1b162f5daefea8508378274d1dd45843a8f0db6")).into(),
				),
				(
					sr25519::Public::from_slice(&hex!("36a6655d34ea6be1bbbe6efcf2b2615465f87196cc9ffd19321e798c6626d71e")).into(), // node-1
					ed25519::Public::from_slice(&hex!("36a6655d34ea6be1bbbe6efcf2b2615465f87196cc9ffd19321e798c6626d71e")).into(),
				),
			],
			// Off-chain authorities
			vec![
				hex!("8d8dc4b6f620b6cc03382997a1b162f5daefea8508378274d1dd45843a8f0db6").into(), // node-0
				hex!("36a6655d34ea6be1bbbe6efcf2b2615465f87196cc9ffd19321e798c6626d71e").into(), // node-1
			],
			// Sudo account
			sr25519::Public::from_slice(&hex!("8d8dc4b6f620b6cc03382997a1b162f5daefea8508378274d1dd45843a8f0db6")).into(), // node-0
			// Pre-funded accounts
			vec![
				hex!("8b3b84c10aac8abf34906f5387b93ee126abc3c03da91da3aca030b868f97a02").into(), // node-0
				hex!("8b3b84c10aac8abf34906f5387b93ee126abc3c03da91da3aca030b868f97a02").into(), // node-1
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn public_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Jackblock Testnet",
		// ID
		"jackblock_testnet",
		ChainType::Live,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				(
					sr25519::Public::from_slice(&hex!("e0a1a14dba60a8c8e934691d2c494ffc81ebc27f1ffba8b01e3554e69b676e08")).into(), // repo
					ed25519::Public::from_slice(&hex!("e0a1a14dba60a8c8e934691d2c494ffc81ebc27f1ffba8b01e3554e69b676e08")).into(),
				),
				(
					sr25519::Public::from_slice(&hex!("e0a1a14dba60a8c8e934691d2c494ffc81ebc27f1ffba8b01e3554e69b676e08")).into(), // nunez
					ed25519::Public::from_slice(&hex!("e0a1a14dba60a8c8e934691d2c494ffc81ebc27f1ffba8b01e3554e69b676e08")).into(),
				),
				(
					sr25519::Public::from_slice(&hex!("e0a1a14dba60a8c8e934691d2c494ffc81ebc27f1ffba8b01e3554e69b676e08")).into(), // testing
					ed25519::Public::from_slice(&hex!("e0a1a14dba60a8c8e934691d2c494ffc81ebc27f1ffba8b01e3554e69b676e08")).into(),
				),
			],
			// Off-chain authorities
			vec![
				hex!("e0a1a14dba60a8c8e934691d2c494ffc81ebc27f1ffba8b01e3554e69b676e08").into(), // repo
				hex!("e0a1a14dba60a8c8e934691d2c494ffc81ebc27f1ffba8b01e3554e69b676e08").into(), // nunez
				hex!("e0a1a14dba60a8c8e934691d2c494ffc81ebc27f1ffba8b01e3554e69b676e08").into(), // testing
			],
			// Sudo account
			sr25519::Public::from_slice(&hex!("8b3b84c10aac8abf34906f5387b93ee126abc3c03da91da3aca030b868f97a02")).into(), // node-0
			// Pre-funded accounts
			vec![
				hex!("8b3b84c10aac8abf34906f5387b93ee126abc3c03da91da3aca030b868f97a02").into(), // repo
				hex!("8b3b84c10aac8abf34906f5387b93ee126abc3c03da91da3aca030b868f97a02").into(), // nunez
				hex!("8b3b84c10aac8abf34906f5387b93ee126abc3c03da91da3aca030b868f97a02").into(), // testing
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	offchain_authorities: Vec<AccountId>, // TO BE REMOVED --------------------------
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		pallet_aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
		pallet_wehub: Some(WeHubConfig {
			offchain_authorities,
		}),
	}
}
