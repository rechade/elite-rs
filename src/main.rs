#![allow(warnings)]
use macroquad::prelude::*;
use std::{thread, time};

use crate::{
    elite::{Commander, PlayerShip, SCR_FRONT_VIEW, SCR_REAR_VIEW,*}, gfx::GFX_SCALE, shipdata::NO_OF_SHIPS, sound::SND_BEEP, space::{UnivObject, dock_player, launch_player}, stars::{Stars, create_new_stars, flip_stars, update_starfield}, swat::{clear_universe, draw_laser_lines, fire_laser, snd_play_sample}, vector::{START_MATRIX, START_VECTOR}
};

pub(crate) mod elite;
pub(crate) mod gfx;
pub(crate) mod pilot;
pub(crate) mod planet;
pub(crate) mod sound;
pub(crate) mod space;
pub(crate) mod stars;
pub(crate) mod swat;
pub(crate) mod trade;
pub(crate) mod vector;
pub(crate) mod shipdata;
pub(crate) mod threed;
const FIRE_KEY: KeyCode = KeyCode::A;
const DOCK_KEY: KeyCode = KeyCode::C;
const ECM_KEY: KeyCode = KeyCode::E;
const FIND_KEY: KeyCode = KeyCode::F;
const HYPERSPACE_KEY: KeyCode = KeyCode::H;
const JUMP_KEY: KeyCode = KeyCode::J;
const FIRE_MISSILE_KEY: KeyCode = KeyCode::M;
const PAUSE_KEY: KeyCode = KeyCode::P;
const TARGET_MISSILE_KEY: KeyCode = KeyCode::T;
const UNARM_MISSILE_KEY: KeyCode = KeyCode::U;
const INCREASE_SPEED_KEY: KeyCode = KeyCode::Space;
const DECREASE_SPEED_KEY: KeyCode = KeyCode::Slash;
const ENERGY_BOMB_KEY: KeyCode = KeyCode::Tab;
const THICKNESS: f32 = 1.0;
// const SCR_INTRO_ONE: i16 = 1;
// const SCR_INTRO_TWO: i16 = 2;
// const SCR_GALACTIC_CHART: i16 = 3;
// const SCR_SHORT_RANGE: i16 = 4;
// const SCR_PLANET_DATA: i16 = 5;
// const SCR_MARKET_PRICES: i16 = 6;
// const SCR_CMDR_STATUS: i16 = 7;
// const SCR_FRONT_VIEW: i16 = 8;
// const SCR_REAR_VIEW: i16 = 9;
// const SCR_LEFT_VIEW: i16 = 10;
// const SCR_RIGHT_VIEW: i16 = 11;
// const SCR_BREAK_PATTERN: i16 = 12;
// const SCR_INVENTORY: i16 = 13;
// const SCR_EQUIP_SHIP: i16 = 14;
// const SCR_OPTIONS: i16 = 15;
// const SCR_LOAD_CMDR: i16 = 16;
// const SCR_SAVE_CMDR: i16 = 17;
// const SCR_QUIT: i16 = 18;
// const SCR_GAME_OVER: i16 = 19;
// const SCR_SETTINGS: i16 = 20;
// const SCR_ESCAPE_POD: i16 = 21;

const PULSE_LASER: i16 = 0x0F;
const BEAM_LASER: i16 = 0x8F;
const MILITARY_LASER: i16 = 0x97;
const MINING_LASER: i16 = 0x32;

const FLG_DEAD: i16 = 1;
const FLG_REMOVE: i16 = 2;
const FLG_EXPLOSION: i16 = 4;
const FLG_ANGRY: i16 = 8;
const FLG_FIRING: i16 = 16;
const FLG_HAS_ECM: i16 = 32;
const FLG_HOSTILE: i16 = 64;
const FLG_CLOAKED: i16 = 128;
const FLG_FLY_TO_PLANET: i16 = 256;
const FLG_FLY_TO_STATION: i16 = 512;
const FLG_INACTIVE: i16 = 1024;
const FLG_SLOW: i16 = 2048;
const FLG_BOLD: i16 = 4096;
const FLG_POLICE: i16 = 8192;

