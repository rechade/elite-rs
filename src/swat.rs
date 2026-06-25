use macroquad::{color::WHITE, shapes::draw_line};

use crate::{
    Config, FLG_ANGRY, FLG_BOLD, FLG_CLOAKED, FLG_FLY_TO_PLANET, FLG_HAS_ECM, FLG_INACTIVE,
    FLG_POLICE, FLG_SLOW, GameParams, MAX_UNIV_OBJECTS, My, THICKNESS,
    elite::{Commander, SCR_FRONT_VIEW, SCR_LEFT_VIEW, SCR_REAR_VIEW, SCR_RIGHT_VIEW, ShipData},
    gfx::{GFX_SCALE, GFX_VIEW_BY},
    planet::PlanetData,
    shipdata::{
        NO_OF_SHIPS, SHIP_ALLOY, SHIP_CARGO, SHIP_COBRA3, SHIP_COBRA3_LONE, SHIP_CONSTRICTOR,
        SHIP_CORIOLIS, SHIP_COUGAR, SHIP_DODEC, SHIP_PLANET, SHIP_ROCK, SHIP_SUN, SHIP_THARGLET,
        SHIP_THARGOID, SHIP_VIPER,
    },
    sound::SND_PULSE,
    space::UnivObject,
    stars::{rand255, randint},
    vector::{Matrix, START_MATRIX},
};

pub const INITIAL_FLAGS: [My; NO_OF_SHIPS + 1] = [
    0,                            // NULL,
    0,                            // missile
    0,                            // coriolis
    FLG_SLOW | FLG_FLY_TO_PLANET, // escape
    FLG_INACTIVE,                 // alloy
    FLG_INACTIVE,                 // cargo
    FLG_INACTIVE,                 // boulder
    FLG_INACTIVE,                 // asteroid
    FLG_INACTIVE,                 // rock
    FLG_FLY_TO_PLANET | FLG_SLOW, // shuttle
    FLG_FLY_TO_PLANET | FLG_SLOW, // transporter
    0,                            // cobra3
    0,                            // python
    0,                            // boa
    FLG_SLOW,                     // anaconda
    FLG_SLOW,                     // hermit
    FLG_BOLD | FLG_POLICE,        // viper
    FLG_BOLD | FLG_ANGRY,         // sidewinder
    FLG_BOLD | FLG_ANGRY,         // mamba
    FLG_BOLD | FLG_ANGRY,         // krait
    FLG_BOLD | FLG_ANGRY,         // adder
    FLG_BOLD | FLG_ANGRY,         // gecko
    FLG_BOLD | FLG_ANGRY,         // cobra1
    FLG_SLOW | FLG_ANGRY,         // worm
    FLG_BOLD | FLG_ANGRY,         // cobra3
    FLG_BOLD | FLG_ANGRY,         // asp2
    FLG_BOLD | FLG_ANGRY,         // python
    FLG_POLICE,                   // fer_de_lance
    FLG_BOLD | FLG_ANGRY,         // moray
    FLG_BOLD | FLG_ANGRY,         // thargoid
    FLG_ANGRY,                    // thargon
    FLG_ANGRY,                    // constrictor
    FLG_POLICE | FLG_CLOAKED,     // cougar
    0,                            // dodec
];
pub const MISSILE_UNARMED: My = -2;
pub const MISSILE_ARMED: My = -1;
// pub struct Swat {
//     ecm_active: My,
//     missile_target: My,
//     // in_battle: My,
// }

// impl Swat {
//     pub fn new() -> Self {
//         Self {
//             ecm_active: 0,
//             missile_target: MISSILE_UNARMED,
//             // in_battle: 0,
//         }
//     }

