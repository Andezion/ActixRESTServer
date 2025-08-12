use bdk::blockchain::{ElectrumBlockchain, ElectrumBlockchainConfig, ConfigurableBlockchain};

pub fn create_blockchain() -> ElectrumBlockchain {
    let config = ElectrumBlockchainConfig {
        url: "ssl://electrum.blockstream.info:60002".to_string(),
        socks5: None,
        retry: 3,
        timeout: None,
        stop_gap: 10,
        validate_domain: true,
        // testnet: true,  
    };

    ElectrumBlockchain::from_config(&config).unwrap()
}