const MAX_UNIV_OBJECTS: usize = 100;
struct Config {
    speed_cap: i16,
    wireframe: i16,
    anti_alias_gfx: i16,
    planet_render_style: i16,
    hoopy_casinos: i16,
    instant_dock: i16,
}

impl Config {
    fn new() -> Self {
        Self {
            speed_cap: 0,
            wireframe: 1,
            anti_alias_gfx: 0,
            planet_render_style: 0,
            hoopy_casinos: 0,
            instant_dock: 0,
        }
    }
    fn set(
        speed_cap: i16,
        wireframe: i16,
        anti_alias_gfx: i16,
        planet_render_style: i16,
        hoopy_casinos: i16,
        instant_dock: i16,
    ) -> Self {
        Self {
            speed_cap,
            wireframe,
            anti_alias_gfx,
            planet_render_style,
            hoopy_casinos,
            instant_dock,
        }
    }
}
struct GameParams {
    current_screen: i16,
    flight_speed: i16,
    flight_roll: i16,
    flight_climb: i16,
    docked: bool,
    front_shield: i16,
    aft_shield: i16,
    energy: i16,
    draw_lasers: i16,
    mcount: i16,
    message_count: i16,
    hyper_ready: bool,
    detonate_bomb: i16,
    find_input: bool,
    witchspace: bool,
    game_paused: bool,
    auto_pilot: bool,
    cross_x: i16,
    cross_y: i16,
    old_cross_x: i16,
    old_cross_y: i16,
    cross_timer: i16,
    myship: PlayerShip,
    message_string: [char; 80],
    rolling: bool,
    climbing: bool,
    have_joystick: i16,
    finish: bool,
    game_over: bool,
    find_name: [char; 20],
    in_battle: bool
}
impl GameParams {
    pub fn increase_flight_roll(&mut self) {
        if self.flight_roll < self.myship.max_roll {
            self.flight_roll += 1;
        }
    }

    pub fn decrease_flight_roll(&mut self) {
        if self.flight_roll > -self.myship.max_roll {
            self.flight_roll -= 1;
        }
    }

    pub fn increase_flight_climb(&mut self) {
        if self.flight_climb < self.myship.max_climb {
            self.flight_climb += 1;
        }
    }

    pub fn decrease_flight_climb(&mut self) {
        if self.flight_climb > -self.myship.max_climb {
            self.flight_climb -= 1;
        }
    }
}
struct ScanConfig {
    scanner_cx: i16,
    scanner_cy: i16,

    compass_centre_x: i16,
    compass_centre_y: i16,
}

impl ScanConfig {
    fn new() -> Self {
        Self {
            scanner_cx: 0,
            scanner_cy: 0,
            compass_centre_x: 0,
            compass_centre_y: 0,
        }
    }
    fn set(scanner_cx: i16, scanner_cy: i16, compass_centre_x: i16, compass_centre_y: i16) -> Self {
        Self {
            scanner_cx,
            scanner_cy,
            compass_centre_x,
            compass_centre_y,
        }
        // scanner_cy += 385;
        // compass_centre_y += 385;
    }
}

impl GameParams {
    fn set(
        current_screen: i16,
        flight_speed: i16,
        flight_roll: i16,
        flight_climb: i16,
        docked: bool,
        front_shield: i16,
        aft_shield: i16,
        energy: i16,
        draw_lasers: i16,
        mcount: i16,
        message_count: i16,
        hyper_ready: bool,
        detonate_bomb: i16,
        find_input: bool,
        witchspace: bool,
        game_paused: bool,
        auto_pilot: bool,
        cross_x: i16,
        cross_y: i16,
        old_cross_x: i16,
        old_cross_y: i16,
        cross_timer: i16,
        myship: PlayerShip,
        message_string: [char; 80],
        rolling: bool,
        climbing: bool,
        have_joystick: i16,
        finish: bool,
        game_over: bool,
        find_name: [char; 20],
        in_battle: bool,
    ) -> Self {
        Self {
            current_screen,
            flight_speed,
            flight_roll,
            flight_climb,
            docked,
            front_shield,
            aft_shield,
            energy,
            draw_lasers,
            mcount,
            message_count,
            hyper_ready,
            detonate_bomb,
            find_input,
            witchspace,
            game_paused,
            auto_pilot,
            cross_x,
            cross_y,
            old_cross_x,
            old_cross_y,
            cross_timer,
            myship,
            message_string,
            rolling,
            climbing,
            have_joystick,
            find_name,
            finish,
            game_over,
            in_battle
        }
    }