//     pub fn set(ecm_active: My, missile_target: My, in_battle: My) -> Self {
//         Self {
//             ecm_active,
//             missile_target,
//             // in_battle,
//         }
//     }
// }
pub fn reset_weapons(params: &mut GameParams) {
    params.myship.laser_temp = 0;
    params.myship.laser_counter = 0;
    params.myship.laser = 0;
    params.myship.ecm_active = false;
    params.myship.missile_target = MISSILE_UNARMED;
}
pub fn draw_laser_lines(params: &GameParams, config: &Config) {
    if config.wireframe != 0 {
        draw_line(
            32.0 * GFX_SCALE as f32,
            GFX_VIEW_BY as f32,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            THICKNESS,
            WHITE,
        );
        draw_line(
            48.0 * GFX_SCALE as f32,
            GFX_VIEW_BY as f32,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            THICKNESS,
            WHITE,
        );
        draw_line(
            208.0 * GFX_SCALE as f32,
            GFX_VIEW_BY as f32,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            THICKNESS,
            WHITE,
        );
        draw_line(
            224.0 * GFX_SCALE as f32,
            GFX_VIEW_BY as f32,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            THICKNESS,
            WHITE,
        );
    } else {
        /*
        draw_triangle(
            32 * GFX_SCALE,
            GFX_VIEW_BY,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            48 * GFX_SCALE,
            GFX_VIEW_BY,
            RED,
        );
        draw_triangle(
            208 * GFX_SCALE,
            GFX_VIEW_BY,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            224 * GFX_SCALE,
            GFX_VIEW_BY,
            RED,
        );
        */
    }
}
pub fn fire_laser(params: &mut GameParams, cmdr: &mut Commander) -> My {
    if (params.myship.laser_counter == 0) && (params.myship.laser_temp < 242) {
        if params.current_screen == SCR_FRONT_VIEW {
            params.myship.laser = cmdr.front_laser;
        } else if params.current_screen == SCR_REAR_VIEW {
            params.myship.laser = cmdr.rear_laser;
        } else if params.current_screen == SCR_RIGHT_VIEW {
            params.myship.laser = cmdr.right_laser;
        } else if params.current_screen == SCR_LEFT_VIEW {
            params.myship.laser = cmdr.left_laser;
        } else {
            params.myship.laser = 0;
        }

        if params.myship.laser != 0 {
            params.myship.laser_counter = if params.myship.laser > 127 {
                0
            } else {
                params.myship.laser & 0xFA
            };
            params.myship.laser &= 127;
            // params.myship.laser2 = params.myship.laser;

            snd_play_sample(SND_PULSE);
            params.myship.laser_temp += 8;
            if params.energy > 1 {
                params.energy -= 1;
            }

            params.myship.laser_x = ((rand255() & 3) + 128 - 2) as My * GFX_SCALE as My;
            params.myship.laser_y = ((rand255() & 3) + 96 - 2) as My * GFX_SCALE as My;

            return 2;
        }
    }

    0
}

pub fn cool_laser(params: &mut GameParams) {
    params.myship.laser = 0;

    if params.myship.laser_temp > 0 {
        params.myship.laser_temp -= 1;
    }

    if params.myship.laser_counter > 0 {
        params.myship.laser_counter -= 1;
    }

    if params.myship.laser_counter > 0 {
        params.myship.laser_counter -= 1;
    }
}
pub fn snd_play_sample(_snd_pulse: usize) {
    println!("snd_play_sample()")
}
pub fn clear_universe(
    univ: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    in_battle: &mut bool,
) {
    for obj in univ.iter_mut() {
        obj.da_type = 0;
    }

    for sh in ship_count.iter_mut() {
        *sh = 0;
    }

    *in_battle = false;
}
pub fn remove_ship(
    un: usize,
    universe: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
) {
    let rotmat: Matrix = START_MATRIX;
    let px: My;
    let mut py: My;
    let pz: My;

    let da_type = universe[un].da_type;

    if da_type == 0 {
        return;
    }

    if da_type > 0 {
        ship_count[da_type] -= 1;
    }

    universe[un].da_type = 0;

    // check_missiles (un);

    if (da_type == SHIP_CORIOLIS) || (da_type == SHIP_DODEC) {
        px = universe[un].location.x as My;
        py = universe[un].location.y as My;
        pz = universe[un].location.z as My;

        py &= 0xFFFF;
        py |= 0x60000;

        add_new_ship(
            SHIP_SUN, px as f32, py as f32, pz as f32, &rotmat, 0, 0, universe, ship_list,
            ship_count,
        );
    }
}
pub fn add_new_station(
    sx: f32,
    sy: f32,
    sz: f32,
    rotmat: &Matrix,
    universe: &mut [UnivObject],
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    current_planet_data: &PlanetData,
) {
    let station = {
        if current_planet_data.techlevel >= 10 {
            SHIP_DODEC
        } else {
            SHIP_CORIOLIS
        }
    };
    // stations go in universe[1]
    universe[1].da_type = 0;
    add_new_ship(
        station, sx, sy, sz, rotmat, 0, -127, universe, ship_list, ship_count,
    );
}

