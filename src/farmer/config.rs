use blst::min_pk::SecretKey;
use dg_xch_core::blockchain::sized_bytes::{Bytes32, Bytes48};
use dg_xch_core::consensus::constants::CONSENSUS_CONSTANTS_MAP;
use dg_xch_keys::decode_puzzle_hash;
use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FarmingInfo {
    pub farmer_secret_key: Bytes32,
    pub launcher_id: Option<Bytes32>,
    pub pool_secret_key: Option<Bytes32>,
    pub owner_secret_key: Option<Bytes32>,
    pub auth_secret_key: Option<Bytes32>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PoolWalletConfig {
    pub launcher_id: Bytes32,
    pub pool_url: String,
    pub difficulty: Option<u64>,
    pub target_puzzle_hash: Bytes32,
    pub p2_singleton_puzzle_hash: Bytes32,
    pub owner_public_key: Bytes48,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BladebitHarvesterConfig {
    pub plot_directories: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HarvesterConfig {
    pub bladebit: Option<BladebitHarvesterConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub selected_network: String,
    pub ssl_root_path: Option<String>,
    pub fullnode_ws_host: String,
    pub fullnode_ws_port: u16,
    pub fullnode_rpc_host: String,
    pub fullnode_rpc_port: u16,
    pub farmer_info: Vec<FarmingInfo>,
    pub pool_info: Vec<PoolWalletConfig>,
    pub payout_address: String,
    pub harvester_configs: HarvesterConfig,
}
impl Config {
    pub fn save_as_yaml<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        fs::write(
            path.as_ref(),
            serde_yaml::to_string(&self)
                .map_err(|e| Error::new(ErrorKind::Other, format!("{:?}", e)))?,
        )
    }
    pub fn is_ready(&self) -> bool {
        CONSENSUS_CONSTANTS_MAP
            .get(&self.selected_network)
            .is_some()
            && !self.fullnode_ws_host.is_empty()
            && !self.fullnode_rpc_host.is_empty()
            && self.fullnode_ws_port != 0
            && self.fullnode_rpc_port != 0
            && !self.farmer_info.is_empty()
            && decode_puzzle_hash(&self.payout_address).is_ok()
            && self.pool_info.iter().all(|c| {
                self.farmer_info
                    .iter()
                    .any(|f| f.launcher_id == Some(c.launcher_id))
            })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            selected_network: "mainnet".to_string(),
            ssl_root_path: None,
            fullnode_rpc_host: "localhost".to_string(),
            fullnode_rpc_port: 8555,
            fullnode_ws_host: "localhost".to_string(),
            fullnode_ws_port: 8444,
            farmer_info: vec![],
            pool_info: vec![],
            payout_address: "".to_string(),
            harvester_configs: HarvesterConfig {
                bladebit: Some(BladebitHarvesterConfig {
                    plot_directories: vec![],
                }),
            },
        }
    }
}
impl TryFrom<&Path> for Config {
    type Error = Error;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        serde_yaml::from_str::<Config>(&fs::read_to_string(value)?)
            .map_err(|e| Error::new(ErrorKind::Other, format!("{:?}", e)))
    }
}
impl TryFrom<&PathBuf> for Config {
    type Error = Error;
    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(value.as_path())
    }
}

pub async fn load_keys(
    config: Arc<Config>,
) -> (
    HashMap<Bytes48, SecretKey>,
    HashMap<Bytes48, SecretKey>,
    HashMap<Bytes48, SecretKey>,
    HashMap<Bytes48, SecretKey>,
) {
    let mut farmer_secret_keys = HashMap::default();
    let mut owner_secret_keys = HashMap::default();
    let mut auth_secret_keys = HashMap::default();
    let mut pool_secret_keys = HashMap::default();
    for farmer_info in config.farmer_info.iter() {
        let f_sk: SecretKey = farmer_info.farmer_secret_key.into();
        farmer_secret_keys.insert(f_sk.sk_to_pk().to_bytes().into(), f_sk.clone());
        if let Some(pk) = farmer_info.pool_secret_key {
            let sec_key: SecretKey = pk.into();
            pool_secret_keys.insert(sec_key.sk_to_pk().to_bytes().into(), sec_key.clone());
        }
        if let Some(pk) = farmer_info.owner_secret_key {
            let sec_key: SecretKey = pk.into();
            owner_secret_keys.insert(sec_key.sk_to_pk().to_bytes().into(), sec_key.clone());
            if let Some(pk2) = farmer_info.auth_secret_key {
                let a_sec_key: SecretKey = pk2.into();
                auth_secret_keys.insert(sec_key.sk_to_pk().to_bytes().into(), a_sec_key.clone());
            }
        }
    }
    (
        farmer_secret_keys,
        owner_secret_keys,
        auth_secret_keys,
        pool_secret_keys,
    )
}