    fn new() -> Self {
        Self {
            current_screen: 0,
            flight_speed: 0,
            flight_roll: 0,
            flight_climb: 0,
            docked: false,
            front_shield: 0,
            aft_shield: 0,
            energy: 0,
            draw_lasers: 0,
            mcount: 0,
            message_count: 0,
            hyper_ready: false,
            detonate_bomb: 0,
            find_input: false,
            witchspace: false,
            game_paused: false,
            auto_pilot: false,
            cross_x: 0,
            cross_y: 0,
            old_cross_x: 0,
            old_cross_y: 0,
            cross_timer: 0,
            myship: PlayerShip::new(),
            message_string: ['c'; 80],
            rolling: false,
            climbing: false,
            have_joystick: 0,
            finish: false,
            game_over: false,
            find_name: ['b'; 20],
            in_battle: false,
        }
    }
}

#[macroquad::main("EliteRS")]
async fn main() {
    let mut ship_count: [i16;NO_OF_SHIPS + 1]   =[0;NO_OF_SHIPS + 1];  /* many */

let esccaps_point: Vec<ShipPoint> = vec![
    ShipPoint::new(-7, 0, 36, 31, 1, 2, 3, 3),
    ShipPoint::new(-7, -14, -12, 31, 0, 2, 3, 3),
    ShipPoint::new(-7, 14, -12, 31, 0, 1, 3, 3),
    ShipPoint::new(21, 0, 0, 31, 0, 1, 2, 2),
];

let esccaps_line: Vec<ShipLine> = vec![
    ShipLine::new(31, 2, 3, 0, 1),
    ShipLine::new(31, 0, 3, 1, 2),
    ShipLine::new(31, 0, 1, 2, 3),
    ShipLine::new(31, 1, 2, 3, 0),
    ShipLine::new(31, 1, 3, 0, 2),
    ShipLine::new(31, 0, 2, 3, 1),
];

let esccaps_face_normal: Vec<ShipFaceNormal> = vec![
    ShipFaceNormal::new(31, 52, 0, -122),
    ShipFaceNormal::new(31, 39, 103, 30),
    ShipFaceNormal::new(31, 39, -103, 30),
    ShipFaceNormal::new(31, -112, 0, 0),
];

let esccaps_data: ShipData = ShipData {
    name: put_into_name("Escape Capsule                  "),
    num_points: 4,
    num_lines: 6,
    num_faces: 4,
    max_loot: 0,
    scoop_type: 2,
    size: 256.0,
    front_laser: 0,
    bounty: 0,
    vanish_point: 8,
    energy: 17,
    velocity: 8,
    missiles: 0,
    laser_strength: 0,
    points: esccaps_point,
    lines: esccaps_line,
    normals: esccaps_face_normal,
};
// let ship_list: [ShipData; NO_OF_SHIPS + 1] = [
let ship_list: [ShipData; NO_OF_SHIPS + 1] = [
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
    esccaps_data.clone(),
];

    let mut universe: Vec<UnivObject> = vec![];
    for i in 0..MAX_UNIV_OBJECTS {
        universe.push(UnivObject::new(START_VECTOR,START_MATRIX,0,0,0,0,0,0,0,0,0,0,0,0,0));
    }
    let frame_duration = time::Duration::from_millis(40);
    let mut config: Config = Config::new();
    let mut scan_config: ScanConfig = ScanConfig::new();
    let mut cmdr = Commander::new();
    let mut params: GameParams = GameParams::new();
    let mut da_stars: Stars = Stars::new();
    create_new_stars(&mut da_stars, &params);
    initialise_game(&mut params);

    clear_universe(&mut universe, &mut ship_count, &mut params.in_battle);
    while !params.finish {
        params.game_over = false;
        dock_player(&mut params);

        // update_console();

        params.current_screen = SCR_FRONT_VIEW;
        // run_first_intro_screen();
        // run_second_intro_screen();

        params.old_cross_x = -1;
        params.old_cross_y = -1;

        dock_player(&mut params);
        // display_commander_status ();
        while !params.game_over {
            // snd_update_sound();
            // gfx_set_clip_region (1, 1, 510, 383);

            params.rolling = false;
            params.climbing = false;

            handle_flight_keys(&mut params, &config, &mut cmdr, &mut da_stars, &mut universe, &mut ship_count);

            if params.game_paused {
                continue;
            }

            if params.message_count > 0 {
                params.message_count -= 1;
            }

            if !params.rolling {
                if params.flight_roll > 0 {
                    params.decrease_flight_roll();
                }

                if params.flight_roll < 0 {
                    params.increase_flight_roll();
                }
            }

            if !params.climbing {
                if params.flight_climb > 0 {
                    params.decrease_flight_climb();
                }

                if params.flight_climb < 0 {
                    params.increase_flight_climb();
                }
            }
            if !params.docked {
                if (params.current_screen == SCR_FRONT_VIEW)
                    || (params.current_screen == SCR_REAR_VIEW)
                    || (params.current_screen == SCR_LEFT_VIEW)
                    || (params.current_screen == SCR_RIGHT_VIEW)
                    || (params.current_screen == SCR_INTRO_ONE)
                    || (params.current_screen == SCR_INTRO_TWO)
                    || (params.current_screen == SCR_GAME_OVER) 
                {
                    clear_background(BLACK);
                    update_starfield(&mut da_stars, &params);
                }

                if params.auto_pilot {
                    // auto_dock();
                    // if ((mcount & 127) == 0){
                    // 	info_message ("Docking Computers On");
                    // }
                }

                // update_universe ();

                if params.docked {
                    // update_console();
                    continue;
                }

                if ((params.current_screen == SCR_FRONT_VIEW)
                    || (params.current_screen == SCR_REAR_VIEW)
                    || (params.current_screen == SCR_LEFT_VIEW)
                    || (params.current_screen == SCR_RIGHT_VIEW))
                {
                    if (params.draw_lasers != 0) {
                        draw_laser_lines(&params, &config);
                        params.draw_lasers -= 1;
                    }

                    draw_laser_sights(&params, &cmdr);
                }

                if params.message_count > 0 {
                    // gfx_display_centre_text (358, message_string, 120, GFX_COL_WHITE);
                }

                if params.hyper_ready {
                    // display_hyper_status();
                    if (params.mcount & 3) == 0 {
                        // countdown_hyperspace();
                    }
                }

                params.mcount -= 1;
                if params.mcount < 0 {
                    params.mcount = 255;
                }

                if (params.mcount & 7) == 0 {
                    // regenerate_shields();
                }

                if (params.mcount & 31) == 10 {
                    if params.energy < 50 {
                        // info_message ("ENERGY LOW");
                        snd_play_sample(SND_BEEP);
                    }

                    // update_altitude();
                }

                if (params.mcount & 31) == 20 {
                    // update_cabin_temp();
                }

                if (params.mcount == 0) && (!params.witchspace) {
                    // random_encounter();
                }

                // cool_laser();
                // time_ecm();

                // update_console();
            }

            // clear_background(LIGHTGRAY);
            // if is_key_down(KeyCode::Right) {
            //     arrow_right(&mut params)
            // }
            // if is_key_down(KeyCode::Left) {
            //     arrow_left(&mut params)
            // }
            // if is_key_down(KeyCode::Down) {
            //     arrow_down(&mut params)
            // }
            // if is_key_down(KeyCode::Up) {
            //     arrow_up(&mut params)
            // }
            // dbg!(game_params.cross_x,game_params.cross_y);
            params.old_cross_x = params.cross_x;
            params.old_cross_y = params.cross_y;
            draw_cross(&params, params.old_cross_x, params.old_cross_y);
            draw_laser_sights(&params, &cmdr);
            update_starfield(&mut da_stars, &params);
            // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
            // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
            // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

            // draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);

            thread::sleep(frame_duration);
            next_frame().await
        }
    }
}

