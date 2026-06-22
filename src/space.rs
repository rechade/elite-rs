use crate::{
    elite::{Commander, ShipData, SCR_ESCAPE_POD, SCR_GAME_OVER, SCR_INTRO_ONE, SCR_INTRO_TWO},
    pilot::disengage_auto_pilot,
    planet::GalaxySeed,
    shipdata::{
        NO_OF_SHIPS, SHIP_CONSTRICTOR, SHIP_CORIOLIS, SHIP_COUGAR, SHIP_DODEC, SHIP_PLANET,
        SHIP_SUN, SHIP_VIPER,
    },
    sound::{SND_EXPLODE, SND_LAUNCH},
    stars::{create_new_stars, Stars},
    swat::{clear_universe, remove_ship, reset_weapons, snd_play_sample},
    threed::draw_ship,
    vector::{Matrix, Vector, START_MATRIX},
    Config, GameParams, FLG_DEAD, FLG_FIRING, FLG_REMOVE, SCR_BREAK_PATTERN,
};

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
#[derive(Clone, Copy)]
pub struct UnivObject {
    pub location: Vector,
    pub rotmat: Matrix,
    pub da_type: i16,
    pub rotx: i16,
    pub rotz: i16,
    pub flags: i16,
    pub energy: i16,
    pub velocity: u8,
    pub acceleration: u8,
    pub missiles: i16,
    pub target: u8,
    pub bravery: u8,
    pub exp_delta: u8,
    pub exp_seed: i32,
    pub distance: i32,
}

impl UnivObject {
    pub fn new(
        location: Vector,
        rotmat: Matrix,
        da_type: i16,
        rotx: i16,
        rotz: i16,
        flags: i16,
        energy: i16,
        velocity: u8,
        acceleration: u8,
        missiles: i16,
        target: u8,
        bravery: u8,
        exp_delta: u8,
        exp_seed: i32,
        distance: i32,
    ) -> Self {
        Self {
            location,
            rotmat,
            da_type,
            rotx,
            rotz,
            flags,
            energy,
            velocity,
            acceleration,
            missiles,
            target,
            bravery,
            exp_delta,
            exp_seed,
            distance,
        }
    }
}

const MAX_UNIV_OBJECTS: usize = 20;
pub struct Space {
    flight_climb: i16,
    flight_roll: i16,
    flight_speed: i16,
    destination_planet: GalaxySeed,
    hyper_ready: bool,
    hyper_countdown: i16,
    hyper_name: [char; 16],
    hyper_distance: i16,
    hyper_galactic: bool,
}
pub fn dock_player(params: &mut GameParams) {
    disengage_auto_pilot(params);
    params.docked = true;
    params.flight_speed = 0;
    params.flight_roll = 0;
    params.flight_climb = 0;
    params.front_shield = 255;
    params.aft_shield = 255;
    params.energy = 255;
    params.myship.altitude = 255;
    params.myship.cabtemp = 30;
    reset_weapons(params);
}
pub fn launch_player(
    params: &mut GameParams,
    cmdr: &mut Commander,
    da_stars: &mut Stars,
    univ: &mut [UnivObject],
    ship_count: &mut [i16; NO_OF_SHIPS + 1],
) {
    let mut rotmat: Matrix = START_MATRIX;

    params.docked = false;
    params.flight_speed = 12;
    params.flight_roll = -15;
    params.flight_climb = 0;
    // cmdr.legal_status |= carrying_contraband();
    create_new_stars(da_stars, params);
    clear_universe(univ, ship_count, &mut params.in_battle);
    // generate_landscape(docked_planet.a * 251 + docked_planet.b);
    // set_init_matrix(rotmat);
    // add_new_ship(SHIP_PLANET, 0, 0, 65536, rotmat, 0, 0);

    rotmat[2].x = -rotmat[2].x;
    rotmat[2].y = -rotmat[2].y;
    rotmat[2].z = -rotmat[2].z;
    // add_new_station(0, 0, -256, rotmat);

    params.current_screen = SCR_BREAK_PATTERN;
    snd_play_sample(SND_LAUNCH);
}
/*
 * Update all the objects in the universe and render them.
 */

fn update_universe(
    universe: &mut [UnivObject; MAX_UNIV_OBJECTS],
    cmdr: &mut Commander,
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
    params: &mut GameParams,
    ship_count: &mut [i16; NO_OF_SHIPS + 1],
    config: &Config,
) {
    let mut da_type;
    let mut bounty;
    let mut da_string: String;
    // char str[80];
    let mut flip: UnivObject;

    for i in 0..MAX_UNIV_OBJECTS {
        da_type = universe[i].da_type;

        if da_type != 0 {
            if (universe[i].flags & FLG_REMOVE) != 0 {
                if da_type == SHIP_VIPER {
                    cmdr.legal_status |= 64;
                }

                bounty = ship_list[da_type as usize].bounty;

                if ((bounty != 0) && (!params.witchspace)) {
                    cmdr.credits += bounty;
                    // sprintf (str, "%d.%d CR", cmdr.credits / 10, cmdr.credits % 10);
                    // info_message (str);
                }

                remove_ship(i, universe, ship_count, ship_list);
                continue;
            }

            let var_name = FLG_DEAD;
            if ((params.detonate_bomb != 0)
                && ((universe[i].flags & var_name) == 0)
                && (da_type != SHIP_PLANET)
                && (da_type != SHIP_SUN)
                && (da_type != SHIP_CONSTRICTOR)
                && (da_type != SHIP_COUGAR)
                && (da_type != SHIP_CORIOLIS)
                && (da_type != SHIP_DODEC))
            {
                snd_play_sample(SND_EXPLODE);
                universe[i].flags |= FLG_DEAD;
            }

            if ((params.current_screen != SCR_INTRO_ONE)
                && (params.current_screen != SCR_INTRO_TWO)
                && (params.current_screen != SCR_GAME_OVER)
                && (params.current_screen != SCR_ESCAPE_POD))
            {
                // tactics (i);
            }

            // move_univ_object (&universe[i]);

            flip = universe[i];
            // switch_to_view (&flip);

            if (da_type == SHIP_PLANET) {
                if ((ship_count[SHIP_CORIOLIS as usize] == 0)
                    && (ship_count[SHIP_DODEC as usize] == 0)
                    && (universe[i].distance < 65792))
                // was 49152
                {
                    // make_station_appear();
                }

                draw_ship(&mut flip, params, config, ship_list);
                continue;
            }

            if (da_type == SHIP_SUN) {
                draw_ship(&mut flip, params, config, ship_list);
                continue;
            }

            if (universe[i].distance < 170) {
                if ((da_type == SHIP_CORIOLIS) || (da_type == SHIP_DODEC)) {
                    // check_docking (i);
                } else {
                    // scoop_item(i);
                }

                continue;
            }

            if (universe[i].distance > 57344) {
                remove_ship(i, universe, ship_count, ship_list);
                continue;
            }

            draw_ship(&mut flip, params, config, ship_list);

            universe[i].flags = flip.flags;
            universe[i].exp_seed = flip.exp_seed;
            universe[i].exp_delta = flip.exp_delta;

            universe[i].flags &= !FLG_FIRING;

            if (universe[i].flags & FLG_DEAD) != 0 {
                continue;
            }

            // check_target(i, &flip);
        }
    }

    params.detonate_bomb = 0;
}
