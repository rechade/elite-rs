use crate::{
    docked::{GRAMS, KILOGRAMS, TONNES},
    elite::{Commander, ShipData},
    shipdata::{NO_OF_SHIPS, SHIP_CARGO, SHIP_MISSILE},
    space::UnivObject,
    stars::rand255,
    GameParams, My, FLG_DEAD,
};

pub struct StockItem {
    pub name: String,
    pub current_quantity: My,
    pub current_price: My,
    pub base_price: My,
    pub eco_adjust: My,
    pub base_quantity: My,
    pub mask: My,
    pub units: usize,
}

pub const NO_OF_STOCK_ITEMS: usize = 17;
pub const ALIEN_ITEMS_IDX: usize = 16;

pub const SLAVES: usize = 3;
pub const NARCOTICS: usize = 6;
pub const FIREARMS: usize = 10;
pub struct StockMarket {
    pub stock_market: [StockItem; NO_OF_STOCK_ITEMS],
}

impl StockMarket {
    pub fn new() -> Self {
        StockMarket {
            stock_market: [
                StockItem {
                    name: "Food".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 19,
                    eco_adjust: -2,
                    base_quantity: 6,
                    mask: 0x01,
                    units: TONNES,
                },
                StockItem {
                    name: "Textiles".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 20,
                    eco_adjust: -1,
                    base_quantity: 10,
                    mask: 0x03,
                    units: TONNES,
                },
                StockItem {
                    name: "Radioactives".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 65,
                    eco_adjust: -3,
                    base_quantity: 2,
                    mask: 0x07,
                    units: TONNES,
                },
                StockItem {
                    name: "Slaves".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 40,
                    eco_adjust: -5,
                    base_quantity: 226,
                    mask: 0x1F,
                    units: TONNES,
                },
                StockItem {
                    name: "Liquor/Wines".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 83,
                    eco_adjust: -5,
                    base_quantity: 251,
                    mask: 0x0F,
                    units: TONNES,
                },
                StockItem {
                    name: "Luxuries".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 196,
                    eco_adjust: 8,
                    base_quantity: 54,
                    mask: 0x03,
                    units: TONNES,
                },
                StockItem {
                    name: "Narcotics".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 235,
                    eco_adjust: 29,
                    base_quantity: 8,
                    mask: 0x78,
                    units: TONNES,
                },
                StockItem {
                    name: "Computers".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 154,
                    eco_adjust: 14,
                    base_quantity: 56,
                    mask: 0x03,
                    units: TONNES,
                },
                StockItem {
                    name: "Machinery".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 117,
                    eco_adjust: 6,
                    base_quantity: 40,
                    mask: 0x07,
                    units: TONNES,
                },
                StockItem {
                    name: "Alloys".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 78,
                    eco_adjust: 1,
                    base_quantity: 17,
                    mask: 0x1F,
                    units: TONNES,
                },
                StockItem {
                    name: "Firearms".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 124,
                    eco_adjust: 13,
                    base_quantity: 29,
                    mask: 0x07,
                    units: TONNES,
                },
                StockItem {
                    name: "Furs".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 176,
                    eco_adjust: -9,
                    base_quantity: 220,
                    mask: 0x3F,
                    units: TONNES,
                },
                StockItem {
                    name: "Minerals".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 32,
                    eco_adjust: -1,
                    base_quantity: 53,
                    mask: 0x03,
                    units: TONNES,
                },
                StockItem {
                    name: "Gold".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 97,
                    eco_adjust: -1,
                    base_quantity: 66,
                    mask: 0x07,
                    units: KILOGRAMS,
                },
                StockItem {
                    name: "Platinum".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 171,
                    eco_adjust: -2,
                    base_quantity: 55,
                    mask: 0x1F,
                    units: KILOGRAMS,
                },
                StockItem {
                    name: "Gem-Stones".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 45,
                    eco_adjust: -1,
                    base_quantity: 250,
                    mask: 0x0F,
                    units: GRAMS,
                },
                StockItem {
                    name: "Alien Items".to_string(),
                    current_quantity: 0,
                    current_price: 0,
                    base_price: 53,
                    eco_adjust: 15,
                    base_quantity: 192,
                    mask: 0x07,
                    units: TONNES,
                },
            ],
        }
    }
}
pub fn carrying_contraband(cmdr: &Commander) -> My {
    ((cmdr.current_cargo[SLAVES] + cmdr.current_cargo[NARCOTICS]) * 2
        + cmdr.current_cargo[FIREARMS])
}
pub fn scoop_item(
    un: usize,
    universe: &mut [UnivObject],
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    cmdr: &Commander,
) {
    let mut trade = 123;

    if (universe[un].flags & FLG_DEAD) != 0 {
        return;
    }

    let da_type = universe[un].da_type;

    if (da_type == SHIP_MISSILE) {
        return;
    }
    // crst
    // if ((cmdr.fuel_scoop == 0)
    //     || (universe[un].location.y >= 0.0)
    //     || (total_cargo() == cmdr.cargo_capacity))
    // {
    //     explode_object(un);
    //     damage_ship(
    //         128.0 + (universe[un].energy / 2),
    //         universe[un].location.z > 0.0,
    //     );
    //     return;
    // }

    // if (da_type == SHIP_CARGO) {
    //     trade = rand255() & 7;
    //     cmdr.current_cargo[trade] += 1;
    //     info_message(stock_market[trade].name);
    //     remove_ship(un);
    //     return;
    // }

    // if (ship_list[da_type].scoop_type != 0) {
    //     trade = ship_list[da_type].scoop_type + 1;
    //     cmdr.current_cargo[trade] += 1;
    //     info_message(stock_market[trade].name);
    //     remove_ship(un);
    //     return;
    // }

    // explode_object(un);
    // damage_ship(universe[un].energy / 2, universe[un].location.z > 0);
}
pub fn generate_stock_market(params: &GameParams, cmdr: &mut Commander) {
    let mut quant: My;
    let mut price: My;

    for i in 0..NO_OF_STOCK_ITEMS {
        price = cmdr.stock_market.stock_market[i].base_price; /* Start with the base price	*/
        price += cmdr.market_rnd & cmdr.stock_market.stock_market[i].mask; /* Add in a random amount		*/
        price +=
            params.current_planet_data.economy as My * cmdr.stock_market.stock_market[i].eco_adjust; /* Adjust for planet economy	*/
        price &= 255; /* Only need bottom 8 bits		*/

        quant = cmdr.stock_market.stock_market[i].base_quantity; /* Start with the base quantity */
        quant += cmdr.market_rnd & cmdr.stock_market.stock_market[i].mask; /* Add in a random amount		*/
        quant -=
            params.current_planet_data.economy as My * cmdr.stock_market.stock_market[i].eco_adjust; /* Adjust for planet economy	*/
        quant &= 255; /* Only need bottom 8 bits		*/

        if (quant > 127) {
            /* In an 8-bit environment '>127' would be negative */
            quant = 0; /* So we set it to a minimum of zero. */
        }

        quant &= 63; /* Quantities range from 0..63 */

        cmdr.stock_market.stock_market[i].current_price = price * 4;
        cmdr.stock_market.stock_market[i].current_quantity = quant;
    }

    /* Alien Items are never available for purchase... */

    cmdr.stock_market.stock_market[ALIEN_ITEMS_IDX].current_quantity = 0;
}
pub fn total_cargo(cmdr: &Commander) -> My {
    let mut cargo_held = 0;
    for i in 0..17 {
        if ((cmdr.current_cargo[i] > 0) && (cmdr.stock_market.stock_market[i].units == TONNES)) {
            cargo_held += cmdr.current_cargo[i];
        }
    }

    return cargo_held;
}
