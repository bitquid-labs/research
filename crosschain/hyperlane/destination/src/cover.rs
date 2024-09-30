use alloy::primitives::U256;

use crate::{cid::{AVALON_FINANCE, BABYLON, BIMA, BITSTABLE, CORE_DAO, EAST_BLUE, INFSTONE, LIQUIDIUM, LOMBARD, LORENZO, MERLIN, OCTOPUS_BRIDGE, OMNIBTC, ORDEEZ, PALLADIUM, PUMPBTC, PWR, RYE, SATOSHI, YONA, ZEST}, types::Covers};


pub fn get_covers() -> Vec<Covers> {
    let babylon: Covers = Covers {
        cid: BABYLON.to_string(),
        risk: 0,
        name: "Babylon Validator".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(40),
        cost: U256::from(5),
        pool_id: U256::from(1),
    };

    let infstone: Covers = Covers {
        cid: INFSTONE.to_string(),
        risk: 0,
        name: "InfStones Validator".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(17),
        cost: U256::from(2),
        pool_id: U256::from(1),
    };

    let merlin: Covers = Covers {
        cid: MERLIN.to_string(),
        risk: 0,
        name: "Merlin Validator".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(16),
        cost: U256::from(5),
        pool_id: U256::from(1),
    };

    let coredao: Covers = Covers {
        cid: CORE_DAO.to_string(),
        risk: 0,
        name: "Core DAO Validator".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(19),
        cost: U256::from(3),
        pool_id: U256::from(1),
    };

    let pwr: Covers = Covers {
        cid: PWR.to_string(),
        risk: 0,
        name: "PWR Validator".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(8),
        cost: U256::from(5),
        pool_id: U256::from(1),
    };

    let lorenzo: Covers = Covers {
        cid: LORENZO.to_string(),
        risk: 1,
        name: "Lorenzo Smart Contract".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(30),
        cost: U256::from(14),
        pool_id: U256::from(2),
    };

    let lombard: Covers = Covers {
        cid: LOMBARD.to_string(),
        risk: 1,
        name: "Lombard Smart Contract".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(22),
        cost: U256::from(8),
        pool_id: U256::from(2),
    };

    let pumpbtc: Covers = Covers {
        cid: PUMPBTC.to_string(),
        risk: 1,
        name: "PumpBTC Smart Contract".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(18),
        cost: U256::from(16),
        pool_id: U256::from(2),
    };

    let octopus_bridge: Covers = Covers {
        cid: OCTOPUS_BRIDGE.to_string(),
        risk: 1,
        name: "Octopus Bridge Smart Contract".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(7),
        cost: U256::from(8),
        pool_id: U256::from(2),
    };

    let avalon_finance: Covers = Covers {
        cid: AVALON_FINANCE.to_string(),
        risk: 1,
        name: "Avalon Finance Smart Contract".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(18),
        cost: U256::from(15),
        pool_id: U256::from(2),
    };

    let omnibtc: Covers = Covers {
        cid: OMNIBTC.to_string(),
        risk: 1,
        name: "OmniBTC Smart Contract".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(5),
        cost: U256::from(13),
        pool_id: U256::from(2),
    };

    let zest: Covers = Covers {
        cid: ZEST.to_string(),
        risk: 3,
        name: "Zest Protocol".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(22),
        cost: U256::from(13),
        pool_id: U256::from(3),
    };

    let liquidium: Covers = Covers {
        cid: LIQUIDIUM.to_string(),
        risk: 3,
        name: "Liquidium Protocol".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(32),
        cost: U256::from(7),
        pool_id: U256::from(3),
    };

    let ordeez: Covers = Covers {
        cid: ORDEEZ.to_string(),
        risk: 3,
        name: "Ordeez Protocol".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(6),
        cost: U256::from(15),
        pool_id: U256::from(3),
    };

    let eastblue: Covers = Covers {
        cid: EAST_BLUE.to_string(),
        risk: 3,
        name: "East Blue Protocol".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(21),
        cost: U256::from(7),
        pool_id: U256::from(3),
    };

    let yona: Covers = Covers {
        cid: YONA.to_string(),
        risk: 3,
        name: "Yona Network".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(19), // Different from amount in docs
        cost: U256::from(12),
        pool_id: U256::from(3),
    };

    let satoshi: Covers = Covers {
        cid: SATOSHI.to_string(),
        risk: 2,
        name: "Satoshi Protocol".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(37),
        cost: U256::from(8),
        pool_id: U256::from(4),
    };

    let palladium: Covers = Covers {
        cid: PALLADIUM.to_string(),
        risk: 2,
        name: "Palladium Protocol".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(8),
        cost: U256::from(4),
        pool_id: U256::from(4),
    };

    let bima: Covers = Covers {
        cid: BIMA.to_string(),
        risk: 2,
        name: "Bima BTC".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(32),
        cost: U256::from(9),
        pool_id: U256::from(4),
    };

    let bitstable: Covers = Covers {
        cid: BITSTABLE.to_string(),
        risk: 2,
        name: "Bitstable".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(11),
        cost: U256::from(5),
        pool_id: U256::from(4),
    };

    let rye: Covers = Covers {
        cid: RYE.to_string(),
        risk: 2,
        name: "Rye Harvest".to_string(),
        chains: "Bitcoin+".to_string(),
        capacity: U256::from(12),
        cost: U256::from(7),
        pool_id: U256::from(4),
    };
    let covers = vec![babylon, infstone, merlin, coredao, pwr, 
                                    lorenzo, lombard, pumpbtc, octopus_bridge, avalon_finance, omnibtc, 
                                    zest, liquidium, ordeez, eastblue, yona,
                                    satoshi, palladium, bima, bitstable, rye];

    covers
}