use macroquad::{
    audio::{self, Sound},
    color::{RED, WHITE},
    prelude::rand,
    shapes::draw_line,
    window::clear_background,
};

use crate::{
    Config, FLG_ANGRY, FLG_BOLD, FLG_CLOAKED, FLG_DEAD, FLG_FLY_TO_PLANET, FLG_HAS_ECM,
    FLG_INACTIVE, FLG_POLICE, FLG_SLOW, GameParams, MAX_UNIV_OBJECTS, MILITARY_LASER, MINING_LASER,
    My, THICKNESS,
    elite::{Commander, SCR_FRONT_VIEW, SCR_LEFT_VIEW, SCR_REAR_VIEW, SCR_RIGHT_VIEW, ShipData},
    gfx::{GFX_SCALE, GFX_VIEW_BY},
    info_message,
    planet::PlanetData,
    shipdata::{
        NO_OF_SHIPS, SHIP_ALLOY, SHIP_ASTEROID, SHIP_CARGO, SHIP_COBRA3, SHIP_COBRA3_LONE,
        SHIP_CONSTRICTOR, SHIP_CORIOLIS, SHIP_COUGAR, SHIP_DODEC, SHIP_HERMIT, SHIP_MISSILE,
        SHIP_PLANET, SHIP_ROCK, SHIP_SIDEWINDER, SHIP_SUN, SHIP_THARGLET, SHIP_THARGOID,
        SHIP_VIPER,
    },
    sound::{SND_BEEP, SND_BOOP, SND_EXPLODE, SND_HIT_ENEMY, SND_MISSILE, SND_PULSE},
    space::{DaType, UnivObject, damage_ship},
    stars::{rand255, randint},
    trade::carrying_contraband,
    vector::{Matrix, START_MATRIX, START_VECTOR, Vector, unit_vector, vector_dot_product},
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
// pub const MISSILE_UNARMED: My = -2;
// pub const MISSILE_ARMED: My = -1;
// ***warning
pub const MISSILE_UNARMED: DaType = -2;
pub const MISSILE_ARMED: DaType = -1;
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
            params.screen_width * 0.1,
            params.row_y_pos,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            THICKNESS,
            WHITE,
        );
        draw_line(
            params.screen_width * 0.2,
            params.row_y_pos,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            THICKNESS,
            WHITE,
        );
        draw_line(
            params.screen_width * 0.8,
            params.row_y_pos,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            THICKNESS,
            WHITE,
        );
        draw_line(
            params.screen_width * 0.9,
            params.row_y_pos,
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
pub fn fire_laser(params: &mut GameParams, cmdr: &mut Commander, sample_list: &[Sound]) -> My {
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
            // crst
            // params.myship.laser2 = params.myship.laser;

            snd_play_sample(sample_list, SND_PULSE);
            params.myship.laser_temp += 8;
            if params.energy > 1 {
                params.energy -= 1;
            }

            params.myship.laser_x = (rand::gen_range(0.0, params.screen_width * 0.05)
                + params.screen_width * 0.475) as i32;
            params.myship.laser_y = (rand::gen_range(0.0, params.screen_height * 0.05)
                + params.row_y_pos * 0.475) as i32;

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
pub fn snd_play_sample(sample_list: &[Sound], sample: usize) {
    audio::play_sound_once(&sample_list[sample]);
}
pub fn clear_universe(
    univ: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    in_battle: &mut usize,
) {
    for obj in univ.iter_mut() {
        obj.da_type = 0;
    }

    for sh in ship_count.iter_mut() {
        *sh = 0;
    }

    *in_battle = 0;
}
fn check_missiles(
    un: DaType,
    params: &mut GameParams,
    universe: &mut [UnivObject],
    sample_list: &[Sound],
) {
    if (params.myship.missile_target == un) {
        params.myship.missile_target = MISSILE_UNARMED;
        info_message("Target Lost".to_string(), params, sample_list);
    }

    for i in 0..MAX_UNIV_OBJECTS {
        if ((universe[i].da_type == SHIP_MISSILE) && (universe[i].target == un)) {
            universe[i].flags |= FLG_DEAD;
        }
    }
}
pub fn remove_ship(
    un: DaType,
    universe: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
    params: &mut GameParams,
    sample_list: &[Sound],
) {
    let rotmat: Matrix = START_MATRIX;
    let px: My;
    let mut py: My;
    let pz: My;

    let da_type = universe[un as usize].da_type;

    if da_type == 0 {
        return;
    }

    if da_type > 0 {
        ship_count[da_type as usize] -= 1;
    }

    universe[un as usize].da_type = 0;

    check_missiles(un, params, universe, sample_list);

    if (da_type == SHIP_CORIOLIS) || (da_type == SHIP_DODEC) {
        px = universe[un as usize].location.x as My;
        py = universe[un as usize].location.y as My;
        pz = universe[un as usize].location.z as My;

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
    // stations always go in universe[1]
    universe[1].da_type = 0;
    add_new_ship(
        station, sx, sy, sz, rotmat, 0, -127, universe, ship_list, ship_count,
    );
}

pub fn add_new_ship(
    ship_type: DaType,
    x: f32,
    y: f32,
    z: f32,
    rotmat: &Matrix,
    rotx: My,
    rotz: My,
    universe: &mut [UnivObject],
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
) -> Option<DaType> {
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

            if (ship_type != SHIP_PLANET) && (ship_type != SHIP_SUN) {
                obj.flags = INITIAL_FLAGS[ship_type as usize];
                obj.energy = ship_list[ship_type as usize].energy;
                obj.missiles = ship_list[ship_type as usize].missiles;
                ship_count[ship_type as usize] += 1;
            }

            return Some(i as DaType);
        }
    }
    None
}
fn create_lone_hunter(
    cmdr: &Commander,
    params: &mut GameParams,
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    universe: &mut [UnivObject],
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
) {
    let rnd;
    let da_type;

    if ((cmdr.mission == 1)
        && (cmdr.galaxy_number == 1)
        && (params.docked_planet.d == 144)
        && (params.docked_planet.b == 33)
        && (ship_count[SHIP_CONSTRICTOR as usize] == 0))
    {
        da_type = SHIP_CONSTRICTOR;
    } else {
        rnd = rand255();
        // crst
        da_type =
            SHIP_COBRA3_LONE + (if rnd & 3 != 0 { 1 } else { 0 }) + (if rnd > 127 { 1 } else { 0 });
    }

    let newship = create_other_ship(da_type, universe, ship_list, ship_count);

    if let Some(ship) = newship {
        universe[ship as usize].flags = FLG_ANGRY;
        if ((rand255() > 200) || (da_type == SHIP_CONSTRICTOR)) {
            universe[ship as usize].flags |= FLG_HAS_ECM;
        }

        universe[ship as usize].bravery = ((rand255() * 2) | 64) & 127;
        params.in_battle += 1;
    }
}
/* Check for a random asteroid encounter... */

fn check_for_asteroids(
    universe: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
) {
    let da_type;
    if ((rand255() >= 35) || (ship_count[SHIP_ASTEROID as usize] >= 3)) {
        return;
    }

    if (rand255() > 253) {
        da_type = SHIP_HERMIT;
    } else {
        da_type = SHIP_ASTEROID;
    }

    let newship = create_other_ship(da_type, universe, ship_list, ship_count);

    if let Some(ship) = newship {
        // universe[newship].velocity = (rand255() & 31) | 16;
        universe[ship as usize].velocity = 8;
        universe[ship as usize].rotz = if rand255() > 127 { -127 } else { 127 };
        universe[ship as usize].rotx = 16;
    }
}

/* If we've been a bad boy then send the cops after us... */

fn check_for_cops(
    universe: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    cmdr: &Commander,
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
) {
    let mut offense = carrying_contraband(cmdr) * 2;
    if (ship_count[SHIP_VIPER as usize] == 0) {
        offense |= cmdr.legal_status;
    }

    if (rand255() >= offense) {
        return;
    }

    let newship = create_other_ship(SHIP_VIPER, universe, ship_list, ship_count);

    if let Some(ship) = newship {
        universe[ship as usize].flags = FLG_ANGRY;
        if (rand255() > 245) {
            universe[ship as usize].flags |= FLG_HAS_ECM;
        }

        universe[ship as usize].bravery = ((rand255() * 2) | 64) & 127;
    }
}

fn check_for_others(
    universe: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    params: &mut GameParams,
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
    cmdr: &Commander,
) {
    let mut rotmat: Matrix = START_MATRIX;
    let mut da_type;
    let mut newship;

    let gov = params.current_planet_data.government as My;
    let rnd = rand255();

    if ((gov != 0) && ((rnd >= 90) || ((rnd & 7) < gov))) {
        return;
    }

    if (rand255() < 100) {
        create_lone_hunter(cmdr, params, ship_count, universe, ship_list);
        return;
    }

    /* Pack hunters... */

    let mut z = 12000.0;
    let mut x = 1000.0 + { if (randint() & 8191 != 0) { 1.0 } else { 0.0 } };
    let mut y = 1000.0 + { if (randint() & 8191 != 0) { 1.0 } else { 0.0 } };

    if (rand255() > 127) {
        x = -x;
    }
    if (rand255() > 127) {
        y = -y;
    }

    let rnd = rand255() & 3;

    for i in 0..rnd {
        da_type = SHIP_SIDEWINDER
            + (if (rand255() & rand255() & 7) != 0 {
                1
            } else {
                0
            });
        newship = add_new_ship(
            da_type, x, y, z, &rotmat, 0, 0, universe, ship_list, ship_count,
        );
        if let Some(ship) = newship {
            universe[ship as usize].flags = FLG_ANGRY;
            if (rand255() > 245) {
                universe[ship as usize].flags |= FLG_HAS_ECM;
            }

            universe[ship as usize].bravery = ((rand255() * 2) | 64) & 127;
            params.in_battle += 1;
        }
    }
}
pub fn random_encounter(
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    universe: &mut [UnivObject],
    params: &mut GameParams,
    cmdr: &Commander,
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
) {
    if (ship_count[SHIP_CORIOLIS as usize] != 0) || (ship_count[SHIP_DODEC as usize] != 0) {
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

    check_for_asteroids(universe, ship_count, ship_list);

    check_for_cops(universe, ship_count, cmdr, ship_list);

    if ship_count[SHIP_VIPER as usize] != 0 {
        return;
    }

    if params.in_battle != 0 {
        return;
    }

    if (cmdr.mission == 5) && (rand255() >= 200) {
        create_thargoid(universe, ship_list, ship_count);
    }

    check_for_others(universe, ship_count, params, ship_list, cmdr);
}
fn create_other_ship(
    da_type: DaType,
    universe: &mut [UnivObject],
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
) -> Option<DaType> {
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

pub fn create_thargoid(
    universe: &mut [UnivObject],
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
) {
    let newship = create_other_ship(SHIP_THARGOID, universe, ship_list, ship_count);
    if let Some(ship) = newship {
        universe[ship as usize].flags = FLG_ANGRY | FLG_HAS_ECM;
        universe[ship as usize].bravery = 113;

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
    if ship_count[SHIP_COUGAR as usize] != 0 {
        return;
    }

    let newship = create_other_ship(SHIP_COUGAR, universe, ship_list, ship_count);
    if let Some(ship) = newship {
        // crst
        universe[ship as usize].flags = FLG_HAS_ECM; // | FLG_CLOAKED;
        universe[ship as usize].bravery = 121;
        universe[ship as usize].velocity = 18;
    }
}

fn create_trader(
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    universe: &mut [UnivObject],
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
) {
    let da_type = SHIP_COBRA3 + (rand255() as DaType & 3);

    let newship = create_other_ship(da_type, universe, ship_list, ship_count);

    if let Some(ship) = newship {
        universe[ship as usize].rotmat[2].z = -1.0;
        universe[ship as usize].rotz = rand255() & 7;

        let rnd = rand255();
        universe[ship as usize].velocity = (rnd & 31) | 16;
        universe[ship as usize].bravery = rnd / 2;

        if (rnd & 1) != 0 {
            universe[ship as usize].flags |= FLG_HAS_ECM;
        }

        // crst
        //		if (rnd & 2)
        //			universe[newship].flags |= FLG_ANGRY;
    }
}

pub fn lone_hunter(
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
        && (ship_count[SHIP_CONSTRICTOR as usize] == 0)
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
        universe[ship as usize].flags = FLG_ANGRY;
        if (rand255() > 200) || (da_type == SHIP_CONSTRICTOR) {
            universe[ship as usize].flags |= FLG_HAS_ECM;
        }

        universe[ship as usize].bravery = ((rand255() * 2) | 64) & 127;
        params.in_battle += 1;
    }
}
pub fn launch_enemy(
    un: DaType,
    da_type: DaType,
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
        universe[un as usize].location.x,
        universe[un as usize].location.y,
        universe[un as usize].location.z,
        &rotmat,
        universe[un as usize].rotx,
        universe[un as usize].rotz,
        universe,
        ship_list,
        ship_count,
    );

    if let Some(n) = newship {
        if (universe[un as usize].da_type == SHIP_CORIOLIS)
            || (universe[un as usize].da_type == SHIP_DODEC)
        {
            universe[n as usize].velocity = 32;
            universe[n as usize].location.x += universe[n as usize].rotmat[2].x * 2.0;
            universe[n as usize].location.y += universe[n as usize].rotmat[2].y * 2.0;
            universe[n as usize].location.z += universe[n as usize].rotmat[2].z * 2.0;
        }

        universe[n as usize].flags |= flags;
        universe[n as usize].rotz /= 2;
        universe[n as usize].rotz *= 2;
        universe[n as usize].bravery = bravery;

        if (da_type == SHIP_CARGO) || (da_type == SHIP_ALLOY) || (da_type == SHIP_ROCK) {
            universe[n as usize].rotz = ((rand255() * 2) & 255) - 128;
            universe[n as usize].rotx = ((rand255() * 2) & 255) - 128;
            universe[n as usize].velocity = rand255() & 15;
        }
    }
}
/*
void activate_ecm (int ours)
{
    if (ecm_active == 0)
    {
        ecm_active = 32;
        ecm_ours = ours;
        snd_play_sample (SND_ECM);
    }
}


void time_ecm (void)
{
    if (ecm_active != 0)
    {
        ecm_active--;
        if (ecm_ours)
            decrease_energy (-1);
    }
}
*/

pub fn arm_missile(cmdr: &Commander, params: &mut GameParams) {
    if (cmdr.missiles != 0) && (params.myship.missile_target == MISSILE_UNARMED) {
        params.myship.missile_target = MISSILE_ARMED;
    }
}

pub fn unarm_missile(params: &mut GameParams, sample_list: &[Sound]) {
    params.myship.missile_target = MISSILE_UNARMED;
    snd_play_sample(sample_list, SND_BOOP);
}

pub fn fire_missile(
    universe: &mut [UnivObject],
    params: &mut GameParams,
    cmdr: &mut Commander,
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    sample_list: &[Sound],
) {
    let mut rotmat = START_MATRIX;

    // warning
    if params.myship.missile_target < 0 {
        return;
    }

    rotmat[2].z = 1.0;
    rotmat[0].x = -1.0;

    let newship = add_new_ship(
        SHIP_MISSILE,
        0.0,
        -28.0,
        14.0,
        &rotmat,
        0,
        0,
        universe,
        ship_list,
        ship_count,
    );

    match newship {
        Some(n) => {
            universe[n as usize].velocity = params.flight_speed * 2;
            universe[n as usize].flags = FLG_ANGRY;
            universe[n as usize].target = params.myship.missile_target;

            if universe[params.myship.missile_target as usize].da_type > SHIP_ROCK {
                universe[params.myship.missile_target as usize].flags |= FLG_ANGRY;
            }
            if cmdr.missiles > 0 {
                cmdr.missiles -= 1;
            }
            params.myship.missile_target = MISSILE_UNARMED;

            snd_play_sample(sample_list, SND_MISSILE);
        }
        None => {
            info_message("Missile Jammed".to_string(), params, sample_list);
        }
    }
}
pub fn track_object(ship: &mut UnivObject, direction: f32, nvec: Vector) {
    let rat = 3;
    let rat2 = 0.111;

    let mut dir = vector_dot_product(&nvec, &ship.rotmat[1]);

    if (direction < -0.861) {
        ship.rotx = if dir < 0.9 { 7 } else { -7 };
        ship.rotz = 0;
        return;
    }

    ship.rotx = 0;

    if (((dir) * 2.0) >= rat2) {
        ship.rotx = if dir < 0.0 { rat } else { -rat };
    }

    if ((ship.rotz).abs() < 16) {
        dir = vector_dot_product(&nvec, &ship.rotmat[0]);

        ship.rotz = 0;

        if (((dir) * 2.0) > rat2) {
            ship.rotz = if dir < 0.0 { rat } else { -rat };

            if (ship.rotx < 0) {
                ship.rotz = -ship.rotz;
            }
        }
    }
}

pub fn missile_tactics(
    un: DaType,
    universe: &mut [UnivObject],
    params: &mut GameParams,
    sample_list: &[Sound],
    cmdr: &mut Commander,
) {
    let mut vec = START_VECTOR;
    let mut nvec;
    let mut direction;
    let cnt2: f32 = 0.223;

    if (params.myship.ecm_active) {
        snd_play_sample(sample_list, SND_EXPLODE);
        universe[un as usize].flags |= FLG_DEAD;
        return;
    }

    if (universe[un as usize].target == 0) {
        if (universe[un as usize].distance < 256) {
            universe[un as usize].flags |= FLG_DEAD;
            snd_play_sample(sample_list, SND_EXPLODE);
            damage_ship(
                250,
                {
                    if universe[un as usize].location.z >= 0.0 {
                        true
                    } else {
                        false
                    }
                },
                params,
            );
            return;
        }
        vec.x = universe[un as usize].location.x;
        vec.y = universe[un as usize].location.y;
        vec.z = universe[un as usize].location.z;
    } else {
        let target = universe[universe[un as usize].target as usize];

        vec.x = universe[un as usize].location.x - target.location.x;
        vec.y = universe[un as usize].location.y - target.location.y;
        vec.z = universe[un as usize].location.z - target.location.z;

        if (((vec.x).abs() < 256.0) && ((vec.y).abs() < 256.0) && ((vec.z).abs() < 256.0)) {
            universe[un as usize].flags |= FLG_DEAD;

            if ((target.da_type != SHIP_CORIOLIS) && (target.da_type != SHIP_DODEC)) {
                explode_object(
                    universe[un as usize].target,
                    cmdr,
                    universe,
                    params,
                    sample_list,
                );
            } else {
                snd_play_sample(sample_list, SND_EXPLODE);
            }
            return;
        }

        if ((rand255() < 16) && (target.flags & FLG_HAS_ECM) != 0) {
            // activate_ecm(0);
            // crst
            return;
        }
    }

    nvec = unit_vector(&vec);
    direction = vector_dot_product(&nvec, &universe[un as usize].rotmat[2]);
    nvec.x = -nvec.x;
    nvec.y = -nvec.y;
    nvec.z = -nvec.z;
    direction = -direction;

    track_object(&mut universe[un as usize], direction, nvec);

    if (direction <= -0.167) {
        universe[un as usize].acceleration = -2;
        return;
    }

    if (direction >= cnt2) {
        universe[un as usize].acceleration = 3;
        return;
    }

    if (universe[un as usize].velocity < 6) {
        universe[un as usize].acceleration = 3;
    } else {
        if (rand255() >= 200) {
            universe[un as usize].acceleration = -2;
        }
    }
    return;
}
fn launch_loot(
    un: DaType,
    loot: DaType,
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
    universe: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
) {
    let mut cnt;

    if (loot == SHIP_ROCK) {
        cnt = rand255() & 3;
    } else {
        cnt = rand255();
        if (cnt >= 128) {
            return;
        }

        cnt &= ship_list[universe[un as usize].da_type as usize].max_loot;
        cnt &= 15;
    }

    for i in 0..cnt {
        launch_enemy(un, loot, 0, 0, universe, ship_list, ship_count);
    }
}

fn in_target(
    da_type: DaType,
    x: f32,
    y: f32,
    z: f32,
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
) -> bool {
    let size: f32;

    if (z < 0.0) {
        return false;
    }

    size = ship_list[da_type as usize].size * GFX_SCALE * 100.0;

    return ((x * x + y * y) <= size);
}

fn make_angry(un: DaType, universe: &mut [UnivObject]) {
    let da_type = universe[un as usize].da_type;
    let flags = universe[un as usize].flags;

    if (flags & FLG_INACTIVE) != 0 {
        return;
    }

    if ((da_type == SHIP_CORIOLIS) || (da_type == SHIP_DODEC)) {
        universe[un as usize].flags |= FLG_ANGRY;
        return;
    }

    if (da_type > SHIP_ROCK) {
        universe[un as usize].rotx = 4;
        universe[un as usize].acceleration = 2;
        universe[un as usize].flags |= FLG_ANGRY;
    }
}

fn explode_object(
    un: DaType,
    cmdr: &mut Commander,
    universe: &mut [UnivObject],
    params: &mut GameParams,
    sample_list: &[Sound],
) {
    cmdr.score += 1;

    if ((cmdr.score & 255) == 0) {
        info_message("Right On Commander!".to_string(), params, sample_list);
    }

    snd_play_sample(sample_list, SND_EXPLODE);
    // crst
    universe[un as usize].flags |= FLG_DEAD;

    if (universe[un as usize].da_type == SHIP_CONSTRICTOR) {
        cmdr.mission = 2;
    }
}
pub fn check_target(
    un: DaType,
    flip: &mut UnivObject,
    universe: &mut [UnivObject],
    params: &mut GameParams,
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
    cmdr: &mut Commander,
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    sample_list: &[Sound],
) {
    let mut univ = universe[un as usize];

    if (in_target(
        univ.da_type,
        flip.location.x,
        flip.location.y,
        flip.location.z,
        ship_list,
    )) {
        // warning
        if ((params.myship.missile_target == MISSILE_ARMED)
            && (univ.da_type != SHIP_SUN && univ.da_type != SHIP_PLANET))
        {
            params.myship.missile_target = un;
            info_message("Target Locked".to_string(), params, sample_list);
            snd_play_sample(sample_list, SND_BEEP);
        }

        if (params.myship.laser != 0) {
            snd_play_sample(sample_list, SND_HIT_ENEMY);
            clear_background(RED);

            if ((univ.da_type != SHIP_CORIOLIS) && (univ.da_type != SHIP_DODEC)) {
                if ((univ.da_type == SHIP_CONSTRICTOR) || (univ.da_type == SHIP_COUGAR)) {
                    if (params.myship.laser == (MILITARY_LASER & 127)) {
                        univ.energy -= params.myship.laser / 4;
                    }
                } else {
                    univ.energy -= params.myship.laser;
                }
            }

            if (univ.energy <= 0) {
                explode_object(un, cmdr, universe, params, sample_list);

                if (univ.da_type == SHIP_ASTEROID) {
                    if (params.myship.laser == (MINING_LASER & 127)) {
                        launch_loot(un, SHIP_ROCK, ship_list, universe, ship_count);
                    }
                } else {
                    launch_loot(un, SHIP_ALLOY, ship_list, universe, ship_count);
                    launch_loot(un, SHIP_CARGO, ship_list, universe, ship_count);
                }
            }

            make_angry(un, universe);
        }
    }
}
