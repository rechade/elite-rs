use crate::{My, elite::Commander};

struct StockItem {
    name: [char; 16],
    current_quantity: My,
    current_price: My,
    base_price: My,
    eco_adjust: My,
    base_quantity: My,
    mask: My,
    units: My,
}

pub const NO_OF_STOCK_ITEMS: usize = 17;
pub const ALIEN_ITEMS_IDX: usize = 16;

pub const SLAVES: usize = 3;
pub const NARCOTICS: usize = 6;
pub const FIREARMS: usize = 10;
struct StockMarket {
    stock_market: [StockItem; NO_OF_STOCK_ITEMS],
}
pub fn carrying_contraband(cmdr: &Commander) -> My {
    ((cmdr.current_cargo[SLAVES] + cmdr.current_cargo[NARCOTICS]) * 2
        + cmdr.current_cargo[FIREARMS])
}