pub fn add_new_ship(
    ship_type: usize,
    x: f32,
    y: f32,
    z: f32,
    rotmat: &Matrix,
    rotx: My,
    rotz: My,
    universe: &mut [UnivObject],
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
) -> Option<usize> {
    for (i, obj) in universe.iter_mut().enumerate() {
        if obj.da_type == 0 {
            obj.da_type = ship_type;
            obj.location.x = x;
            obj.location.y = y;
            obj.location.z = z;

            obj.distance = (x * x + y * y + z * z).sqrt() as My;

            obj.rotmat[0] = rotmat[0];
            obj.rotmat[1] = rotmat[1];
            obj.rotmat[2] = rotmat[2];

            obj.rotx = rotx;
            obj.rotz = rotz;

            obj.velocity = 0;
            obj.acceleration = 0;
            obj.bravery = 0;
            obj.target = 0;

            obj.flags = INITIAL_FLAGS[ship_type];

            if (ship_type != SHIP_PLANET) && (ship_type != SHIP_SUN) {
                obj.energy = ship_list[ship_type].energy;
                obj.missiles = ship_list[ship_type].missiles;
                ship_count[ship_type] += 1;
            }

            return Some(i);
        }
    }
    None
}
pub fn random_encounter(
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    universe: &mut [UnivObject],
    params: &GameParams,
    cmdr: &Commander,
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
) {
    create_thargoid(universe, ship_list, ship_count); //test
    if (ship_count[SHIP_CORIOLIS] != 0) || (ship_count[SHIP_DODEC] != 0) {
        return;
    }

    if rand255() == 136 {
        if ((universe[0].location.z as My) & 0x3e) != 0 {
            create_thargoid(universe, ship_list, ship_count);
        } else {
            create_cougar(ship_count, universe, ship_list);
        }
        return;
    }

    if (rand255() & 7) == 0 {
        create_trader(ship_count, universe, ship_list);
        return;
    }

    // check_for_asteroids();

    // check_for_cops();

    if ship_count[SHIP_VIPER] != 0 {
        return;
    }

    if params.in_battle {
        return;
    }

    if (cmdr.mission == 5) && (rand255() >= 200) {
        create_thargoid(universe, ship_list, ship_count);
    }

    // check_for_others();
}
fn create_other_ship(
    da_type: usize,
    universe: &mut [UnivObject],
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
) -> Option<usize> {
    let rotmat: Matrix = START_MATRIX;

    let z = 12000;
    let mut x = 1000 + (randint() & 8191);
    let mut y = 1000 + (randint() & 8191);

    if rand255() > 127 {
        x = -x;
    }
    if rand255() > 127 {
        y = -y;
    }
    add_new_ship(
        da_type, x as f32, y as f32, z as f32, &rotmat, 0, 0, universe, ship_list, ship_count,
    )
}

fn create_thargoid(
    universe: &mut [UnivObject],
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
) {
    let newship = create_other_ship(SHIP_THARGOID, universe, ship_list, ship_count);
    if let Some(ship) = newship {
        universe[ship].flags = FLG_ANGRY | FLG_HAS_ECM;
        universe[ship].bravery = 113;

        if rand255() > 64 {
            launch_enemy(
                ship,
                SHIP_THARGLET,
                FLG_ANGRY | FLG_HAS_ECM,
                96,
                universe,
                ship_list,
                ship_count,
            );
        }
    }
}

