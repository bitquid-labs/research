use alloy::primitives::U256;

use crate::types::Pool;

pub fn get_pools() -> Vec<Pool>{
    let slashing = Pool{
        risk: 0,
        name: "Pool BB (Slashing Cover)".to_string(),
        apy: U256::from(4),
        min_period: U256::from(90)
    };

    let smart_contract = Pool{
        risk: 1,
        name: "Pool B+ (Smart Contract Cover)".to_string(),
        apy: U256::from(12),
        min_period: U256::from(180)
    };

    let protocol = Pool{
        risk: 3,
        name: "Pool BB- (Protocol Cover)".to_string(),
        apy: U256::from(11),
        min_period: U256::from(120)
    };

    let stablecoin = Pool{
        risk: 2,
        name: "Pool BBB (Stable Coin Cover)".to_string(),
        apy: U256::from(7),
        min_period: U256::from(60)
    };

    let pools = vec![slashing, smart_contract, protocol, stablecoin];
    pools
}