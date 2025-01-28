use xcavate_parachain_runtime::{Balance, UNIT};
// para_id & relay_chain
pub const PARACHAIN_ID: u32 = 4605;
pub const RELAY_CHAIN: &str = "paseo";

// token properties
pub const TOKEN_SYMBOL: &str = "XCAV";
pub const TOKEN_DECIMALS: u32 = 12;
pub const SS58_FORMAT: u32 = 42;

pub const ENDOWMENT: Balance = 100 * UNIT;