const WEI_TO_ETH: f64 = 1_000_000_000_000_000_000f64;
const GWEI_TO_ETH: f64 = 1_000_000_000f64;

pub fn eth_to_gwei(value: f64) -> u128 {
    (value * GWEI_TO_ETH) as u128
}

pub fn eth_to_wei(value: f64) -> u128 {
    (value * WEI_TO_ETH) as u128
}

pub fn gwei_to_eth(value: u128) -> f64 {
    value as f64 / GWEI_TO_ETH
}

pub fn wei_to_eth(value: u128) -> f64 {
    value as f64 / WEI_TO_ETH
}

#[cfg(test)]
mod tests {
    use crate::convert;

    #[test]
    pub fn eth_to_gwei() {
        assert_eq!(1000000000, convert::eth_to_gwei(1f64))
    }

    #[test]
    pub fn eth_to_wei() {
        assert_eq!(1000000000000000000, convert::eth_to_wei(1f64))
    }

    #[test]
    pub fn gwei_to_eth() {
        assert_eq!(1f64, convert::gwei_to_eth(1000000000))
    }

    #[test]
    pub fn wei_to_eth() {
        assert_eq!(1f64, convert::wei_to_eth(1000000000000000000))
    }
}