fn put_into_name(new_name: &str)->[char;32]  {
    let mut result =[' ';32];
    for (i,c) in new_name.chars().enumerate() {
        if i < result.len(){
        result[i]=c;
        }
    }
    result
}
/*
 * Initialise the game parameters.
 */

fn initialise_game(params: &mut GameParams) {
    // set_rand_seed(time(NULL));
    params.current_screen = SCR_INTRO_ONE;
    params.current_screen = SCR_FRONT_VIEW;

    // restore_saved_commander();

    params.flight_speed = 1;
    params.flight_roll = 0;
    params.flight_climb = 0;
    params.docked = true;
    params.front_shield = 255;
    params.aft_shield = 255;
    params.energy = 255;
    params.draw_lasers = 0;
    params.mcount = 0;
    params.message_count = 0;
    params.hyper_ready = false;
    params.detonate_bomb = 0;
    params.find_input = false;
    params.witchspace = false;
    params.game_paused = false;
    params.auto_pilot = false;

    // create_new_stars();
    // clear_universe();

    params.old_cross_x = -1;
    params.old_cross_y = -1;
    params.cross_x = -1;
    params.cross_y = -1;
    params.cross_timer = 0;

    params.myship.max_speed = 40; /* 0.27 Light Mach */
    params.myship.max_roll = 31;
    params.myship.max_climb = 8; /* CF 8 */
    params.myship.max_fuel = 70; /* 7.0 Light Years */
    params.message_string = ['c'; 80];
    params.rolling = false;
    params.climbing = false;
    params.have_joystick = 0;
    params.find_name = ['d'; 20];
}
fn finish_game(params: &mut GameParams) {
    params.finish = true;
    params.game_over = true;
}

