use crate::{
    FLG_DEAD, My,
    elite::{Commander, ShipData},
    shipdata::{NO_OF_SHIPS, SHIP_CARGO, SHIP_MISSILE},
    space::UnivObject,
    stars::rand255,
};

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