fn create_cougar(
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    universe: &mut [UnivObject],
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
) {
    if ship_count[SHIP_COUGAR] != 0 {
        return;
    }

    let newship = create_other_ship(SHIP_COUGAR, universe, ship_list, ship_count);
    if let Some(ship) = newship {
        universe[ship].flags = FLG_HAS_ECM; // | FLG_CLOAKED;
        universe[ship].bravery = 121;
        universe[ship].velocity = 18;
    }
}

fn create_trader(
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    universe: &mut [UnivObject],
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
) {
    let da_type = SHIP_COBRA3 + (rand255() as usize & 3);

    let newship = create_other_ship(da_type, universe, ship_list, ship_count);

    if let Some(ship) = newship {
        universe[ship].rotmat[2].z = -1.0;
        universe[ship].rotz = rand255() & 7;

        let rnd = rand255();
        universe[ship].velocity = (rnd & 31) | 16;
        universe[ship].bravery = rnd / 2;

        if (rnd & 1) != 0 {
            universe[ship].flags |= FLG_HAS_ECM;
        }

        //		if (rnd & 2)
        //			universe[newship].flags |= FLG_ANGRY;
    }
}

fn lone_hunter(
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    universe: &mut [UnivObject],
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    cmdr: &Commander,
    params: &mut GameParams,
) {
    let rnd;
    let mut da_type;

    if (cmdr.mission == 1)
        && (cmdr.galaxy_number == 1)
        && (params.docked_planet.d == 144)
        && (params.docked_planet.b == 33)
        && (ship_count[SHIP_CONSTRICTOR] == 0)
    {
        da_type = SHIP_CONSTRICTOR;
    } else {
        rnd = rand255();
        da_type = SHIP_COBRA3_LONE;
        if (rnd & 3) != 0 {
            da_type += 1
        };
        if rnd > 127 {
            da_type += 1
        };
    }

    let newship = create_other_ship(da_type, universe, ship_list, ship_count);

    if let Some(ship) = newship {
        universe[ship].flags = FLG_ANGRY;
        if (rand255() > 200) || (da_type == SHIP_CONSTRICTOR) {
            universe[ship].flags |= FLG_HAS_ECM;
        }

        universe[ship].bravery = ((rand255() * 2) | 64) & 127;
        params.in_battle = true;
    }
}
fn launch_enemy(
    un: usize,
    da_type: usize,
    flags: My,
    bravery: My,
    universe: &mut [UnivObject],
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
) {
    let rotmat = START_MATRIX;

    // add a new ship to the universe and get its index
    let newship = add_new_ship(
        da_type,
        universe[un].location.x,
        universe[un].location.y,
        universe[un].location.z,
        &rotmat,
        universe[un].rotx,
        universe[un].rotz,
        universe,
        ship_list,
        ship_count,
    );

    if let Some(n) = newship {
        if (universe[un].da_type == SHIP_CORIOLIS) || (universe[un].da_type == SHIP_DODEC) {
            universe[n].velocity = 32;
            universe[n].location.x += universe[n].rotmat[2].x * 2.0;
            universe[n].location.y += universe[n].rotmat[2].y * 2.0;
            universe[n].location.z += universe[n].rotmat[2].z * 2.0;
        }

        universe[n].flags |= flags;
        universe[n].rotz /= 2;
        universe[n].rotz *= 2;
        universe[n].bravery = bravery;

        if (da_type == SHIP_CARGO) || (da_type == SHIP_ALLOY) || (da_type == SHIP_ROCK) {
            universe[n].rotz = ((rand255() * 2) & 255) - 128;
            universe[n].rotx = ((rand255() * 2) & 255) - 128;
            universe[n].velocity = rand255() & 15;
        }
    }
}
