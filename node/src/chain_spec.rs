use sp_core::{Pair, Public, crypto::UncheckedInto, sr25519};
use node_template_runtime::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, SessionConfig, StakingConfig, DemocracyConfig,
	ElectionsConfig, CouncilConfig, TechnicalCommitteeConfig, opaque::SessionKeys,
	StakerStatus, Balance, currency::DOLLARS, WASM_BINARY, Signature, ImOnlineConfig,
};
use sp_consensus_babe::{AuthorityId as BabeId};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::{AuthorityId as ImOnlineId};
use sp_runtime::{Perbill, traits::{Verify, IdentifyAccount}};
use sc_service::ChainType;
use hex_literal::hex;
use telemetry::TelemetryEndpoints;
use pallet_staking::Forcing;

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
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

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
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

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
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
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
	}
}

// public staging network
pub fn tao_staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
		"Tao Staging Testnet",
		"tao_staging",
		ChainType::Live,
		tao_staging_testnet_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(TAO_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Westend Staging telemetry url is valid; qed")
		),
		Some("tao_staging"),
		None,
		Default::default(),
	)
}



fn tao_staging_testnet_genesis() -> GenesisConfig {
	// subkey generate
	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5CofS1P8bDL3kA1Xnwi7o1gWupMWEXSuCfLQUwiAtqDCrxxs
		hex!["0x20c363718239ce01110d0cec210fc623e008b58a7988c51a4d8734f2df28a17d"].into(),
		// 5CoXxz6YJg1rQ87zhRyUMX4oPAMvF6GuGZSKzgDgthypXVUw
		hex!["0x20aa4188baa85e057ceae411e9e8b1c6b1604977ff8c31a33bb38debb6abcc0b"].into(),
		// 5Dc8CVt6ivHLMhJP3uztcaZ3MRonmbLXCSBoZ2uACMKu8gad
		hex!["0x4432c87a5dd48cb456321be55f573119626ba04e9be732bd2c0b88f758efc960"].into(),
		// 5FLCHDZsnc47LVFPHTrcobcLmN7GpwkmAB3fGJfVUca3Ry9L
		hex!["0x908510084b9285192bddc815e8e3e64f1d572e011f3862a3d13d227c76233322"].into(),
	];

	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey inspect --scheme sr25519 "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey inspect --scheme ed25519 "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey inspect --scheme sr25519 "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
	)> = vec![(
				  // 5G3koKYUuXVDtKQu1ZKQ3m2Dn5q6pC8hzWZDF53a5bR6gTWo
				  hex!["0xb03797cd606a7d2cc5f2eeab3068f662b8392236fbecae8a7a60940c0bbbb64a"].into(),
				  // 5H3n23ibheesCeGwfT6i1BnWNtvMWtHSgdSo2VJTXsseRY42
				  hex!["0xdc77ee70de928ff692521c2bca563d93268d28e3ba8333eaa958124e547dc458"].into(),
				  // 5CrM34VcUtzRHQ8dr9rVLUyciV5fbMjzgKsRrDGschf2sCa8
				  hex!["0x22cf331016e368f9aed28dda562b345178f4d7637f21b2c25382865209625670"].unchecked_into(),
				  // 5CKdn4ocmvNT8sCL92QEwkDAb4j92FKTQEQXN6V1UwofGw65
				  hex!["0x0b62f2e1068a89291659c248ff6c9a07a34f942e3ba8a4700d99a7fbe522caea"].unchecked_into(),
				  // 5ExmeSd3R2qJMGidNpTtsThzRvj4ahbBFW4gYLyRt3Gc84sS
				  hex!["0x802dfbc895211a247848e2efb2f253342279c473af42effbc9e2cc9a9a0d223e"].unchecked_into(),
			  ),(
				  // 5DSZDLLL8EX6Ac2wsaMK3UDt7oEfSinPVRXAC9JFrr7KiEoe
				  hex!["0x3ce688f9bee25141c4a9d208310baf2314192851b3632780002e3bd187b97e55"].into(),
				  // 5CQ4rMKg1Kt4HteRom6QARLn8E5w1eZpVzUT3udTMa4FmfwC
				  hex!["0x0ec456285fdfd45773dd64b2503a8b6d23d1f7c2dcbb1bcdc028ecd7d4fd4c3a"].into(),
				  // 5FvgdaCP6PrTT11tiVU8csK8e8x7cNVzC7iTjQRWQaKasKpY
				  hex!["0xaad2d81e6229c13e42f87d3025781877a0b64b84ad1054e05272211ce768fc20"].unchecked_into(),
				  // 5CEhhW7yCUhkeMEQMeYuZvyPrMPGUxpboWk9kGzxVagXpzsb
				  hex!["0x079fec0c20ac5f58dd0e58ff1efa2cebd80e375052699e76e61bfaf6eeaf337e"].unchecked_into(),
				  // 5F9bgFpLxM1SGUXCbXmgYRcJK7fd9muUk89ihL34iPqRiHfB
				  hex!["0x8870200ff3df4b6724201eafc6daac4ca87ac1877b212bddd5644145a96f441c"].unchecked_into(),
			  ),(
				  // 5DbwcHg2HpAPqHyMsuC6LqfqemxPpyTJDMb8XXqJtebMRDfK
				  hex!["0x440f2271f81bc962307eb1197502d1006dc9bae219012b7b6a2337658dee974c"].into(),
				  // 5EYx4Xp61cLwsvyJ5GrCy84uqG3sU9vRFi5qfn19gm6JqvDJ
				  hex!["0x6e0327ac31455fa6480febdd10fce00483dd6e37347001516b19fb9f25a80011"].into(),
				  // 5HTPbJgeVMf5Zt8NJ9AYvtKHbb6xdFrdyuuL4axMg3ymYZ3E
				  hex!["0xee7a53955e8adf176fe4cbacd0c1c7439ad3bcafc463d8c3fb21b615bc99680b"].unchecked_into(),
				  // 5EtvdGs2YMG9q2nzodjRMwKQAxnfo9FhCno1s4w5REY7uBnC
				  hex!["0x7d3f3b1631e47eecd75afe7859038e4a267546eb81d691722ca2b7daee744d93"].unchecked_into(),
				  // 5EnknmfLckmo3N5wPxMvLgrMLuNCv378xHjVL8TPdRpmjVk8
				  hex!["0x788aa584aa75aa175936aca1acae4dd4a89e405de2bdf4b35949ef4fe26ef400"].unchecked_into(),
			  ),(
				  // 5CkbPcFZazsz8NjZqupkoTjAfNsxBH8yUGFpU8RvEWzkBuDL
				  hex!["0x1e6c0c3d63e6ed885a18faa6394fc91ecfc9afdf028095178615ba4178c3e962"].into(),
				  // 5G3pt7fTCA9tQpRNrwD8q1Am9VFs8RfXgb92mEXFSGZ6cht2
				  hex!["0xb04556241709692db9e742fb8c44bf0da6dd55228dfb6bf496b4af292986955e"].into(),
				  // 5CDic61nnMNAUhV3h2dPA3ZnhLDQ98UceJf8zuBPgPSPLdPh
				  hex!["0x06dfba87c9abef7af04c5120003a50a04c1816df3749c54c6e3a53026267ff05"].unchecked_into(),
				  // 5GbGUHs5Givgg7k1qjg7hxH4AXscDKCiryCiLtEHCbus5fRd
				  hex!["0xc84015c7dd985f7406c4b5d013b4bb5fa9025f382090b3e28dc789609aa9a01b"].unchecked_into(),
				  // 5CXdpD5hv8gbcqWgjDNhhMkRFTWajNYbKMV1SPTksDieuZZ6
				  hex!["0x148a0499149297dd02c49866a7c9ee6b9060d15d970158c3913a4aec8ecc3708"].unchecked_into(),
			  )];

	const ENDOWMENT: u128 = 1_000_000 * DOLLARS;
	const STASH: u128 = 100 * DOLLARS;
	let num_endowed_accounts = endowed_accounts.len();

	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		}),
		babe: Some(BabeConfig {
			authorities: vec![],
		}),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		sudo: Some(SudoConfig {
			key: endowed_accounts[0].clone(),
		}),
		session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(
					x.0.clone(),
					x.0.clone(),
					session_keys(x.2.clone(), x.3.clone(), x.4.clone())
				)
			}).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		democracy: Some(DemocracyConfig::default()),
		elections_phragmen: Some(ElectionsConfig {
			members: endowed_accounts.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		}),
		collective_Instance1: Some(CouncilConfig::default()),
		collective_Instance2: Some(TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		}),
		membership_Instance1: Some(Default::default()),
		treasury: Some(Default::default()),
	}
}

