use alloy::primitives::U256;

#[derive(Debug, Clone)]
pub struct Covers {
    pub cid: String,
    pub risk: u8,
    pub name: String,
    pub chains: String,
    pub capacity: U256,
    pub cost: U256,
    pub pool_id: U256
}

#[derive(Debug, Clone)]
pub struct Pool {
    pub risk: u8,
    pub name: String,
    pub apy: U256,
    pub min_period: U256
}