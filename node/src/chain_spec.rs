use sc_service::{ChainType, Properties};
use sp_core::{crypto::Ss58Codec, sr25519};

use crate::runtime::{
	get_account_id_from_seed, testnet_genesis, AccountId, GenesisConfig, ValidatorIdentity,
	WASM_BINARY,
};

pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

fn properties(token_symbol: &str) -> Option<Properties> {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), token_symbol.into());
	properties.insert("tokenDecimals".into(), 18u32.into());
	Some(properties)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let balance = 200_000_000;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					// Alice pub in EVM is: 0xd43593c715fdd31c61141abd04a99fd6822c8558
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						balance,
					),
					(get_account_id_from_seed::<sr25519::Public>("Bob"), balance),
					(
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						balance,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						balance,
					),
					// Arrakis.TEST account in MetaMask
					// Import known test account with private key
					// 0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391
					// H160 address: 0xaaafB3972B05630fCceE866eC69CdADd9baC2771
					(
						AccountId::from_ss58check(
							"5FQedkNQcF2fJPwkB6Z1ZcMgGti4vcJQNs6x85YPv3VhjBBT",
						)
						.unwrap(),
						balance,
					),
				],
				// Initial PoA authorities
				vec![ValidatorIdentity::from_seed("Alice")],
				888888,
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("GGX chain Dev"),
		None,
		// Properties
		properties("GGX Dev"),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let balance = 333_333_333;
	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						balance,
					),
					(get_account_id_from_seed::<sr25519::Public>("Bob"), balance),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						balance,
					),
				],
				vec![
					ValidatorIdentity::from_seed("Alice"),
					ValidatorIdentity::from_seed("Bob"),
				],
				888888,
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("GGX Chain Local"),
		None,
		// Properties
		properties("GGX Local"),
		// Extensions
		None,
	))
}

#[cfg(not(feature = "brooklyn"))]
pub fn sydney_testnet_config() -> Result<ChainSpec, String> {
	use sc_telemetry::TelemetryEndpoints;
	use sp_runtime::traits::IdentifyAccount;

	let wasm_binary = WASM_BINARY.ok_or_else(|| "Runtime wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"GGX Chain Sydney Testnet",
		"GGX",
		ChainType::Live,
		move || {
			testnet_genesis(
				wasm_binary,
				// Sudo account
				// Multisig
				sr25519::Public::from_ss58check("5Fxk5MUYBQf4ALRFkmSr9UNGyVz52dLhv7w8PkgKKBkjA5Fz")
					.unwrap()
					.into_account()
					.into(),
				// Pre-funded accounts
				vec![
					(
						sr25519::Public::from_ss58check(
							"5Fxk5MUYBQf4ALRFkmSr9UNGyVz52dLhv7w8PkgKKBkjA5Fz",
						)
						.unwrap()
						.into_account()
						.into(),
						1_000_000_000 - 10_000 - 1100,
					),
					(
						sr25519::Public::from_ss58check(
							"5Gn1Vdty5miDaaxBS3RE9UdxZJQeSJEgVmzVbqjmMZWvXiSR",
						)
						.unwrap()
						.into_account()
						.into(),
						1100,
					),
					(
						sr25519::Public::from_ss58check(
							"5GxBFw4nqvhX9X1C6apRpvWm57QaRb6MkVSF7YzQXLq2VJ6m",
						)
						.unwrap()
						.into_account()
						.into(),
						10_000,
					),
				],
				// Initial Validator
				vec![ValidatorIdentity::from_pub(
					"5Dg4ny1CSPek3yiKMTDSC9tRHzbv2xmidGLRhQug9cLZweQk",
					"5Gn1Vdty5miDaaxBS3RE9UdxZJQeSJEgVmzVbqjmMZWvXiSR",
					"5GwiY7NjgEAyn8dtYEaAr5GVpwJZncVGQxX2ayBsVCASirfY",
				)],
				8886,
				false,
			)
		},
		// Some dns bootnodes
		vec![],
		// Telemetry
		Some(
			TelemetryEndpoints::new(vec![(
				"wss://testnet.telemetry.sydney.ggxchain.io/submit/".into(),
				0,
			)])
			.expect("Telemetry url is valid"),
		),
		// Protocol ID
		Some("GGX Sydney"),
		None,
		// Properties
		properties("GGXT"),
		// Extensions
		None,
	))
}