/*
 * Move the planet chart cross hairs to specified position.
 */

fn move_cross(params: &mut GameParams, dx: i16, dy: i16) {
    params.cross_timer = 5;

    if params.current_screen == SCR_SHORT_RANGE {
        params.cross_x += dx * 4;
        params.cross_y += dy * 4;
        return;
    }

    if params.current_screen == SCR_GALACTIC_CHART {
        params.cross_x += dx * 2;
        params.cross_y += dy * 2;

        if params.cross_x < 1 {
            params.cross_x = 1;
        }

        if params.cross_x > 510 {
            params.cross_x = 510;
        }

        params.cross_y = params.cross_y.clamp(37, 293);
    }
}

/*
 * Draw the cross hairs at the specified position.
 */

fn draw_cross(params: &GameParams, cx: i16, cy: i16) {
    if params.current_screen == SCR_SHORT_RANGE {
        // gfx_set_clip_region(1, 37, 510, 339);
        // xor_mode(TRUE);
        draw_line(
            (cx - 16) as f32,
            cy as f32,
            (cx + 16) as f32,
            cy as f32,
            THICKNESS,
            RED,
        );
        draw_line(
            cx as f32,
            (cy - 16) as f32,
            cx as f32,
            (cy + 16) as f32,
            THICKNESS,
            RED,
        );
        // xor_mode(FALSE);
        // gfx_set_clip_region(1, 1, 510, 383);
        return;
    }

    if params.current_screen == SCR_GALACTIC_CHART {
        // gfx_set_clip_region(1, 37, 510, 293);
        // xor_mode(TRUE);
        draw_line(
            (cx - 8) as f32,
            cy as f32,
            (cx + 8) as f32,
            cy as f32,
            THICKNESS,
            RED,
        );
        draw_line(
            cx as f32,
            (cy - 8) as f32,
            cx as f32,
            (cy + 8) as f32,
            THICKNESS,
            RED,
        );
        // xor_mode(FALSE);
        // gfx_set_clip_region(1, 1, 510, 383);
    }
}

