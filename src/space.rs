use crate::{
    GameParams, SCR_BREAK_PATTERN,
    elite::Commander,
    pilot::disengage_auto_pilot,
    planet::GalaxySeed,
    sound::SND_LAUNCH,
    stars::{Stars, create_new_stars},
    swat::{reset_weapons, snd_play_sample},
};

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
pub fn launch_player(params: &mut GameParams, cmdr: &mut Commander, da_stars: &mut Stars) {
    // Matrix rotmat;

    params.docked = false;
    params.flight_speed = 12;
    params.flight_roll = -15;
    params.flight_climb = 0;
    // cmdr.legal_status |= carrying_contraband();
    create_new_stars(da_stars, params);
    // // clear_universe();
    // // generate_landscape(docked_planet.a * 251 + docked_planet.b);
    // // set_init_matrix(rotmat);
    // // add_new_ship(SHIP_PLANET, 0, 0, 65536, rotmat, 0, 0);

    // rotmat[2].x = -rotmat[2].x;
    // rotmat[2].y = -rotmat[2].y;
    // rotmat[2].z = -rotmat[2].z;
    // add_new_station(0, 0, -256, rotmat);

    params.current_screen = SCR_BREAK_PATTERN;
    snd_play_sample(SND_LAUNCH);
}
