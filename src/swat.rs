use macroquad::{
    color::{RED, WHITE},
    shapes::{draw_line, draw_triangle},
};

use crate::{
    elite::{Commander, ShipData},
    gfx::{GFX_SCALE, GFX_VIEW_BY},
    planet::PlanetData,
    shipdata::{NO_OF_SHIPS, SHIP_CORIOLIS, SHIP_DODEC, SHIP_PLANET, SHIP_SUN},
    sound::SND_PULSE,
    space::UnivObject,
    stars::rand255,
    vector::{Matrix, Vector, START_MATRIX},
    Config, GameParams, My, FLG_ANGRY, FLG_BOLD, FLG_CLOAKED, FLG_FLY_TO_PLANET, FLG_INACTIVE,
    FLG_POLICE, FLG_SLOW, MAX_UNIV_OBJECTS, THICKNESS,
};

pub const initial_flags: [My; NO_OF_SHIPS + 1] = [
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
pub struct Swat {
    ecm_active: My,
    missile_target: My,
    in_battle: My,
}

impl Swat {
    pub fn new() -> Self {
        Self {
            ecm_active: 0,
            missile_target: MISSILE_UNARMED,
            in_battle: 0,
        }
    }

    pub fn set(ecm_active: My, missile_target: My, in_battle: My) -> Self {
        Self {
            ecm_active,
            missile_target,
            in_battle,
        }
    }
}
pub fn reset_weapons(params: &mut GameParams) {
    params.myship.laser_temp = 0;
    params.myship.laser_counter = 0;
    params.myship.laser = 0;
    params.myship.ecm_active = 0;
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
        match params.current_screen {
            SCR_FRONT_VIEW => {
                params.myship.laser = cmdr.front_laser;
            }

            SCR_REAR_VIEW => {
                params.myship.laser = cmdr.rear_laser;
            }

            SCR_RIGHT_VIEW => {
                params.myship.laser = cmdr.right_laser;
            }

            SCR_LEFT_VIEW => {
                params.myship.laser = cmdr.left_laser;
            }

            _ => {
                params.myship.laser = 0;
            }
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

    return 0;
}

pub fn cool_laser(params: &mut GameParams) {
    params.myship.laser = 0;

    if (params.myship.laser_temp > 0) {
        params.myship.laser_temp -= 1;
    }

    if (params.myship.laser_counter > 0) {
        params.myship.laser_counter -= 1;
    }

    if (params.myship.laser_counter > 0) {
        params.myship.laser_counter -= 1;
    }
}
pub fn snd_play_sample(snd_pulse: usize) {
    println!("snd_play_sample()")
}
pub fn clear_universe(
    univ: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    in_battle: &mut bool,
) {
    for i in 0..MAX_UNIV_OBJECTS {
        univ[i as usize].da_type = 0;
    }

    for i in 0..NO_OF_SHIPS {
        ship_count[i] = 0;
    }

    *in_battle = false;
}
pub fn remove_ship(
    un: usize,
    universe: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
) {
    let da_type;
    let mut rotmat: Matrix = START_MATRIX;
    let px: My;
    let mut py: My;
    let pz: My;

    da_type = universe[un].da_type;

    if (da_type == 0) {
        return;
    }

    if (da_type > 0) {
        ship_count[da_type as usize] -= 1;
    }

    universe[un].da_type = 0;

    // check_missiles (un);

    if ((da_type == SHIP_CORIOLIS) || (da_type == SHIP_DODEC)) {
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
    universe[1].da_type = 0;
    add_new_ship(
        station, sx, sy, sz, rotmat, 0, -127, universe, ship_list, ship_count,
    );
}

pub fn add_new_ship(
    ship_type: My,
    x: f32,
    y: f32,
    z: f32,
    rotmat: &Matrix,
    rotx: My,
    rotz: My,
    universe: &mut [UnivObject],
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
) -> My {
    for i in 0..MAX_UNIV_OBJECTS {
        if (universe[i].da_type == 0) {
            universe[i].da_type = ship_type;
            universe[i].location.x = x as f32;
            universe[i].location.y = y as f32;
            universe[i].location.z = z as f32;

            universe[i].distance = (x * x + y * y + z * z).sqrt() as My;

            universe[i].rotmat[0] = rotmat[0];
            universe[i].rotmat[1] = rotmat[1];
            universe[i].rotmat[2] = rotmat[2];

            universe[i].rotx = rotx;
            universe[i].rotz = rotz;

            universe[i].velocity = 0;
            universe[i].acceleration = 0;
            universe[i].bravery = 0;
            universe[i].target = 0;

            universe[i].flags = initial_flags[ship_type as usize];

            if ((ship_type != SHIP_PLANET) && (ship_type != SHIP_SUN)) {
                universe[i].energy = ship_list[ship_type as usize].energy;
                universe[i].missiles = ship_list[ship_type as usize].missiles;
                ship_count[ship_type as usize] += 1;
            }

            return i as My;
        }
    }

    return -1;
}