fn draw_laser_sights(params: &GameParams, cmdr: &Commander) {
    let mut laser: i16 = 0;
    let mut x1: i16;
    let mut y1: i16;
    let mut x2: i16;
    let mut y2: i16;

    match params.current_screen {
        SCR_FRONT_VIEW => {
            draw_text("Front View", 10.0, 32.0, 12.0, WHITE);
            laser = cmdr.front_laser;
        }

        SCR_REAR_VIEW => {
            draw_text("Rear View", 10.0, 32.0, 12.0, WHITE);
            laser = cmdr.rear_laser;
        }

        SCR_LEFT_VIEW => {
            draw_text("Left View", 10.0, 32.0, 12.0, WHITE);
            laser = cmdr.left_laser;
        }

        SCR_RIGHT_VIEW => {
            draw_text("Right View", 10.0, 32.0, 12.0, WHITE);
            laser = cmdr.right_laser;
        }
        _ => (),
    }

    if laser != 0 {
        x1 = 128 * GFX_SCALE;
        y1 = (96 - 8) * GFX_SCALE;
        y2 = (96 - 16) * GFX_SCALE;

        draw_line(
            (x1 - 1) as f32,
            y1 as f32,
            (x1 - 1) as f32,
            y2 as f32,
            THICKNESS,
            GRAY,
        );
        draw_line(x1 as f32, y1 as f32, x1 as f32, y2 as f32, THICKNESS, WHITE);
        draw_line(
            (x1 + 1) as f32,
            y1 as f32,
            (x1 + 1) as f32,
            y2 as f32,
            THICKNESS,
            GRAY,
        );

        y1 = (96 + 8) * GFX_SCALE;
        y2 = (96 + 16) * GFX_SCALE;

        draw_line(
            (x1 - 1) as f32,
            y1 as f32,
            (x1 - 1) as f32,
            y2 as f32,
            THICKNESS,
            GRAY,
        );
        draw_line(x1 as f32, y1 as f32, x1 as f32, y2 as f32, THICKNESS, WHITE);
        draw_line(
            (x1 + 1) as f32,
            y1 as f32,
            (x1 + 1) as f32,
            y2 as f32,
            THICKNESS,
            GRAY,
        );

        x1 = (128 - 8) * GFX_SCALE;
        y1 = 96 * GFX_SCALE;
        x2 = (128 - 16) * GFX_SCALE;

        draw_line(
            x1 as f32,
            (y1 - 1) as f32,
            x2 as f32,
            (y1 - 1) as f32,
            THICKNESS,
            GRAY,
        );
        draw_line(x1 as f32, y1 as f32, x2 as f32, y1 as f32, THICKNESS, WHITE);
        draw_line(
            x1 as f32,
            (y1 + 1) as f32,
            x2 as f32,
            (y1 + 1) as f32,
            THICKNESS,
            GRAY,
        );

        x1 = (128 + 8) * GFX_SCALE;
        x2 = (128 + 16) * GFX_SCALE;

        draw_line(
            x1 as f32,
            (y1 - 1) as f32,
            x2 as f32,
            (y1 - 1) as f32,
            THICKNESS,
            GRAY,
        );
        draw_line(x1 as f32, y1 as f32, x2 as f32, y1 as f32, THICKNESS, WHITE);
        draw_line(
            x1 as f32,
            (y1 + 1) as f32,
            x2 as f32,
            (y1 + 1) as f32,
            THICKNESS,
            GRAY,
        );
    }
}

fn arrow_right(params: &mut GameParams) {
    match params.current_screen {
        SCR_MARKET_PRICES => {
            // buy_stock();
        }

        SCR_SETTINGS => {
            // select_right_setting();
        }

        SCR_SHORT_RANGE | SCR_GALACTIC_CHART => {
            move_cross(params, 1, 0);
        }

        SCR_FRONT_VIEW | SCR_REAR_VIEW | SCR_RIGHT_VIEW | SCR_LEFT_VIEW => {
            if params.flight_roll > 0 {
                params.flight_roll = 0;
            } else {
                params.decrease_flight_roll();
                params.decrease_flight_roll();
                params.rolling = true;
            }
        }
        _ => (),
    }
}

fn arrow_left(params: &mut GameParams) {
    match params.current_screen {
        SCR_MARKET_PRICES => {
            // sell_stock();
        }

        SCR_SETTINGS => {
            // select_left_setting();
        }

        SCR_SHORT_RANGE | SCR_GALACTIC_CHART => {
            move_cross(params, -1, 0);
        }

        SCR_FRONT_VIEW | SCR_REAR_VIEW | SCR_RIGHT_VIEW | SCR_LEFT_VIEW => {
            if params.flight_roll < 0 {
                params.flight_roll = 0;
            } else {
                params.increase_flight_roll();
                params.increase_flight_roll();
                params.rolling = true;
            }
        }
        _ => (),
    }
}

