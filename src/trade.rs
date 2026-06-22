struct StockItem {
    name: [char; 16],
    current_quantity: i16,
    current_price: i16,
    base_price: i16,
    eco_adjust: i16,
    base_quantity: i16,
    mask: i16,
    units: i16,
}

pub const NO_OF_STOCK_ITEMS: usize = 17;
pub const ALIEN_ITEMS_IDX: usize = 16;

struct StockMarket {
    stock_market: [StockItem; NO_OF_STOCK_ITEMS],
}