fn arrow_up(params: &mut GameParams) {
    match params.current_screen {
        SCR_MARKET_PRICES => {
            // select_previous_stock();
        }

        SCR_EQUIP_SHIP => {
            // select_previous_equip();
        }

        SCR_OPTIONS => {
            // select_previous_option();
        }

        SCR_SETTINGS => {
            // select_up_setting();
        }

        SCR_SHORT_RANGE | SCR_GALACTIC_CHART => {
            move_cross(params, 0, -1);
        }

        SCR_FRONT_VIEW | SCR_REAR_VIEW | SCR_RIGHT_VIEW | SCR_LEFT_VIEW => {
            if params.flight_climb > 0 {
                params.flight_climb = 0;
            } else {
                params.decrease_flight_climb();
            }
            params.climbing = true;
        }
        _ => (),
    }
}

fn arrow_down(params: &mut GameParams) {
    match params.current_screen {
        SCR_MARKET_PRICES => {
            // select_next_stock();
        }

        SCR_EQUIP_SHIP => {
            // select_next_equip();
        }

        SCR_OPTIONS => {
            // select_next_option();
        }

        SCR_SETTINGS => {
            // select_down_setting();
        }

        SCR_SHORT_RANGE | SCR_GALACTIC_CHART => {
            move_cross(params, 0, 1);
        }

        SCR_FRONT_VIEW | SCR_REAR_VIEW | SCR_RIGHT_VIEW | SCR_LEFT_VIEW => {
            if params.flight_climb < 0 {
                params.flight_climb = 0;
            } else {
                params.increase_flight_climb();
            }
            params.climbing = true;
        }
        _ => (),
    }
}

fn handle_flight_keys(
    params: &mut GameParams,
    config: &Config,
    cmdr: &mut Commander,
    da_stars: &mut Stars,
    univ: &mut [UnivObject],
    ship_count: &mut [i16; NO_OF_SHIPS +1]
) {
    let mut keyasc;

    if params.docked
        && ((params.current_screen == SCR_MARKET_PRICES)
            || (params.current_screen == SCR_OPTIONS)
            || (params.current_screen == SCR_SETTINGS)
            || (params.current_screen == SCR_EQUIP_SHIP))
    {
        // kbd_read_key();
    }

    // kbd_poll_keyboard();

    /*
    if (have_joystick)
    {
        poll_joystick();

        if (joy[0].stick[0].axis[1].d1)
            arrow_up();

        if (joy[0].stick[0].axis[1].d2)
            arrow_down();

        if (joy[0].stick[0].axis[0].d1)
            arrow_left();

        if (joy[0].stick[0].axis[0].d2)
            arrow_right();

        if (joy[0].button[0].b)
            kbd_fire_pressed = 1;

        if (joy[0].button[1].b)
            kbd_inc_speed_pressed = 1;

        if (joy[0].button[2].b)
            kbd_dec_speed_pressed = 1;
    }
    */
    if params.game_paused {
        if is_key_down(KeyCode::R) {
            params.game_paused = false;
        }
        return;
    }

    if is_key_down(KeyCode::F1) {
        params.find_input = false;

        if params.docked {
            launch_player(params, cmdr, da_stars, univ,ship_count);
        } else {
            if params.current_screen != SCR_FRONT_VIEW {
                params.current_screen = SCR_FRONT_VIEW;
                flip_stars(da_stars, params);
            }
        }
    }

    if is_key_down(KeyCode::F2) {
        params.find_input = false;

        if !params.docked {
            if params.current_screen != SCR_REAR_VIEW {
                params.current_screen = SCR_REAR_VIEW;
                flip_stars(da_stars, params);
            }
        }
    }

    if is_key_down(KeyCode::F3) {
        params.find_input = false;

        if !params.docked {
            if params.current_screen != SCR_LEFT_VIEW {
                params.current_screen = SCR_LEFT_VIEW;
                flip_stars(da_stars, params);
            }
        }
    }

    if is_key_down(KeyCode::F4) {
        params.find_input = false;

        if params.docked {
            // equip_ship();
        } else {
            if params.current_screen != SCR_RIGHT_VIEW {
                params.current_screen = SCR_RIGHT_VIEW;
                flip_stars(da_stars, params);
            }
        }
    }

    if is_key_down(KeyCode::F5) {
        params.find_input = false;
        params.old_cross_x = -1;
        // display_galactic_chart();
    }

    if is_key_down(KeyCode::F6) {
        params.find_input = false;
        params.old_cross_x = -1;
        // display_short_range_chart();
    }

    if is_key_down(KeyCode::F7) {
        params.find_input = false;
        // display_data_on_planet();
    }

    if is_key_down(KeyCode::F8) && (!params.witchspace) {
        params.find_input = false;
        // display_market_prices();
    }

    if is_key_down(KeyCode::F9) {
        params.find_input = false;
        // display_commander_status();
    }

    if is_key_down(KeyCode::F10) {
        params.find_input = false;
        // display_inventory();
    }

    if is_key_down(KeyCode::F11) {
        params.find_input = false;
        // display_options();
    }

    if params.find_input {
        keyasc = kbd_read_key();

        if is_key_down(KeyCode::Enter) {
            params.find_input = false;
            // find_planet_by_name (find_name);
            return;
        }

        if is_key_down(KeyCode::Backspace) {
            // delete_find_char();
            return;
        }

        if isalpha(keyasc) {
            // add_find_char (keyasc);
        }
        return;
    }

    if is_key_down(KeyCode::Y) {
        y_pressed();
    }

    if is_key_down(KeyCode::N) {
        n_pressed();
    }

    if is_key_down(FIRE_KEY) {
        if (!params.docked) && (params.draw_lasers == 0) {
            params.draw_lasers = fire_laser(params, cmdr);
        }
    }

    if is_key_down(DOCK_KEY) {
        if !params.docked && cmdr.docking_computer != 0 {
            if config.instant_dock != 0 {
                // engage_docking_computer();
            } else {
                // engage_auto_pilot();
            }
        }
    }

    if is_key_down(KeyCode::D) {
        d_pressed();
    }

    if is_key_down(ECM_KEY) {
        if !params.docked && cmdr.ecm != 0 {
            // activate_ecm(1);
        }
    }

    if is_key_down(FIND_KEY) {
        // f_pressed ();
    }

    if is_key_down(HYPERSPACE_KEY) && (!params.docked) {
        if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
            // start_galactic_hyperspace();
        } else {
            // start_hyperspace();
        }
    }

    if is_key_down(JUMP_KEY) && (!params.docked) && (!params.witchspace) {
        // jump_warp();
    }

    if is_key_down(FIRE_MISSILE_KEY) {
        if !params.docked {
            // fire_missile();
        }
    }

    if is_key_down(KeyCode::O) {
        // o_pressed();
    }

    if is_key_down(PAUSE_KEY) {
        params.game_paused = true;
    }

    if is_key_down(TARGET_MISSILE_KEY) {
        if !params.docked {
            // arm_missile();
        }
    }

    if is_key_down(UNARM_MISSILE_KEY) {
        if !params.docked {
            // unarm_missile();
        }
    }

    if is_key_down(INCREASE_SPEED_KEY) {
        if !params.docked {
            if params.flight_speed < params.myship.max_speed {
                params.flight_speed += 1;
            }
        }
    }

    if is_key_down(DECREASE_SPEED_KEY) {
        if !params.docked {
            if params.flight_speed > 1 {
                params.flight_speed -= 1;
            }
        }
    }

    if is_key_down(KeyCode::Up) {
        arrow_up(params);
    }

    if is_key_down(KeyCode::Down) {
        arrow_down(params);
    }

    if is_key_down(KeyCode::Left) {
        arrow_left(params);
    }

    if is_key_down(KeyCode::Right) {
        arrow_right(params);
    }

    if is_key_down(KeyCode::Enter) {
        // return_pressed();
    }

    if is_key_down(ENERGY_BOMB_KEY) {
        if (!params.docked) && (cmdr.energy_bomb != 0) {
            params.detonate_bomb = 1;
            cmdr.energy_bomb = 0;
        }
    }

    if is_key_down(ENERGY_BOMB_KEY) {
        if (!params.docked) && (cmdr.escape_pod != 0) && (!params.witchspace) {
            // run_escape_sequence();
        }
    }
}

fn d_pressed() {
    println!("d_pressed()");
}

fn kbd_read_key() -> KeyCode {
    println!("kbd_read_key()");
    KeyCode::Space
}

fn n_pressed() {
    println!("n_pressed()");
}

fn y_pressed() {
    println!("y_pressed()");
}

fn isalpha(keyasc: KeyCode) -> bool {
    println!("is_alpha()");
    false
}
