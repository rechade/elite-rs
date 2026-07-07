#![allow(warnings)]

// fix the star removal boundaries and centring.
// maybe switch all the model data and whatever needs it to f32.

use macroquad::{
    audio::{self, Sound},
    prelude::*,
};
use std::{thread, time};

use crate::{
    docked::{
        display_commander_status, display_data_on_planet, display_galactic_chart, display_short_range_chart, move_cursor_to_origin, show_distance_to_planet
    },
    elite::{Commander, PlayerShip, SCR_FRONT_VIEW, SCR_REAR_VIEW, *},
    gfx::{GFX_SCALE, GFX_X_CENTRE, GFX_Y_CENTRE},
    pilot::{
        engage_auto_pilot, fly_to_docking_bay, fly_to_planet, fly_to_station, fly_to_station_front,
    },
    planet::{GalaxySeed, PlanetData},
    shipdata::{NO_OF_SHIPS, SHIP_COBRA3, SHIP_CORIOLIS, SHIP_DODEC},
    sound::{SND_BEEP, SND_BLUE_DANUBE, SND_ELITE_THEME},
    space::{
        DaType, UnivObject, countdown_hyperspace, display_hyper_status, dock_player, engage_docking_computer, jump_warp, launch_player, regenerate_shields, start_galactic_hyperspace, start_hyperspace, update_altitude, update_cabin_temp, update_console, update_universe
    },
    stars::{Stars, create_new_stars, flip_stars, update_starfield},
    swat::{
        add_new_ship, arm_missile, clear_universe, cool_laser, draw_laser_lines, fire_laser,
        fire_missile, random_encounter, snd_play_sample, unarm_missile,
    },
    threed::draw_ship,
    vector::{START_MATRIX, START_VECTOR, set_init_matrix, unit_vector, vector_dot_product},
};

const MIN_DIST: [f32; NO_OF_SHIPS + 1] = [
    0.0, 200.0, 800.0, 200.0, 200.0, 200.0, 300.0, 384.0, 200.0, 200.0, 200.0, 420.0, 900.0, 500.0,
    800.0, 384.0, 384.0, 384.0, 384.0, 384.0, 200.0, 384.0, 384.0, 384.0, 0.0, 384.0, 0.0, 384.0,
    384.0, 700.0, 384.0, 0.0, 0.0, 900.0,
];

pub(crate) mod docked;
pub(crate) mod elite;
pub(crate) mod gfx;
pub(crate) mod pilot;
pub(crate) mod planet;
pub(crate) mod shipdata;
pub(crate) mod sound;
pub(crate) mod space;
pub(crate) mod stars;
pub(crate) mod swat;
pub(crate) mod threed;
pub(crate) mod trade;
pub(crate) mod vector;
const DIAL_BAR_MARGIN_PROPORTION: f32 = 0.05;
pub const SCANNER_X_PROPORTION: f32 = 0.2;
const SCANNER_Y_PROPORTION: f32 = 0.25;
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
const PULSE_LASER: My = 0x0F;
const BEAM_LASER: My = 0x8F;
const MILITARY_LASER: My = 0x97;
const MINING_LASER: My = 0x32;
const FLG_DEAD: My = 1;
const FLG_REMOVE: My = 2;
const FLG_EXPLOSION: My = 4;
const FLG_ANGRY: My = 8;
const FLG_FIRING: My = 16;
const FLG_HAS_ECM: My = 32;
const FLG_HOSTILE: My = 64;
const FLG_CLOAKED: My = 128;
const FLG_FLY_TO_PLANET: My = 256;
const FLG_FLY_TO_STATION: My = 512;
const FLG_INACTIVE: My = 1024;
const FLG_SLOW: My = 2048;
const FLG_BOLD: My = 4096;
const FLG_POLICE: My = 8192;

struct Config {
    speed_cap: My,
    wireframe: My,
    anti_alias_gfx: My,
    planet_render_style: My,
    hoopy_casinos: My,
    instant_dock: My,
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
        speed_cap: My,
        wireframe: My,
        anti_alias_gfx: My,
        planet_render_style: My,
        hoopy_casinos: My,
        instant_dock: My,
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

pub type My = i32;
struct GameParams {
    current_screen: My,
    flight_speed: My,
    flight_roll: My,
    flight_climb: My,
    docked: bool,
    front_shield: My,
    aft_shield: My,
    energy: My,
    draw_lasers: My,
    mcount: u8,
    warp_stars: bool,
    message_count: My,
    hyper_ready: bool,
    detonate_bomb: My,
    find_input: bool,
    witchspace: bool,
    game_paused: bool,
    auto_pilot: bool,
    cross_x: My,
    cross_y: My,
    old_cross_x: My,
    old_cross_y: My,
    cross_timer: My,
    myship: PlayerShip,
    message_string: String,
    rolling: bool,
    climbing: bool,
    have_joystick: My,
    finish: bool,
    game_over: bool,
    find_name: [char; 20],
    in_battle: usize,
    docked_planet: GalaxySeed,
    hyperspace_planet: GalaxySeed,
    destination_planet: GalaxySeed,
    dest_planet_string: String,
    current_planet_data: PlanetData,
    curr_galaxy_num: My,
    curr_fuel: My,
    carry_flag: My,
    screen_width: f32,
    screen_height: f32,
    scanner_cx: f32,
    scanner_cy: f32,
    row_y_pos: f32,
    row_inc: f32,
    row_width: f32,
    dial_bar_margin: f32,
    dial_bar_width: f32,
    compass_x: f32,
    compass_y: f32,
    compass_r: f32,
    direction: f32,
    show_time: My,
    ship_no: DaType,
    hyper_distance: My,
    hyper_galactic: bool,
    hyper_countdown: My,
    hyper_name: String,
}
impl GameParams {
    pub fn update_screen_params(&mut self) {
        self.screen_width = screen_width();
        self.screen_height = screen_height();
        self.scanner_cx = self.screen_width * 0.5;
        self.scanner_cy = self.screen_height - self.screen_height * (SCANNER_Y_PROPORTION * 0.5);
        self.row_y_pos = self.screen_height * (1.0 - SCANNER_Y_PROPORTION);
        self.row_inc = self.screen_height * SCANNER_Y_PROPORTION / 7.0;
        self.row_width = self.screen_width * SCANNER_Y_PROPORTION;
        self.dial_bar_margin = self.screen_width * DIAL_BAR_MARGIN_PROPORTION;
        self.dial_bar_width = self.row_width - self.dial_bar_margin;
        self.compass_x = self.screen_width * 0.716;
        self.compass_y = self.screen_height - (self.screen_height * SCANNER_Y_PROPORTION * 0.75);
        self.compass_r = self.screen_width * 0.03;
    }
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
    fn set(
        current_screen: My,
        flight_speed: My,
        flight_roll: My,
        flight_climb: My,
        docked: bool,
        front_shield: My,
        aft_shield: My,
        energy: My,
        draw_lasers: My,
        mcount: u8,
        warp_stars: bool,
        message_count: My,
        hyper_ready: bool,
        detonate_bomb: My,
        find_input: bool,
        witchspace: bool,
        game_paused: bool,
        auto_pilot: bool,
        cross_x: My,
        cross_y: My,
        old_cross_x: My,
        old_cross_y: My,
        cross_timer: My,
        myship: PlayerShip,
        message_string: String,
        rolling: bool,
        climbing: bool,
        have_joystick: My,
        finish: bool,
        game_over: bool,
        find_name: [char; 20],
        in_battle: usize,
        docked_planet: GalaxySeed,
        hyperspace_planet: GalaxySeed,
        current_planet_data: PlanetData,
        dest_planet_string: String,
        curr_galaxy_num: My,
        curr_fuel: My,
        carry_flag: My,
        screen_width: f32,
        screen_height: f32,
        scanner_cx: f32,
        scanner_cy: f32,
        row_y_pos: f32,
        row_inc: f32,
        row_width: f32,
        dial_bar_margin: f32,
        dial_bar_width: f32,
        compass_x: f32,
        compass_y: f32,
        compass_r: f32,
        direction: f32,
        show_time: My,
        ship_no: DaType,
        destination_planet: GalaxySeed,
        hyper_distance: My,
        hyper_galactic: bool,
        hyper_countdown: My,
        hyper_name: String,
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
            warp_stars,
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
            in_battle,
            docked_planet,
            hyperspace_planet,
            current_planet_data,
            dest_planet_string,
            curr_galaxy_num,
            curr_fuel,
            carry_flag,
            screen_width,
            screen_height,
            scanner_cx,
            scanner_cy,
            row_y_pos,
            row_inc,
            row_width,
            dial_bar_margin,
            dial_bar_width,
            compass_x,
            compass_y,
            compass_r,
            direction,
            show_time,
            ship_no,
            destination_planet,
            hyper_distance,
            hyper_galactic,
            hyper_countdown,
            hyper_name,
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
            warp_stars: false,
            message_count: 0,
            hyper_ready: false,
            detonate_bomb: 0,
            find_input: false,
            witchspace: false,
            game_paused: false,
            auto_pilot: true,
            cross_x: 0,
            cross_y: 0,
            old_cross_x: 0,
            old_cross_y: 0,
            cross_timer: 0,
            myship: PlayerShip::new(),
            message_string: "".to_string(),
            rolling: false,
            climbing: false,
            have_joystick: 0,
            finish: false,
            game_over: false,
            find_name: ['b'; 20],
            in_battle: 0,
            docked_planet: GalaxySeed::new(),
            hyperspace_planet: GalaxySeed::new(),
            current_planet_data: PlanetData::new(0, 0, 0, 0, 0, 0),
            dest_planet_string: "".to_string(),
            curr_galaxy_num: 1,
            curr_fuel: 70,
            carry_flag: 0,
            screen_width: screen_width(),
            screen_height: screen_height(),
            scanner_cx: 0.0,
            scanner_cy: 0.0,
            row_y_pos: 0.0,
            row_inc: 0.0,
            row_width: 0.0,
            dial_bar_margin: 0.0,
            dial_bar_width: 0.0,
            compass_x: 0.0,
            compass_y: 0.0,
            compass_r: 0.0,
            ship_no: 0,
            show_time: 0,
            direction: 0.0,
            destination_planet: GalaxySeed::new(),
            hyper_distance: 0,
            hyper_galactic: false,
            hyper_countdown: 0,
            hyper_name: "".to_string(),
        }
    }
}
const NUM_SAMPLES:usize = 16;
#[macroquad::main("EliteRS")]
async fn main() {
    let font = load_ttf_font("./assets/Terminus.ttf").await.unwrap();
    set_pc_assets_folder("assets");
    let beep_sfx = audio::load_sound("beep.wav").await.unwrap();
    let boop_sfx = audio::load_sound("boop.wav").await.unwrap();
    let crash_sfx = audio::load_sound("crash.wav").await.unwrap();
    let danube_sfx = audio::load_sound("danube.wav").await.unwrap();
    let dock_sfx = audio::load_sound("dock.wav").await.unwrap();
    let ecm_sfx = audio::load_sound("ecm.wav").await.unwrap();
    let enemy_sfx = audio::load_sound("hitem.wav").await.unwrap();
    let explode_sfx = audio::load_sound("explode.wav").await.unwrap();
    let gameover_sfx = audio::load_sound("gameover.wav").await.unwrap();
    let hitem_sfx = audio::load_sound("hitem.wav").await.unwrap();
    let hyper_sfx = audio::load_sound("hyper.wav").await.unwrap();
    let incom1_sfx = audio::load_sound("incom1.wav").await.unwrap();
    let incom2_sfx = audio::load_sound("incom2.wav").await.unwrap();
    let launch_sfx = audio::load_sound("launch.wav").await.unwrap();
    let missile_sfx = audio::load_sound("missile.wav").await.unwrap();
    let pulse_sfx = audio::load_sound("pulse.wav").await.unwrap();
    let theme_sfx = audio::load_sound("theme.wav").await.unwrap();
let sample_list:[Sound;NUM_SAMPLES] =
[
	launch_sfx,
	crash_sfx,
	dock_sfx,
	gameover_sfx,
	pulse_sfx,
	hitem_sfx,
	explode_sfx,
	ecm_sfx,
	missile_sfx,
	hyper_sfx,
	incom1_sfx,
	incom2_sfx,
	beep_sfx,
	boop_sfx,
	theme_sfx,
	danube_sfx
];
    let labels: Vec<&str> = vec![
        "FS", "SP", "AS", "RL", "FU", "DC", "CT", " 1", "LT", " 2", "AL", " 3", "MI", " 4",
    ];
    let mut ship_count: [My; NO_OF_SHIPS + 1] = [0; NO_OF_SHIPS + 1]; /* many */

    let missile_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, 0.0, 68.0, 31.0, 1, 0, 3, 2),
        ShipPoint::new(8.0, -8.0, 36.0, 31.0, 2, 1, 5, 4),
        ShipPoint::new(8.0, 8.0, 36.0, 31.0, 3, 2, 7, 4),
        ShipPoint::new(-8.0, 8.0, 36.0, 31.0, 3, 0, 7, 6),
        ShipPoint::new(-8.0, -8.0, 36.0, 31.0, 1, 0, 6, 5),
        ShipPoint::new(8.0, 8.0, -44.0, 31.0, 7, 4, 8, 8),
        ShipPoint::new(8.0, -8.0, -44.0, 31.0, 5, 4, 8, 8),
        ShipPoint::new(-8.0, -8.0, -44.0, 31.0, 6, 5, 8, 8),
        ShipPoint::new(-8.0, 8.0, -44.0, 31.0, 7, 6, 8, 8),
        ShipPoint::new(12.0, 12.0, -44.0, 8.0, 7, 4, 8, 8),
        ShipPoint::new(12.0, -12.0, -44.0, 8.0, 5, 4, 8, 8),
        ShipPoint::new(-12.0, -12.0, -44.0, 8.0, 6, 5, 8, 8),
        ShipPoint::new(-12.0, 12.0, -44.0, 8.0, 7, 6, 8, 8),
        ShipPoint::new(-8.0, 8.0, -12.0, 8.0, 7, 6, 7, 7),
        ShipPoint::new(-8.0, -8.0, -12.0, 8.0, 6, 5, 6, 6),
        ShipPoint::new(8.0, 8.0, -12.0, 8.0, 7, 4, 7, 7),
        ShipPoint::new(8.0, -8.0, -12.0, 8.0, 5, 4, 5, 5),
    ];

    let missile_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 2, 1, 0, 1),
        ShipLine::new(31, 3, 2, 0, 2),
        ShipLine::new(31, 3, 0, 0, 3),
        ShipLine::new(31, 1, 0, 0, 4),
        ShipLine::new(31, 2, 4, 1, 2),
        ShipLine::new(31, 5, 1, 1, 4),
        ShipLine::new(31, 6, 0, 3, 4),
        ShipLine::new(31, 7, 3, 2, 3),
        ShipLine::new(31, 7, 4, 2, 5),
        ShipLine::new(31, 5, 4, 1, 6),
        ShipLine::new(31, 6, 5, 4, 7),
        ShipLine::new(31, 7, 6, 3, 8),
        ShipLine::new(31, 8, 6, 7, 8),
        ShipLine::new(31, 8, 7, 5, 8),
        ShipLine::new(31, 8, 4, 5, 6),
        ShipLine::new(31, 8, 5, 6, 7),
        ShipLine::new(8, 8, 5, 6, 10),
        ShipLine::new(8, 8, 7, 5, 9),
        ShipLine::new(8, 8, 7, 8, 12),
        ShipLine::new(8, 8, 5, 7, 11),
        ShipLine::new(8, 7, 4, 9, 15),
        ShipLine::new(8, 5, 4, 10, 16),
        ShipLine::new(8, 7, 6, 12, 13),
        ShipLine::new(8, 6, 5, 11, 14),
    ];

    let missile_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, -64.0, 0.0, 16.0),
        ShipFaceNormal::new(31.0, 0.0, -64.0, 16.0),
        ShipFaceNormal::new(31.0, 64.0, 0.0, 16.0),
        ShipFaceNormal::new(31.0, 0.0, 64.0, 16.0),
        ShipFaceNormal::new(31.0, 32.0, 0.0, 0.0),
        ShipFaceNormal::new(31.0, 0.0, -32.0, 0.0),
        ShipFaceNormal::new(31.0, -32.0, 0.0, 0.0),
        ShipFaceNormal::new(31.0, 0.0, 32.0, 0.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -176.0),
    ];

    let missile_data: ShipData = ShipData {
        name: put_into_name("Missile"),
        num_points: 17,
        num_lines: 24,
        num_faces: 9,
        max_loot: 0,
        scoop_type: 0,
        size: 1600.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 14,
        energy: 2,
        velocity: 44,
        missiles: 0,
        laser_strength: 0,
        points: missile_point,
        lines: missile_line,
        normals: missile_face_normal,
    };
    let coriolis_point: Vec<ShipPoint> = vec![
        ShipPoint::new(160.0, 0.0, 160.0, 31.0, 1, 0, 6, 2),
        ShipPoint::new(0.0, 160.0, 160.0, 31.0, 2, 0, 8, 3),
        ShipPoint::new(-160.0, 0.0, 160.0, 31.0, 3, 0, 7, 4),
        ShipPoint::new(0.0, -160.0, 160.0, 31.0, 1, 0, 5, 4),
        ShipPoint::new(160.0, -160.0, 0.0, 31.0, 5, 1, 10, 6),
        ShipPoint::new(160.0, 160.0, 0.0, 31.0, 6, 2, 11, 8),
        ShipPoint::new(-160.0, 160.0, 0.0, 31.0, 7, 3, 12, 8),
        ShipPoint::new(-160.0, -160.0, 0.0, 31.0, 5, 4, 9, 7),
        ShipPoint::new(160.0, 0.0, -160.0, 31.0, 10, 6, 13, 11),
        ShipPoint::new(0.0, 160.0, -160.0, 31.0, 11, 8, 13, 12),
        ShipPoint::new(-160.0, 0.0, -160.0, 31.0, 9, 7, 13, 12),
        ShipPoint::new(0.0, -160.0, -160.0, 31.0, 9, 5, 13, 10),
        ShipPoint::new(10.0, -30.0, 160.0, 30.0, 0, 0, 0, 0),
        ShipPoint::new(10.0, 30.0, 160.0, 30.0, 0, 0, 0, 0),
        ShipPoint::new(-10.0, 30.0, 160.0, 30.0, 0, 0, 0, 0),
        ShipPoint::new(-10.0, -30.0, 160.0, 30.0, 0, 0, 0, 0),
    ];

    let coriolis_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 1, 0, 0, 3),
        ShipLine::new(31, 2, 0, 0, 1),
        ShipLine::new(31, 3, 0, 1, 2),
        ShipLine::new(31, 4, 0, 2, 3),
        ShipLine::new(31, 5, 1, 3, 4),
        ShipLine::new(31, 6, 1, 0, 4),
        ShipLine::new(31, 6, 2, 0, 5),
        ShipLine::new(31, 8, 2, 5, 1),
        ShipLine::new(31, 8, 3, 1, 6),
        ShipLine::new(31, 7, 3, 2, 6),
        ShipLine::new(31, 7, 4, 2, 7),
        ShipLine::new(31, 5, 4, 3, 7),
        ShipLine::new(31, 13, 10, 8, 11),
        ShipLine::new(31, 13, 11, 8, 9),
        ShipLine::new(31, 13, 12, 9, 10),
        ShipLine::new(31, 13, 9, 10, 11),
        ShipLine::new(31, 10, 5, 4, 11),
        ShipLine::new(31, 10, 6, 4, 8),
        ShipLine::new(31, 11, 6, 5, 8),
        ShipLine::new(31, 11, 8, 5, 9),
        ShipLine::new(31, 12, 8, 6, 9),
        ShipLine::new(31, 12, 7, 6, 10),
        ShipLine::new(31, 9, 7, 7, 10),
        ShipLine::new(31, 9, 5, 7, 11),
        ShipLine::new(30, 0, 0, 12, 13),
        ShipLine::new(30, 0, 0, 13, 14),
        ShipLine::new(30, 0, 0, 14, 15),
        ShipLine::new(30, 0, 0, 15, 12),
    ];

    let coriolis_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 0.0, 0.0, 160.0),
        ShipFaceNormal::new(31.0, 107.0, -107.0, 107.0),
        ShipFaceNormal::new(31.0, 107.0, 107.0, 107.0),
        ShipFaceNormal::new(31.0, -107.0, 107.0, 107.0),
        ShipFaceNormal::new(31.0, -107.0, -107.0, 107.0),
        ShipFaceNormal::new(31.0, 0.0, -160.0, 0.0),
        ShipFaceNormal::new(31.0, 160.0, 0.0, 0.0),
        ShipFaceNormal::new(31.0, -160.0, 0.0, 0.0),
        ShipFaceNormal::new(31.0, 0.0, 160.0, 0.0),
        ShipFaceNormal::new(31.0, -107.0, -107.0, -107.0),
        ShipFaceNormal::new(31.0, 107.0, -107.0, -107.0),
        ShipFaceNormal::new(31.0, 107.0, 107.0, -107.0),
        ShipFaceNormal::new(31.0, -107.0, 107.0, -107.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -160.0),
    ];

    let coriolis_data: ShipData = ShipData {
        name: put_into_name("Coriolis Space Station"),
        num_points: 16,
        num_lines: 28,
        num_faces: 14,
        max_loot: 0,
        scoop_type: 0,
        size: 25600.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 120,
        energy: 240,
        velocity: 0,
        missiles: 6,
        laser_strength: 3,
        points: coriolis_point,
        lines: coriolis_line,
        normals: coriolis_face_normal,
    };
    let esccaps_point: Vec<ShipPoint> = vec![
        ShipPoint::new(-7.0, 0.0, 36.0, 31.0, 1, 2, 3, 3),
        ShipPoint::new(-7.0, -14.0, -12.0, 31.0, 0, 2, 3, 3),
        ShipPoint::new(-7.0, 14.0, -12.0, 31.0, 0, 1, 3, 3),
        ShipPoint::new(21.0, 0.0, 0.0, 31.0, 0, 1, 2, 2),
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
        ShipFaceNormal::new(31.0, 52.0, 0.0, -122.0),
        ShipFaceNormal::new(31.0, 39.0, 103.0, 30.0),
        ShipFaceNormal::new(31.0, 39.0, -103.0, 30.0),
        ShipFaceNormal::new(31.0, -112.0, 0.0, 0.0),
    ];

    let esccaps_data: ShipData = ShipData {
        name: put_into_name("Escape Capsule"),
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
    let alloy_point: Vec<ShipPoint> = vec![
        ShipPoint::new(-15.0, -22.0, -9.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-15.0, 38.0, -9.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(19.0, 32.0, 11.0, 20.0, 15, 15, 15, 15),
        ShipPoint::new(10.0, -46.0, 6.0, 20.0, 15, 15, 15, 15),
    ];

    let alloy_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 15, 15, 0, 1),
        ShipLine::new(16, 15, 15, 1, 2),
        ShipLine::new(20, 15, 15, 2, 3),
        ShipLine::new(16, 15, 15, 3, 0),
    ];

    let alloy_face_normal: Vec<ShipFaceNormal> = vec![ShipFaceNormal::new(0.0, 0.0, 0.0, 0.0)];

    let alloy_data: ShipData = ShipData {
        name: put_into_name("Alloy"),
        num_points: 4,
        num_lines: 4,
        num_faces: 1,
        max_loot: 0,
        scoop_type: 8,
        size: 100.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 5,
        energy: 16,
        velocity: 16,
        missiles: 0,
        laser_strength: 0,
        points: alloy_point,
        lines: alloy_line,
        normals: alloy_face_normal,
    };

    let cargo_point: Vec<ShipPoint> = vec![
        ShipPoint::new(24.0, 16.0, 0.0, 31.0, 1, 0, 5, 5),
        ShipPoint::new(24.0, 5.0, 15.0, 31.0, 1, 0, 2, 2),
        ShipPoint::new(24.0, -13.0, 9.0, 31.0, 2, 0, 3, 3),
        ShipPoint::new(24.0, -13.0, -9.0, 31.0, 3, 0, 4, 4),
        ShipPoint::new(24.0, 5.0, -15.0, 31.0, 4, 0, 5, 5),
        ShipPoint::new(-24.0, 16.0, 0.0, 31.0, 5, 1, 6, 6),
        ShipPoint::new(-24.0, 5.0, 15.0, 31.0, 2, 1, 6, 6),
        ShipPoint::new(-24.0, -13.0, 9.0, 31.0, 3, 2, 6, 6),
        ShipPoint::new(-24.0, -13.0, -9.0, 31.0, 4, 3, 6, 6),
        ShipPoint::new(-24.0, 5.0, -15.0, 31.0, 5, 4, 6, 6),
    ];

    let cargo_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 1, 0, 0, 1),
        ShipLine::new(31, 2, 0, 1, 2),
        ShipLine::new(31, 3, 0, 2, 3),
        ShipLine::new(31, 4, 0, 3, 4),
        ShipLine::new(31, 5, 0, 0, 4),
        ShipLine::new(31, 5, 1, 0, 5),
        ShipLine::new(31, 2, 1, 1, 6),
        ShipLine::new(31, 3, 2, 2, 7),
        ShipLine::new(31, 4, 3, 3, 8),
        ShipLine::new(31, 5, 4, 4, 9),
        ShipLine::new(31, 6, 1, 5, 6),
        ShipLine::new(31, 6, 2, 6, 7),
        ShipLine::new(31, 6, 3, 7, 8),
        ShipLine::new(31, 6, 4, 8, 9),
        ShipLine::new(31, 6, 5, 9, 5),
    ];

    let cargo_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 96.0, 0.0, 0.0),
        ShipFaceNormal::new(31.0, 0.0, 41.0, 30.0),
        ShipFaceNormal::new(31.0, 0.0, -18.0, 48.0),
        ShipFaceNormal::new(31.0, 0.0, -51.0, 0.0),
        ShipFaceNormal::new(31.0, 0.0, -18.0, -48.0),
        ShipFaceNormal::new(31.0, 0.0, 41.0, -30.0),
        ShipFaceNormal::new(31.0, -96.0, 0.0, 0.0),
    ];

    let cargo_data: ShipData = ShipData {
        name: put_into_name("Cargo Canister"),
        num_points: 10,
        num_lines: 15,
        num_faces: 7,
        max_loot: 0,
        scoop_type: 0,
        size: 400.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 12,
        energy: 17,
        velocity: 15,
        missiles: 0,
        laser_strength: 0,
        points: cargo_point,
        lines: cargo_line,
        normals: cargo_face_normal,
    };

    let boulder_point: Vec<ShipPoint> = vec![
        ShipPoint::new(-18.0, 37.0, -11.0, 31.0, 0, 1, 5, 9),
        ShipPoint::new(30.0, 7.0, 12.0, 31.0, 1, 2, 5, 6),
        ShipPoint::new(28.0, -7.0, -12.0, 31.0, 2, 3, 6, 7),
        ShipPoint::new(2.0, 0.0, -39.0, 31.0, 3, 4, 7, 8),
        ShipPoint::new(-28.0, 34.0, -30.0, 31.0, 0, 4, 8, 9),
        ShipPoint::new(5.0, -10.0, 13.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(20.0, 17.0, -30.0, 31.0, 15, 15, 15, 15),
    ];

    let boulder_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 1, 5, 0, 1),
        ShipLine::new(31, 2, 6, 1, 2),
        ShipLine::new(31, 3, 7, 2, 3),
        ShipLine::new(31, 4, 8, 3, 4),
        ShipLine::new(31, 0, 9, 4, 0),
        ShipLine::new(31, 0, 1, 0, 5),
        ShipLine::new(31, 1, 2, 1, 5),
        ShipLine::new(31, 2, 3, 2, 5),
        ShipLine::new(31, 3, 4, 3, 5),
        ShipLine::new(31, 0, 4, 4, 5),
        ShipLine::new(31, 5, 9, 0, 6),
        ShipLine::new(31, 5, 6, 1, 6),
        ShipLine::new(31, 6, 7, 2, 6),
        ShipLine::new(31, 7, 8, 3, 6),
        ShipLine::new(31, 8, 9, 4, 6),
    ];

    let boulder_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, -15.0, -3.0, 8.0),
        ShipFaceNormal::new(31.0, -7.0, 12.0, 30.0),
        ShipFaceNormal::new(31.0, 32.0, -47.0, 24.0),
        ShipFaceNormal::new(31.0, -3.0, -39.0, -7.0),
        ShipFaceNormal::new(31.0, -5.0, -4.0, -1.0),
        ShipFaceNormal::new(31.0, 49.0, 84.0, 8.0),
        ShipFaceNormal::new(31.0, 112.0, 21.0, -21.0),
        ShipFaceNormal::new(31.0, 76.0, -35.0, -82.0),
        ShipFaceNormal::new(31.0, 22.0, 56.0, -137.0),
        ShipFaceNormal::new(31.0, 40.0, 110.0, -38.0),
    ];

    let boulder_data: ShipData = ShipData {
        name: put_into_name("Boulder"),
        num_points: 7,
        num_lines: 15,
        num_faces: 10,
        max_loot: 0,
        scoop_type: 0,
        size: 900.0,
        front_laser: 0,
        bounty: 1,
        vanish_point: 20,
        energy: 20,
        velocity: 30,
        missiles: 0,
        laser_strength: 0,
        points: boulder_point,
        lines: boulder_line,
        normals: boulder_face_normal,
    };

    let asteroid_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, 80.0, 0.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-80.0, -10.0, 0.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(0.0, -80.0, 0.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(70.0, -40.0, 0.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(60.0, 50.0, 0.0, 31.0, 6, 5, 13, 12),
        ShipPoint::new(50.0, 0.0, 60.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-40.0, 0.0, 70.0, 31.0, 1, 0, 3, 2),
        ShipPoint::new(0.0, 30.0, -75.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(0.0, -50.0, -60.0, 31.0, 9, 8, 11, 10),
    ];

    let asteroid_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 7, 2, 0, 1),
        ShipLine::new(31, 13, 6, 0, 4),
        ShipLine::new(31, 12, 5, 3, 4),
        ShipLine::new(31, 11, 4, 2, 3),
        ShipLine::new(31, 10, 3, 1, 2),
        ShipLine::new(31, 3, 2, 1, 6),
        ShipLine::new(31, 3, 1, 2, 6),
        ShipLine::new(31, 4, 1, 2, 5),
        ShipLine::new(31, 1, 0, 5, 6),
        ShipLine::new(31, 6, 0, 0, 5),
        ShipLine::new(31, 5, 4, 3, 5),
        ShipLine::new(31, 2, 0, 0, 6),
        ShipLine::new(31, 6, 5, 4, 5),
        ShipLine::new(31, 10, 8, 1, 8),
        ShipLine::new(31, 8, 7, 1, 7),
        ShipLine::new(31, 13, 7, 0, 7),
        ShipLine::new(31, 13, 12, 4, 7),
        ShipLine::new(31, 12, 9, 3, 7),
        ShipLine::new(31, 11, 9, 3, 8),
        ShipLine::new(31, 11, 10, 2, 8),
        ShipLine::new(31, 9, 8, 7, 8),
    ];

    let asteroid_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 9.0, 66.0, 81.0),
        ShipFaceNormal::new(31.0, 9.0, -66.0, 81.0),
        ShipFaceNormal::new(31.0, -72.0, 64.0, 31.0),
        ShipFaceNormal::new(31.0, -64.0, -73.0, 47.0),
        ShipFaceNormal::new(31.0, 45.0, -79.0, 65.0),
        ShipFaceNormal::new(31.0, 135.0, 15.0, 35.0),
        ShipFaceNormal::new(31.0, 38.0, 76.0, 70.0),
        ShipFaceNormal::new(31.0, -66.0, 59.0, -39.0),
        ShipFaceNormal::new(31.0, -67.0, -15.0, -80.0),
        ShipFaceNormal::new(31.0, 66.0, -14.0, -75.0),
        ShipFaceNormal::new(31.0, -70.0, -80.0, -40.0),
        ShipFaceNormal::new(31.0, 58.0, -102.0, -51.0),
        ShipFaceNormal::new(31.0, 81.0, 9.0, -67.0),
        ShipFaceNormal::new(31.0, 47.0, 94.0, -63.0),
    ];

    let asteroid_data: ShipData = ShipData {
        name: put_into_name("Asteroid"),
        num_points: 9,
        num_lines: 21,
        num_faces: 14,
        max_loot: 0,
        scoop_type: 0,
        size: 6400.0,
        front_laser: 0,
        bounty: 5,
        vanish_point: 50,
        energy: 60,
        velocity: 30,
        missiles: 0,
        laser_strength: 0,
        points: asteroid_point,
        lines: asteroid_line,
        normals: asteroid_face_normal,
    };

    let rock_point: Vec<ShipPoint> = vec![
        ShipPoint::new(-24.0, -25.0, 16.0, 31.0, 1, 2, 3, 3),
        ShipPoint::new(0.0, 12.0, -10.0, 31.0, 0, 2, 3, 3),
        ShipPoint::new(11.0, -6.0, 2.0, 31.0, 0, 1, 3, 3),
        ShipPoint::new(12.0, 42.0, 7.0, 31.0, 0, 1, 2, 2),
    ];

    let rock_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 2, 3, 0, 1),
        ShipLine::new(31, 0, 3, 1, 2),
        ShipLine::new(31, 0, 1, 2, 3),
        ShipLine::new(31, 1, 2, 3, 0),
        ShipLine::new(31, 1, 3, 0, 2),
        ShipLine::new(31, 0, 2, 3, 1),
    ];

    let rock_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(18.0, 30.0, 0.0, 0.0),
        ShipFaceNormal::new(20.0, 22.0, 32.0, -8.0),
        ShipFaceNormal::new(0.0, 0.0, 2.0, 0.0),
        ShipFaceNormal::new(0.0, 17.0, 23.0, 95.0),
    ];

    let rock_data: ShipData = ShipData {
        name: put_into_name("Rock"),
        num_points: 4,
        num_lines: 6,
        num_faces: 4,
        max_loot: 0,
        scoop_type: 11,
        size: 256.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 8,
        energy: 20,
        velocity: 10,
        missiles: 0,
        laser_strength: 0,
        points: rock_point,
        lines: rock_line,
        normals: rock_face_normal,
    };

    let orbit_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, -17.0, 23.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-17.0, 0.0, 23.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(0.0, 18.0, 23.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(18.0, 0.0, 23.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-20.0, -20.0, -27.0, 31.0, 1, 2, 3, 9),
        ShipPoint::new(-20.0, 20.0, -27.0, 31.0, 3, 4, 5, 9),
        ShipPoint::new(20.0, 20.0, -27.0, 31.0, 5, 6, 7, 9),
        ShipPoint::new(20.0, -20.0, -27.0, 31.0, 1, 7, 8, 9),
        ShipPoint::new(5.0, 0.0, -27.0, 16.0, 9, 9, 9, 9),
        ShipPoint::new(0.0, -2.0, -27.0, 16.0, 9, 9, 9, 9),
        ShipPoint::new(-5.0, 0.0, -27.0, 9.0, 9, 9, 9, 9),
        ShipPoint::new(0.0, 3.0, -27.0, 9.0, 9, 9, 9, 9),
        ShipPoint::new(0.0, -9.0, 35.0, 16.0, 0, 10, 11, 12),
        ShipPoint::new(3.0, -1.0, 31.0, 7.0, 15, 15, 0, 2),
        ShipPoint::new(4.0, 11.0, 25.0, 8.0, 0, 1, 15, 4),
        ShipPoint::new(11.0, 4.0, 25.0, 8.0, 10, 1, 3, 15),
        ShipPoint::new(-3.0, -1.0, 31.0, 7.0, 6, 11, 2, 3),
        ShipPoint::new(-3.0, 11.0, 25.0, 8.0, 15, 8, 12, 0),
        ShipPoint::new(-10.0, 4.0, 25.0, 8.0, 4, 15, 1, 8),
    ];

    let orbit_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 0, 2, 0, 1),
        ShipLine::new(31, 4, 10, 1, 2),
        ShipLine::new(31, 6, 11, 2, 3),
        ShipLine::new(31, 8, 12, 0, 3),
        ShipLine::new(31, 1, 8, 0, 7),
        ShipLine::new(24, 1, 2, 0, 4),
        ShipLine::new(31, 2, 3, 1, 4),
        ShipLine::new(24, 3, 4, 1, 5),
        ShipLine::new(31, 4, 5, 2, 5),
        ShipLine::new(12, 5, 6, 2, 6),
        ShipLine::new(31, 6, 7, 3, 6),
        ShipLine::new(24, 7, 8, 3, 7),
        ShipLine::new(31, 3, 9, 4, 5),
        ShipLine::new(31, 5, 9, 5, 6),
        ShipLine::new(31, 7, 9, 6, 7),
        ShipLine::new(31, 1, 9, 4, 7),
        ShipLine::new(16, 0, 12, 0, 12),
        ShipLine::new(16, 0, 10, 1, 12),
        ShipLine::new(16, 10, 11, 2, 12),
        ShipLine::new(16, 11, 12, 3, 12),
        ShipLine::new(16, 9, 9, 8, 9),
        ShipLine::new(7, 9, 9, 9, 10),
        ShipLine::new(9, 9, 9, 10, 11),
        ShipLine::new(7, 9, 9, 8, 11),
        ShipLine::new(5, 11, 11, 13, 14),
        ShipLine::new(8, 11, 11, 14, 15),
        ShipLine::new(7, 11, 11, 13, 15),
        ShipLine::new(5, 10, 10, 16, 17),
        ShipLine::new(8, 10, 10, 17, 18),
        ShipLine::new(7, 10, 10, 16, 18),
    ];

    let orbit_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, -55.0, -55.0, 40.0),
        ShipFaceNormal::new(31.0, 0.0, -74.0, 4.0),
        ShipFaceNormal::new(31.0, -51.0, -51.0, 23.0),
        ShipFaceNormal::new(31.0, -74.0, 0.0, 4.0),
        ShipFaceNormal::new(31.0, -51.0, 51.0, 23.0),
        ShipFaceNormal::new(31.0, 0.0, 74.0, 4.0),
        ShipFaceNormal::new(31.0, 51.0, 51.0, 23.0),
        ShipFaceNormal::new(31.0, 74.0, 0.0, 4.0),
        ShipFaceNormal::new(31.0, 51.0, -51.0, 23.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -107.0),
        ShipFaceNormal::new(31.0, -41.0, 41.0, 90.0),
        ShipFaceNormal::new(31.0, 41.0, 41.0, 90.0),
        ShipFaceNormal::new(31.0, 55.0, -55.0, 40.0),
    ];

    let orbit_data: ShipData = ShipData {
        name: put_into_name("Orbit Shuttle"),
        num_points: 19,
        num_lines: 30,
        num_faces: 13,
        max_loot: 15,
        scoop_type: 0,
        size: 2500.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 22,
        energy: 32,
        velocity: 8,
        missiles: 0,
        laser_strength: 0,
        points: orbit_point,
        lines: orbit_line,
        normals: orbit_face_normal,
    };
    let transp_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, 10.0, -26.0, 31.0, 0, 6, 7, 7),
        ShipPoint::new(-25.0, 4.0, -26.0, 31.0, 0, 1, 7, 7),
        ShipPoint::new(-28.0, -3.0, -26.0, 31.0, 0, 1, 2, 2),
        ShipPoint::new(-25.0, -8.0, -26.0, 31.0, 0, 2, 3, 3),
        ShipPoint::new(26.0, -8.0, -26.0, 31.0, 0, 3, 4, 4),
        ShipPoint::new(29.0, -3.0, -26.0, 31.0, 0, 4, 5, 5),
        ShipPoint::new(26.0, 4.0, -26.0, 31.0, 0, 5, 6, 6),
        ShipPoint::new(0.0, 6.0, 12.0, 19.0, 15, 15, 15, 15),
        ShipPoint::new(-30.0, -1.0, 12.0, 31.0, 1, 7, 8, 9),
        ShipPoint::new(-33.0, -8.0, 12.0, 31.0, 1, 2, 3, 9),
        ShipPoint::new(33.0, -8.0, 12.0, 31.0, 3, 4, 5, 10),
        ShipPoint::new(30.0, -1.0, 12.0, 31.0, 5, 6, 10, 11),
        ShipPoint::new(-11.0, -2.0, 30.0, 31.0, 8, 9, 12, 13),
        ShipPoint::new(-13.0, -8.0, 30.0, 31.0, 3, 9, 13, 13),
        ShipPoint::new(14.0, -8.0, 30.0, 31.0, 3, 10, 13, 13),
        ShipPoint::new(11.0, -2.0, 30.0, 31.0, 10, 11, 12, 13),
        ShipPoint::new(-5.0, 6.0, 2.0, 7.0, 7, 7, 7, 7),
        ShipPoint::new(-18.0, 3.0, 2.0, 7.0, 7, 7, 7, 7),
        ShipPoint::new(-5.0, 7.0, -7.0, 7.0, 7, 7, 7, 7),
        ShipPoint::new(-18.0, 4.0, -7.0, 7.0, 7, 7, 7, 7),
        ShipPoint::new(-11.0, 6.0, -14.0, 7.0, 7, 7, 7, 7),
        ShipPoint::new(-11.0, 5.0, -7.0, 7.0, 7, 7, 7, 7),
        ShipPoint::new(5.0, 7.0, -14.0, 7.0, 6, 6, 6, 6),
        ShipPoint::new(18.0, 4.0, -14.0, 7.0, 6, 6, 6, 6),
        ShipPoint::new(11.0, 5.0, -7.0, 7.0, 6, 6, 6, 6),
        ShipPoint::new(5.0, 6.0, -3.0, 7.0, 6, 6, 6, 6),
        ShipPoint::new(18.0, 3.0, -3.0, 7.0, 6, 6, 6, 6),
        ShipPoint::new(11.0, 4.0, 8.0, 7.0, 6, 6, 6, 6),
        ShipPoint::new(11.0, 5.0, -3.0, 7.0, 6, 6, 6, 6),
        ShipPoint::new(-16.0, -8.0, -13.0, 6.0, 3, 3, 3, 3),
        ShipPoint::new(-16.0, -8.0, 16.0, 6.0, 3, 3, 3, 3),
        ShipPoint::new(17.0, -8.0, -13.0, 6.0, 3, 3, 3, 3),
        ShipPoint::new(17.0, -8.0, 16.0, 6.0, 3, 3, 3, 3),
        ShipPoint::new(-13.0, -3.0, -26.0, 8.0, 0, 0, 0, 0),
        ShipPoint::new(13.0, -3.0, -26.0, 8.0, 0, 0, 0, 0),
        ShipPoint::new(9.0, 3.0, -26.0, 5.0, 0, 0, 0, 0),
        ShipPoint::new(-8.0, 3.0, -26.0, 5.0, 0, 0, 0, 0),
    ];

    let transp_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 0, 7, 0, 1),
        ShipLine::new(31, 0, 1, 1, 2),
        ShipLine::new(31, 0, 2, 2, 3),
        ShipLine::new(31, 0, 3, 3, 4),
        ShipLine::new(31, 0, 4, 4, 5),
        ShipLine::new(31, 0, 5, 5, 6),
        ShipLine::new(31, 0, 6, 0, 6),
        ShipLine::new(16, 6, 7, 0, 7),
        ShipLine::new(31, 1, 7, 1, 8),
        ShipLine::new(11, 1, 2, 2, 9),
        ShipLine::new(31, 2, 3, 3, 9),
        ShipLine::new(31, 3, 4, 4, 10),
        ShipLine::new(11, 4, 5, 5, 10),
        ShipLine::new(31, 5, 6, 6, 11),
        ShipLine::new(17, 7, 8, 7, 8),
        ShipLine::new(17, 1, 9, 8, 9),
        ShipLine::new(17, 5, 10, 10, 11),
        ShipLine::new(17, 6, 11, 7, 11),
        ShipLine::new(19, 11, 12, 7, 15),
        ShipLine::new(19, 8, 12, 7, 12),
        ShipLine::new(16, 8, 9, 8, 12),
        ShipLine::new(31, 3, 9, 9, 13),
        ShipLine::new(31, 3, 10, 10, 14),
        ShipLine::new(16, 10, 11, 11, 15),
        ShipLine::new(31, 9, 13, 12, 13),
        ShipLine::new(31, 3, 13, 13, 14),
        ShipLine::new(31, 10, 13, 14, 15),
        ShipLine::new(31, 12, 13, 12, 15),
        ShipLine::new(7, 7, 7, 16, 17),
        ShipLine::new(7, 7, 7, 18, 19),
        ShipLine::new(7, 7, 7, 19, 20),
        ShipLine::new(7, 7, 7, 18, 20),
        ShipLine::new(7, 7, 7, 20, 21),
        ShipLine::new(7, 6, 6, 22, 23),
        ShipLine::new(7, 6, 6, 23, 24),
        ShipLine::new(7, 6, 6, 24, 22),
        ShipLine::new(7, 6, 6, 25, 26),
        ShipLine::new(7, 6, 6, 26, 27),
        ShipLine::new(7, 6, 6, 25, 27),
        ShipLine::new(7, 6, 6, 27, 28),
        ShipLine::new(6, 3, 3, 29, 30),
        ShipLine::new(6, 3, 3, 31, 32),
        ShipLine::new(8, 0, 0, 33, 34),
        ShipLine::new(5, 0, 0, 34, 35),
        ShipLine::new(5, 0, 0, 35, 36),
        ShipLine::new(5, 0, 0, 36, 33),
    ];

    let transp_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(41.0, 0.0, 0.0, -103.0),
        ShipFaceNormal::new(41.0, -111.0, 48.0, -7.0),
        ShipFaceNormal::new(41.0, -105.0, -63.0, -21.0),
        ShipFaceNormal::new(41.0, 0.0, -34.0, 0.0),
        ShipFaceNormal::new(41.0, 105.0, -63.0, -21.0),
        ShipFaceNormal::new(41.0, 111.0, 48.0, -7.0),
        ShipFaceNormal::new(41.0, 8.0, 32.0, 3.0),
        ShipFaceNormal::new(41.0, -8.0, 32.0, 3.0),
        ShipFaceNormal::new(29.0, -8.0, 34.0, 11.0),
        ShipFaceNormal::new(41.0, -75.0, 32.0, 79.0),
        ShipFaceNormal::new(41.0, 75.0, 32.0, 79.0),
        ShipFaceNormal::new(29.0, 8.0, 34.0, 11.0),
        ShipFaceNormal::new(41.0, 0.0, 38.0, 17.0),
        ShipFaceNormal::new(41.0, 0.0, 0.0, 121.0),
    ];

    let transp_data: ShipData = ShipData {
        name: put_into_name("Transporter"),
        num_points: 37,
        num_lines: 46,
        num_faces: 14,
        max_loot: 0,
        scoop_type: 0,
        size: 2500.0,
        front_laser: 12,
        bounty: 0,
        vanish_point: 16,
        energy: 32,
        velocity: 10,
        missiles: 0,
        laser_strength: 0,
        points: transp_point,
        lines: transp_line,
        normals: transp_face_normal,
    };

    let cobra3a_point: Vec<ShipPoint> = vec![
        ShipPoint::new(32.0, 0.0, 76.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-32.0, 0.0, 76.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(0.0, 26.0, 24.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-120.0, -3.0, -8.0, 31.0, 3, 7, 10, 10),
        ShipPoint::new(120.0, -3.0, -8.0, 31.0, 4, 8, 12, 12),
        ShipPoint::new(-88.0, 16.0, -40.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(88.0, 16.0, -40.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(128.0, -8.0, -40.0, 31.0, 8, 9, 12, 12),
        ShipPoint::new(128.0, -8.0, -40.0, 31.0, 7, 9, 10, 10),
        ShipPoint::new(0.0, 26.0, -40.0, 31.0, 5, 6, 9, 9),
        ShipPoint::new(-32.0, -24.0, -40.0, 31.0, 9, 10, 11, 11),
        ShipPoint::new(32.0, -24.0, -40.0, 31.0, 9, 11, 12, 12),
        ShipPoint::new(-36.0, 8.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(-8.0, 12.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(8.0, 12.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(36.0, 8.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(36.0, -12.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(8.0, -16.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(-8.0, -16.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(-36.0, -12.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(0.0, 0.0, 76.0, 6.0, 0, 11, 11, 11),
        ShipPoint::new(0.0, 0.0, 90.0, 31.0, 0, 11, 11, 11),
        ShipPoint::new(-80.0, -6.0, -40.0, 8.0, 9, 9, 9, 9),
        ShipPoint::new(-80.0, 6.0, -40.0, 8.0, 9, 9, 9, 9),
        ShipPoint::new(-88.0, 0.0, -40.0, 6.0, 9, 9, 9, 9),
        ShipPoint::new(80.0, 6.0, -40.0, 8.0, 9, 9, 9, 9),
        ShipPoint::new(88.0, 0.0, -40.0, 6.0, 9, 9, 9, 9),
        ShipPoint::new(80.0, -6.0, -40.0, 8.0, 9, 9, 9, 9),
    ];

    let cobra3a_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 0, 11, 0, 1),
        ShipLine::new(31, 4, 12, 0, 4),
        ShipLine::new(31, 3, 10, 1, 3),
        ShipLine::new(31, 7, 10, 3, 8),
        ShipLine::new(31, 8, 12, 4, 7),
        ShipLine::new(31, 8, 9, 6, 7),
        ShipLine::new(31, 6, 9, 6, 9),
        ShipLine::new(31, 5, 9, 5, 9),
        ShipLine::new(31, 7, 9, 5, 8),
        ShipLine::new(31, 1, 5, 2, 5),
        ShipLine::new(31, 2, 6, 2, 6),
        ShipLine::new(31, 3, 7, 3, 5),
        ShipLine::new(31, 4, 8, 4, 6),
        ShipLine::new(31, 0, 1, 1, 2),
        ShipLine::new(31, 0, 2, 0, 2),
        ShipLine::new(31, 9, 10, 8, 10),
        ShipLine::new(31, 9, 11, 10, 11),
        ShipLine::new(31, 9, 12, 7, 11),
        ShipLine::new(31, 10, 11, 1, 10),
        ShipLine::new(31, 11, 12, 0, 11),
        ShipLine::new(29, 1, 3, 1, 5),
        ShipLine::new(29, 2, 4, 0, 6),
        ShipLine::new(6, 0, 11, 20, 21),
        ShipLine::new(20, 9, 9, 12, 13),
        ShipLine::new(20, 9, 9, 18, 19),
        ShipLine::new(20, 9, 9, 14, 15),
        ShipLine::new(20, 9, 9, 16, 17),
        ShipLine::new(19, 9, 9, 15, 16),
        ShipLine::new(17, 9, 9, 14, 17),
        ShipLine::new(19, 9, 9, 13, 18),
        ShipLine::new(19, 9, 9, 12, 19),
        ShipLine::new(30, 5, 6, 2, 9),
        ShipLine::new(6, 9, 9, 22, 24),
        ShipLine::new(6, 9, 9, 23, 24),
        ShipLine::new(8, 9, 9, 22, 23),
        ShipLine::new(6, 9, 9, 25, 26),
        ShipLine::new(6, 9, 9, 26, 27),
        ShipLine::new(8, 9, 9, 25, 27),
    ];

    let cobra3a_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 0.0, 62.0, 31.0),
        ShipFaceNormal::new(31.0, -18.0, 55.0, 16.0),
        ShipFaceNormal::new(31.0, 18.0, 55.0, 16.0),
        ShipFaceNormal::new(31.0, -16.0, 52.0, 14.0),
        ShipFaceNormal::new(31.0, 16.0, 52.0, 14.0),
        ShipFaceNormal::new(31.0, -14.0, 47.0, 0.0),
        ShipFaceNormal::new(31.0, 14.0, 47.0, 0.0),
        ShipFaceNormal::new(31.0, -61.0, 102.0, 0.0),
        ShipFaceNormal::new(31.0, 61.0, 102.0, 0.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -80.0),
        ShipFaceNormal::new(31.0, -7.0, -42.0, 9.0),
        ShipFaceNormal::new(31.0, 0.0, -30.0, 6.0),
        ShipFaceNormal::new(31.0, 7.0, -42.0, 9.0),
    ];

    let cobra3a_data: ShipData = ShipData {
        name: put_into_name("Cobra MkIIIa"),
        num_points: 28,
        num_lines: 38,
        num_faces: 13,
        max_loot: 3,
        scoop_type: 0,
        size: 9025.0,
        front_laser: 21,
        bounty: 0,
        vanish_point: 50,
        energy: 150,
        velocity: 28,
        missiles: 3,
        laser_strength: 9,
        points: cobra3a_point,
        lines: cobra3a_line,
        normals: cobra3a_face_normal,
    };

    let pythona_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, 0.0, 224.0, 31.0, 1, 0, 3, 2),
        ShipPoint::new(0.0, 48.0, 48.0, 31.0, 1, 0, 5, 4),
        ShipPoint::new(96.0, 0.0, -16.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-96.0, 0.0, -16.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(0.0, 48.0, -32.0, 31.0, 5, 4, 9, 8),
        ShipPoint::new(0.0, 24.0, -112.0, 31.0, 8, 9, 12, 12),
        ShipPoint::new(-48.0, 0.0, -112.0, 31.0, 11, 8, 12, 12),
        ShipPoint::new(48.0, 0.0, -112.0, 31.0, 10, 9, 12, 12),
        ShipPoint::new(0.0, -48.0, 48.0, 31.0, 3, 2, 7, 6),
        ShipPoint::new(0.0, -48.0, -32.0, 31.0, 7, 6, 11, 10),
        ShipPoint::new(0.0, -24.0, -112.0, 31.0, 11, 10, 12, 12),
    ];

    let pythona_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 3, 2, 0, 8),
        ShipLine::new(31, 2, 0, 0, 3),
        ShipLine::new(31, 3, 1, 0, 2),
        ShipLine::new(31, 1, 0, 0, 1),
        ShipLine::new(31, 5, 9, 2, 4),
        ShipLine::new(31, 5, 1, 1, 2),
        ShipLine::new(31, 3, 7, 2, 8),
        ShipLine::new(31, 4, 0, 1, 3),
        ShipLine::new(31, 6, 2, 3, 8),
        ShipLine::new(31, 10, 7, 2, 9),
        ShipLine::new(31, 8, 4, 3, 4),
        ShipLine::new(31, 11, 6, 3, 9),
        ShipLine::new(7, 8, 8, 3, 5),
        ShipLine::new(7, 11, 11, 3, 10),
        ShipLine::new(7, 9, 9, 2, 5),
        ShipLine::new(7, 10, 10, 2, 10),
        ShipLine::new(31, 10, 9, 2, 7),
        ShipLine::new(31, 11, 8, 3, 6),
        ShipLine::new(31, 12, 8, 5, 6),
        ShipLine::new(31, 12, 9, 5, 7),
        ShipLine::new(31, 10, 12, 7, 10),
        ShipLine::new(31, 12, 11, 6, 10),
        ShipLine::new(31, 9, 8, 4, 5),
        ShipLine::new(31, 11, 10, 9, 10),
        ShipLine::new(31, 5, 4, 1, 4),
        ShipLine::new(31, 7, 6, 8, 9),
    ];

    let pythona_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, -27.0, 40.0, 11.0),
        ShipFaceNormal::new(31.0, 27.0, 40.0, 11.0),
        ShipFaceNormal::new(31.0, -27.0, -40.0, 11.0),
        ShipFaceNormal::new(31.0, 27.0, -40.0, 11.0),
        ShipFaceNormal::new(31.0, -19.0, 38.0, 0.0),
        ShipFaceNormal::new(31.0, 19.0, 38.0, 0.0),
        ShipFaceNormal::new(31.0, -19.0, -38.0, 0.0),
        ShipFaceNormal::new(31.0, 19.0, -38.0, 0.0),
        ShipFaceNormal::new(31.0, -25.0, 37.0, -11.0),
        ShipFaceNormal::new(31.0, 25.0, 37.0, -11.0),
        ShipFaceNormal::new(31.0, 25.0, -37.0, -11.0),
        ShipFaceNormal::new(31.0, -25.0, -37.0, -11.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -112.0),
    ];

    let pythona_data: ShipData = ShipData {
        name: put_into_name("Pythona"),
        num_points: 11,
        num_lines: 26,
        num_faces: 13,
        max_loot: 5,
        scoop_type: 0,
        size: 6400.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 40,
        energy: 250,
        velocity: 20,
        missiles: 3,
        laser_strength: 13,
        points: pythona_point,
        lines: pythona_line,
        normals: pythona_face_normal,
    };

    let boa_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, 0.0, 93.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(0.0, 40.0, -87.0, 24.0, 0, 2, 3, 3),
        ShipPoint::new(38.0, -25.0, -99.0, 24.0, 0, 1, 4, 4),
        ShipPoint::new(-38.0, -25.0, -99.0, 24.0, 1, 2, 5, 5),
        ShipPoint::new(-38.0, 40.0, -59.0, 31.0, 2, 3, 6, 9),
        ShipPoint::new(38.0, 40.0, -59.0, 31.0, 0, 3, 6, 11),
        ShipPoint::new(62.0, 0.0, -67.0, 31.0, 0, 4, 8, 11),
        ShipPoint::new(24.0, -65.0, -79.0, 31.0, 1, 4, 8, 10),
        ShipPoint::new(-24.0, -65.0, -79.0, 31.0, 1, 5, 7, 10),
        ShipPoint::new(-62.0, 0.0, -67.0, 31.0, 2, 5, 7, 9),
        ShipPoint::new(0.0, 7.0, -107.0, 22.0, 0, 2, 10, 10),
        ShipPoint::new(13.0, -9.0, -107.0, 22.0, 0, 1, 10, 10),
        ShipPoint::new(-13.0, -9.0, -107.0, 22.0, 1, 2, 12, 12),
    ];

    let boa_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 6, 11, 0, 5),
        ShipLine::new(31, 8, 10, 0, 7),
        ShipLine::new(31, 7, 9, 0, 9),
        ShipLine::new(29, 6, 9, 0, 4),
        ShipLine::new(29, 8, 11, 0, 6),
        ShipLine::new(29, 7, 10, 0, 8),
        ShipLine::new(31, 3, 6, 4, 5),
        ShipLine::new(31, 0, 11, 5, 6),
        ShipLine::new(31, 4, 8, 6, 7),
        ShipLine::new(31, 1, 10, 7, 8),
        ShipLine::new(31, 5, 7, 8, 9),
        ShipLine::new(31, 2, 9, 4, 9),
        ShipLine::new(24, 2, 3, 1, 4),
        ShipLine::new(24, 0, 3, 1, 5),
        ShipLine::new(24, 2, 5, 3, 9),
        ShipLine::new(24, 1, 5, 3, 8),
        ShipLine::new(24, 0, 4, 2, 6),
        ShipLine::new(24, 1, 4, 2, 7),
        ShipLine::new(22, 0, 2, 1, 10),
        ShipLine::new(22, 0, 1, 2, 11),
        ShipLine::new(22, 1, 2, 3, 12),
        ShipLine::new(14, 0, 12, 10, 11),
        ShipLine::new(14, 1, 12, 11, 12),
        ShipLine::new(14, 2, 12, 12, 10),
    ];

    let boa_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 43.0, 37.0, -60.0),
        ShipFaceNormal::new(31.0, 0.0, -45.0, -89.0),
        ShipFaceNormal::new(31.0, -43.0, 37.0, -60.0),
        ShipFaceNormal::new(31.0, 0.0, 40.0, 0.0),
        ShipFaceNormal::new(31.0, 62.0, -32.0, -20.0),
        ShipFaceNormal::new(31.0, -62.0, -32.0, -20.0),
        ShipFaceNormal::new(31.0, 0.0, 23.0, 6.0),
        ShipFaceNormal::new(31.0, -23.0, -15.0, 9.0),
        ShipFaceNormal::new(31.0, 23.0, -15.0, 9.0),
        ShipFaceNormal::new(31.0, -26.0, 13.0, 10.0),
        ShipFaceNormal::new(31.0, 0.0, -31.0, 12.0),
        ShipFaceNormal::new(31.0, 26.0, 13.0, 10.0),
        ShipFaceNormal::new(14.0, 0.0, 0.0, -107.0),
    ];

    let boa_data: ShipData = ShipData {
        name: put_into_name("Boa"),
        num_points: 13,
        num_lines: 24,
        num_faces: 13,
        max_loot: 5,
        scoop_type: 0,
        size: 4900.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 40,
        energy: 250,
        velocity: 24,
        missiles: 4,
        laser_strength: 14,
        points: boa_point,
        lines: boa_line,
        normals: boa_face_normal,
    };

    let anacnda_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, 7.0, -58.0, 30.0, 0, 1, 5, 5),
        ShipPoint::new(-43.0, -13.0, -37.0, 30.0, 0, 1, 2, 2),
        ShipPoint::new(-26.0, -47.0, -3.0, 30.0, 0, 2, 3, 3),
        ShipPoint::new(26.0, -47.0, -3.0, 30.0, 0, 3, 4, 4),
        ShipPoint::new(43.0, -13.0, -37.0, 30.0, 0, 4, 5, 5),
        ShipPoint::new(0.0, 48.0, -49.0, 30.0, 1, 5, 6, 6),
        ShipPoint::new(-69.0, 15.0, -15.0, 30.0, 1, 2, 7, 7),
        ShipPoint::new(-43.0, -39.0, 40.0, 31.0, 2, 3, 8, 8),
        ShipPoint::new(43.0, -39.0, 40.0, 31.0, 3, 4, 9, 9),
        ShipPoint::new(69.0, 15.0, -15.0, 30.0, 4, 5, 10, 10),
        ShipPoint::new(-43.0, 53.0, -23.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-69.0, -1.0, 32.0, 31.0, 2, 7, 8, 8),
        ShipPoint::new(0.0, 0.0, 254.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(69.0, -1.0, 32.0, 31.0, 4, 9, 10, 10),
        ShipPoint::new(43.0, 53.0, -23.0, 31.0, 15, 15, 15, 15),
    ];

    let anacnda_line: Vec<ShipLine> = vec![
        ShipLine::new(30, 0, 1, 0, 1),
        ShipLine::new(30, 0, 2, 1, 2),
        ShipLine::new(30, 0, 3, 2, 3),
        ShipLine::new(30, 0, 4, 3, 4),
        ShipLine::new(30, 0, 5, 0, 4),
        ShipLine::new(29, 1, 5, 0, 5),
        ShipLine::new(29, 1, 2, 1, 6),
        ShipLine::new(29, 2, 3, 2, 7),
        ShipLine::new(29, 3, 4, 3, 8),
        ShipLine::new(29, 4, 5, 4, 9),
        ShipLine::new(30, 1, 6, 5, 10),
        ShipLine::new(30, 1, 7, 6, 10),
        ShipLine::new(30, 2, 7, 6, 11),
        ShipLine::new(30, 2, 8, 7, 11),
        ShipLine::new(31, 3, 8, 7, 12),
        ShipLine::new(31, 3, 9, 8, 12),
        ShipLine::new(30, 4, 9, 8, 13),
        ShipLine::new(30, 4, 10, 9, 13),
        ShipLine::new(30, 5, 10, 9, 14),
        ShipLine::new(30, 5, 6, 5, 14),
        ShipLine::new(30, 6, 11, 10, 14),
        ShipLine::new(31, 7, 11, 10, 12),
        ShipLine::new(31, 7, 8, 11, 12),
        ShipLine::new(31, 9, 10, 12, 13),
        ShipLine::new(31, 10, 11, 12, 14),
    ];

    let anacnda_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(30.0, 0.0, -51.0, -49.0),
        ShipFaceNormal::new(30.0, -51.0, 18.0, -87.0),
        ShipFaceNormal::new(30.0, -77.0, -57.0, -19.0),
        ShipFaceNormal::new(31.0, 0.0, -90.0, 16.0),
        ShipFaceNormal::new(30.0, 77.0, -57.0, -19.0),
        ShipFaceNormal::new(30.0, 51.0, 18.0, -87.0),
        ShipFaceNormal::new(30.0, 0.0, 111.0, -20.0),
        ShipFaceNormal::new(31.0, -97.0, 72.0, 24.0),
        ShipFaceNormal::new(31.0, -108.0, -68.0, 34.0),
        ShipFaceNormal::new(31.0, 108.0, -68.0, 34.0),
        ShipFaceNormal::new(31.0, 97.0, 72.0, 24.0),
        ShipFaceNormal::new(31.0, 0.0, 94.0, 18.0),
    ];

    let anacnda_data: ShipData = ShipData {
        name: put_into_name("Anaconda"),
        num_points: 15,
        num_lines: 25,
        num_faces: 12,
        max_loot: 7,
        scoop_type: 0,
        size: 10000.0,
        front_laser: 12,
        bounty: 0,
        vanish_point: 36,
        energy: 252,
        velocity: 14,
        missiles: 7,
        laser_strength: 31,
        points: anacnda_point,
        lines: anacnda_line,
        normals: anacnda_face_normal,
    };

    let hermit_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, 80.0, 0.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-80.0, -10.0, 0.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(0.0, -80.0, 0.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(70.0, -40.0, 0.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(60.0, 50.0, 0.0, 31.0, 6, 5, 13, 12),
        ShipPoint::new(50.0, 0.0, 60.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-40.0, 0.0, 70.0, 31.0, 1, 0, 3, 2),
        ShipPoint::new(0.0, 30.0, -75.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(0.0, -50.0, -60.0, 31.0, 9, 8, 11, 10),
    ];

    let hermit_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 7, 2, 0, 1),
        ShipLine::new(31, 13, 6, 0, 4),
        ShipLine::new(31, 12, 5, 3, 4),
        ShipLine::new(31, 11, 4, 2, 3),
        ShipLine::new(31, 10, 3, 1, 2),
        ShipLine::new(31, 3, 2, 1, 6),
        ShipLine::new(31, 3, 1, 2, 6),
        ShipLine::new(31, 4, 1, 2, 5),
        ShipLine::new(31, 1, 0, 5, 6),
        ShipLine::new(31, 6, 0, 0, 5),
        ShipLine::new(31, 5, 4, 3, 5),
        ShipLine::new(31, 2, 0, 0, 6),
        ShipLine::new(31, 6, 5, 4, 5),
        ShipLine::new(31, 10, 8, 1, 8),
        ShipLine::new(31, 8, 7, 1, 7),
        ShipLine::new(31, 13, 7, 0, 7),
        ShipLine::new(31, 13, 12, 4, 7),
        ShipLine::new(31, 12, 9, 3, 7),
        ShipLine::new(31, 11, 9, 3, 8),
        ShipLine::new(31, 11, 10, 2, 8),
        ShipLine::new(31, 9, 8, 7, 8),
    ];

    let hermit_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 9.0, 66.0, 81.0),
        ShipFaceNormal::new(31.0, 9.0, -66.0, 81.0),
        ShipFaceNormal::new(31.0, -72.0, 64.0, 31.0),
        ShipFaceNormal::new(31.0, -64.0, -73.0, 47.0),
        ShipFaceNormal::new(31.0, 45.0, -79.0, 65.0),
        ShipFaceNormal::new(31.0, 135.0, 15.0, 35.0),
        ShipFaceNormal::new(31.0, 38.0, 76.0, 70.0),
        ShipFaceNormal::new(31.0, -66.0, 59.0, -39.0),
        ShipFaceNormal::new(31.0, -67.0, -15.0, -80.0),
        ShipFaceNormal::new(31.0, 66.0, -14.0, -75.0),
        ShipFaceNormal::new(31.0, -70.0, -80.0, -40.0),
        ShipFaceNormal::new(31.0, 58.0, -102.0, -51.0),
        ShipFaceNormal::new(31.0, 81.0, 9.0, -67.0),
        ShipFaceNormal::new(31.0, 47.0, 94.0, -63.0),
    ];

    let hermit_data: ShipData = ShipData {
        name: put_into_name("Rock Hermit"),
        num_points: 9,
        num_lines: 21,
        num_faces: 14,
        max_loot: 7,
        scoop_type: 0,
        size: 6400.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 50,
        energy: 180,
        velocity: 30,
        missiles: 2,
        laser_strength: 1,
        points: hermit_point,
        lines: hermit_line,
        normals: hermit_face_normal,
    };

    let viper_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, 0.0, 72.0, 31.0, 2, 1, 4, 3),
        ShipPoint::new(0.0, 16.0, 24.0, 30.0, 1, 0, 2, 2),
        ShipPoint::new(0.0, -16.0, 24.0, 30.0, 4, 3, 5, 5),
        ShipPoint::new(48.0, 0.0, -24.0, 31.0, 4, 2, 6, 6),
        ShipPoint::new(-48.0, 0.0, -24.0, 31.0, 3, 1, 6, 6),
        ShipPoint::new(24.0, -16.0, -24.0, 30.0, 5, 4, 6, 6),
        ShipPoint::new(-24.0, -16.0, -24.0, 30.0, 3, 5, 6, 6),
        ShipPoint::new(24.0, 16.0, -24.0, 31.0, 2, 0, 6, 6),
        ShipPoint::new(-24.0, 16.0, -24.0, 31.0, 1, 0, 6, 6),
        ShipPoint::new(-32.0, 0.0, -24.0, 19.0, 6, 6, 6, 6),
        ShipPoint::new(32.0, 0.0, -24.0, 19.0, 6, 6, 6, 6),
        ShipPoint::new(8.0, 8.0, -24.0, 19.0, 6, 6, 6, 6),
        ShipPoint::new(-8.0, 8.0, -24.0, 19.0, 6, 6, 6, 6),
        ShipPoint::new(-8.0, -8.0, -24.0, 18.0, 6, 6, 6, 6),
        ShipPoint::new(8.0, -8.0, -24.0, 18.0, 6, 6, 6, 6),
    ];

    let viper_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 4, 2, 0, 3),
        ShipLine::new(30, 2, 1, 0, 1),
        ShipLine::new(30, 4, 3, 0, 2),
        ShipLine::new(31, 3, 1, 0, 4),
        ShipLine::new(30, 2, 0, 1, 7),
        ShipLine::new(30, 1, 0, 1, 8),
        ShipLine::new(30, 5, 4, 2, 5),
        ShipLine::new(30, 5, 3, 2, 6),
        ShipLine::new(31, 6, 0, 7, 8),
        ShipLine::new(30, 6, 5, 5, 6),
        ShipLine::new(31, 6, 1, 4, 8),
        ShipLine::new(30, 6, 3, 4, 6),
        ShipLine::new(31, 6, 2, 3, 7),
        ShipLine::new(30, 4, 6, 3, 5),
        ShipLine::new(19, 6, 6, 9, 12),
        ShipLine::new(18, 6, 6, 9, 13),
        ShipLine::new(19, 6, 6, 10, 11),
        ShipLine::new(18, 6, 6, 10, 14),
        ShipLine::new(16, 6, 6, 11, 14),
        ShipLine::new(16, 6, 6, 12, 13),
    ];

    let viper_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 0.0, 32.0, 0.0),
        ShipFaceNormal::new(31.0, -22.0, 33.0, 11.0),
        ShipFaceNormal::new(31.0, 22.0, 33.0, 11.0),
        ShipFaceNormal::new(31.0, -22.0, -33.0, 11.0),
        ShipFaceNormal::new(31.0, 22.0, -33.0, 11.0),
        ShipFaceNormal::new(31.0, 0.0, -32.0, 0.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -48.0),
    ];

    let viper_data: ShipData = ShipData {
        name: put_into_name("Viper"),
        num_points: 15,
        num_lines: 20,
        num_faces: 7,
        max_loot: 0,
        scoop_type: 0,
        size: 5625.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 23,
        energy: 140,
        velocity: 32,
        missiles: 1,
        laser_strength: 8,
        points: viper_point,
        lines: viper_line,
        normals: viper_face_normal,
    };

    let sidewnd_point: Vec<ShipPoint> = vec![
        ShipPoint::new(-32.0, 0.0, 36.0, 31.0, 1, 0, 5, 4),
        ShipPoint::new(32.0, 0.0, 36.0, 31.0, 2, 0, 6, 5),
        ShipPoint::new(64.0, 0.0, -28.0, 31.0, 3, 2, 6, 6),
        ShipPoint::new(-64.0, 0.0, -28.0, 31.0, 3, 1, 4, 4),
        ShipPoint::new(0.0, 16.0, -28.0, 31.0, 1, 0, 3, 2),
        ShipPoint::new(0.0, -16.0, -28.0, 31.0, 4, 3, 6, 5),
        ShipPoint::new(-12.0, 6.0, -28.0, 15.0, 3, 3, 3, 3),
        ShipPoint::new(12.0, 6.0, -28.0, 15.0, 3, 3, 3, 3),
        ShipPoint::new(12.0, -6.0, -28.0, 12.0, 3, 3, 3, 3),
        ShipPoint::new(-12.0, -6.0, -28.0, 12.0, 3, 3, 3, 3),
    ];

    let sidewnd_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 5, 0, 0, 1),
        ShipLine::new(31, 6, 2, 1, 2),
        ShipLine::new(31, 2, 0, 1, 4),
        ShipLine::new(31, 1, 0, 0, 4),
        ShipLine::new(31, 4, 1, 0, 3),
        ShipLine::new(31, 3, 1, 3, 4),
        ShipLine::new(31, 3, 2, 2, 4),
        ShipLine::new(31, 4, 3, 3, 5),
        ShipLine::new(31, 6, 3, 2, 5),
        ShipLine::new(31, 6, 5, 1, 5),
        ShipLine::new(31, 5, 4, 0, 5),
        ShipLine::new(15, 3, 3, 6, 7),
        ShipLine::new(12, 3, 3, 7, 8),
        ShipLine::new(12, 3, 3, 6, 9),
        ShipLine::new(12, 3, 3, 8, 9),
    ];

    let sidewnd_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 0.0, 32.0, 8.0),
        ShipFaceNormal::new(31.0, -12.0, 47.0, 6.0),
        ShipFaceNormal::new(31.0, 12.0, 47.0, 6.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -112.0),
        ShipFaceNormal::new(31.0, -12.0, -47.0, 6.0),
        ShipFaceNormal::new(31.0, 0.0, -32.0, 8.0),
        ShipFaceNormal::new(31.0, 12.0, -47.0, 6.0),
    ];

    let sidewnd_data: ShipData = ShipData {
        name: put_into_name("Sidewinder"),
        num_points: 10,
        num_lines: 15,
        num_faces: 7,
        max_loot: 0,
        scoop_type: 0,
        size: 4225.0,
        front_laser: 0,
        bounty: 50,
        vanish_point: 20,
        energy: 70,
        velocity: 37,
        missiles: 0,
        laser_strength: 8,
        points: sidewnd_point,
        lines: sidewnd_line,
        normals: sidewnd_face_normal,
    };

    let mamba_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, 0.0, 64.0, 31.0, 1, 0, 3, 2),
        ShipPoint::new(-64.0, -8.0, -32.0, 31.0, 2, 0, 4, 4),
        ShipPoint::new(-32.0, 8.0, -32.0, 30.0, 2, 1, 4, 4),
        ShipPoint::new(32.0, 8.0, -32.0, 30.0, 3, 1, 4, 4),
        ShipPoint::new(64.0, -8.0, -32.0, 31.0, 3, 0, 4, 4),
        ShipPoint::new(-4.0, 4.0, 16.0, 14.0, 1, 1, 1, 1),
        ShipPoint::new(4.0, 4.0, 16.0, 14.0, 1, 1, 1, 1),
        ShipPoint::new(8.0, 3.0, 28.0, 13.0, 1, 1, 1, 1),
        ShipPoint::new(-8.0, 3.0, 28.0, 13.0, 1, 1, 1, 1),
        ShipPoint::new(-20.0, -4.0, 16.0, 20.0, 0, 0, 0, 0),
        ShipPoint::new(20.0, -4.0, 16.0, 20.0, 0, 0, 0, 0),
        ShipPoint::new(-24.0, -7.0, -20.0, 20.0, 0, 0, 0, 0),
        ShipPoint::new(-16.0, -7.0, -20.0, 16.0, 0, 0, 0, 0),
        ShipPoint::new(16.0, -7.0, -20.0, 16.0, 0, 0, 0, 0),
        ShipPoint::new(24.0, -7.0, -20.0, 20.0, 0, 0, 0, 0),
        ShipPoint::new(-8.0, 4.0, -32.0, 13.0, 4, 4, 4, 4),
        ShipPoint::new(8.0, 4.0, -32.0, 13.0, 4, 4, 4, 4),
        ShipPoint::new(8.0, -4.0, -32.0, 14.0, 4, 4, 4, 4),
        ShipPoint::new(-8.0, -4.0, -32.0, 14.0, 4, 4, 4, 4),
        ShipPoint::new(-32.0, 4.0, -32.0, 7.0, 4, 4, 4, 4),
        ShipPoint::new(32.0, 4.0, -32.0, 7.0, 4, 4, 4, 4),
        ShipPoint::new(36.0, -4.0, -32.0, 7.0, 4, 4, 4, 4),
        ShipPoint::new(-36.0, -4.0, -32.0, 7.0, 4, 4, 4, 4),
        ShipPoint::new(-38.0, 0.0, -32.0, 5.0, 4, 4, 4, 4),
        ShipPoint::new(38.0, 0.0, -32.0, 5.0, 4, 4, 4, 4),
    ];

    let mamba_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 2, 0, 0, 1),
        ShipLine::new(31, 3, 0, 0, 4),
        ShipLine::new(31, 4, 0, 1, 4),
        ShipLine::new(30, 4, 2, 1, 2),
        ShipLine::new(30, 4, 1, 2, 3),
        ShipLine::new(30, 4, 3, 3, 4),
        ShipLine::new(14, 1, 1, 5, 6),
        ShipLine::new(12, 1, 1, 6, 7),
        ShipLine::new(13, 1, 1, 7, 8),
        ShipLine::new(12, 1, 1, 5, 8),
        ShipLine::new(20, 0, 0, 9, 11),
        ShipLine::new(16, 0, 0, 9, 12),
        ShipLine::new(16, 0, 0, 10, 13),
        ShipLine::new(20, 0, 0, 10, 14),
        ShipLine::new(14, 0, 0, 13, 14),
        ShipLine::new(14, 0, 0, 11, 12),
        ShipLine::new(13, 4, 4, 15, 16),
        ShipLine::new(14, 4, 4, 17, 18),
        ShipLine::new(12, 4, 4, 15, 18),
        ShipLine::new(12, 4, 4, 16, 17),
        ShipLine::new(7, 4, 4, 20, 21),
        ShipLine::new(5, 4, 4, 20, 24),
        ShipLine::new(5, 4, 4, 21, 24),
        ShipLine::new(7, 4, 4, 19, 22),
        ShipLine::new(5, 4, 4, 19, 23),
        ShipLine::new(5, 4, 4, 22, 23),
        ShipLine::new(30, 2, 1, 0, 2),
        ShipLine::new(30, 3, 1, 0, 3),
    ];

    let mamba_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(30.0, 0.0, -24.0, 2.0),
        ShipFaceNormal::new(30.0, 0.0, 24.0, 2.0),
        ShipFaceNormal::new(30.0, -32.0, 64.0, 16.0),
        ShipFaceNormal::new(30.0, 32.0, 64.0, 16.0),
        ShipFaceNormal::new(30.0, 0.0, 0.0, -127.0),
    ];

    let mamba_data: ShipData = ShipData {
        name: put_into_name("Mamba"),
        num_points: 25,
        num_lines: 28,
        num_faces: 5,
        max_loot: 1,
        scoop_type: 0,
        size: 4900.0,
        front_laser: 0,
        bounty: 150,
        vanish_point: 25,
        energy: 90,
        velocity: 30,
        missiles: 2,
        laser_strength: 9,
        points: mamba_point,
        lines: mamba_line,
        normals: mamba_face_normal,
    };

    let krait_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, 0.0, 96.0, 31.0, 0, 1, 2, 3),
        ShipPoint::new(0.0, 18.0, -48.0, 31.0, 0, 3, 4, 5),
        ShipPoint::new(0.0, -18.0, -48.0, 31.0, 1, 2, 4, 5),
        ShipPoint::new(90.0, 0.0, -3.0, 31.0, 0, 1, 4, 4),
        ShipPoint::new(-90.0, 0.0, -3.0, 31.0, 2, 3, 5, 5),
        ShipPoint::new(90.0, 0.0, 87.0, 30.0, 0, 1, 1, 1),
        ShipPoint::new(-90.0, 0.0, 87.0, 30.0, 2, 3, 3, 3),
        ShipPoint::new(0.0, 5.0, 53.0, 9.0, 0, 0, 3, 3),
        ShipPoint::new(0.0, 7.0, 38.0, 6.0, 0, 0, 3, 3),
        ShipPoint::new(-18.0, 7.0, 19.0, 9.0, 3, 3, 3, 3),
        ShipPoint::new(18.0, 7.0, 19.0, 9.0, 0, 0, 0, 0),
        ShipPoint::new(18.0, 11.0, -39.0, 8.0, 4, 4, 4, 4),
        ShipPoint::new(18.0, -11.0, -39.0, 8.0, 4, 4, 4, 4),
        ShipPoint::new(36.0, 0.0, -30.0, 8.0, 4, 4, 4, 4),
        ShipPoint::new(-18.0, 11.0, -39.0, 8.0, 5, 5, 5, 5),
        ShipPoint::new(-18.0, -11.0, -39.0, 8.0, 5, 5, 5, 5),
        ShipPoint::new(-36.0, 0.0, -30.0, 8.0, 5, 5, 5, 5),
    ];

    let krait_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 0, 3, 0, 1),
        ShipLine::new(31, 1, 2, 0, 2),
        ShipLine::new(31, 0, 1, 0, 3),
        ShipLine::new(31, 2, 3, 0, 4),
        ShipLine::new(31, 3, 5, 1, 4),
        ShipLine::new(31, 2, 5, 4, 2),
        ShipLine::new(31, 1, 4, 2, 3),
        ShipLine::new(31, 0, 4, 3, 1),
        ShipLine::new(30, 0, 1, 3, 5),
        ShipLine::new(30, 2, 3, 4, 6),
        ShipLine::new(8, 4, 5, 1, 2),
        ShipLine::new(9, 0, 0, 7, 10),
        ShipLine::new(6, 0, 0, 8, 10),
        ShipLine::new(9, 3, 3, 7, 9),
        ShipLine::new(6, 3, 3, 8, 9),
        ShipLine::new(8, 4, 4, 11, 13),
        ShipLine::new(8, 4, 4, 13, 12),
        ShipLine::new(7, 4, 4, 12, 11),
        ShipLine::new(7, 5, 5, 14, 15),
        ShipLine::new(8, 5, 5, 15, 16),
        ShipLine::new(8, 5, 5, 16, 14),
    ];

    let krait_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 3.0, 24.0, 3.0),
        ShipFaceNormal::new(31.0, 3.0, -24.0, 3.0),
        ShipFaceNormal::new(31.0, -3.0, -24.0, 3.0),
        ShipFaceNormal::new(31.0, -3.0, 24.0, 3.0),
        ShipFaceNormal::new(31.0, 38.0, 0.0, -77.0),
        ShipFaceNormal::new(31.0, -38.0, 0.0, -77.0),
    ];

    let krait_data: ShipData = ShipData {
        name: put_into_name("Krait"),
        num_points: 17,
        num_lines: 21,
        num_faces: 6,
        max_loot: 1,
        scoop_type: 0,
        size: 3600.0,
        front_laser: 0,
        bounty: 100,
        vanish_point: 20,
        energy: 80,
        velocity: 30,
        missiles: 0,
        laser_strength: 8,
        points: krait_point,
        lines: krait_line,
        normals: krait_face_normal,
    };

    let adder_point: Vec<ShipPoint> = vec![
        ShipPoint::new(-18.0, 0.0, 40.0, 31.0, 0, 1, 11, 10),
        ShipPoint::new(18.0, 0.0, 40.0, 31.0, 0, 1, 2, 0),
        ShipPoint::new(30.0, 0.0, -24.0, 31.0, 2, 3, 4, 0),
        ShipPoint::new(30.0, 0.0, -40.0, 31.0, 4, 5, 6, 0),
        ShipPoint::new(18.0, -7.0, -40.0, 31.0, 5, 6, 7, 10),
        ShipPoint::new(-18.0, -7.0, -40.0, 31.0, 7, 8, 10, 10),
        ShipPoint::new(-30.0, 0.0, -40.0, 31.0, 8, 9, 10, 10),
        ShipPoint::new(-30.0, 0.0, -24.0, 31.0, 9, 10, 11, 10),
        ShipPoint::new(-18.0, 7.0, -40.0, 31.0, 7, 8, 9, 10),
        ShipPoint::new(18.0, 7.0, -40.0, 31.0, 4, 6, 7, 10),
        ShipPoint::new(-18.0, 7.0, 13.0, 31.0, 0, 9, 11, 10),
        ShipPoint::new(18.0, 7.0, 13.0, 31.0, 0, 2, 4, 10),
        ShipPoint::new(-18.0, -7.0, 13.0, 31.0, 1, 10, 12, 10),
        ShipPoint::new(18.0, -7.0, 13.0, 31.0, 1, 3, 5, 10),
        ShipPoint::new(-11.0, 3.0, 29.0, 5.0, 0, 0, 0, 0),
        ShipPoint::new(11.0, 3.0, 29.0, 5.0, 0, 0, 0, 0),
        ShipPoint::new(11.0, 4.0, 24.0, 4.0, 0, 0, 0, 0),
        ShipPoint::new(-11.0, 4.0, 24.0, 4.0, 0, 0, 0, 0),
    ];

    let adder_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 0, 1, 0, 1),
        ShipLine::new(7, 2, 3, 1, 2),
        ShipLine::new(31, 4, 5, 2, 3),
        ShipLine::new(31, 5, 6, 3, 4),
        ShipLine::new(31, 7, 14, 4, 5),
        ShipLine::new(31, 8, 10, 5, 6),
        ShipLine::new(31, 9, 10, 6, 7),
        ShipLine::new(7, 11, 12, 7, 0),
        ShipLine::new(31, 4, 6, 3, 9),
        ShipLine::new(31, 7, 13, 9, 8),
        ShipLine::new(31, 8, 9, 8, 6),
        ShipLine::new(31, 0, 11, 0, 10),
        ShipLine::new(31, 9, 11, 7, 10),
        ShipLine::new(31, 0, 2, 1, 11),
        ShipLine::new(31, 2, 4, 2, 11),
        ShipLine::new(31, 1, 12, 0, 12),
        ShipLine::new(31, 10, 12, 7, 12),
        ShipLine::new(31, 1, 3, 1, 13),
        ShipLine::new(31, 3, 5, 2, 13),
        ShipLine::new(31, 0, 13, 10, 11),
        ShipLine::new(31, 1, 14, 12, 13),
        ShipLine::new(31, 9, 13, 8, 10),
        ShipLine::new(31, 4, 13, 9, 11),
        ShipLine::new(31, 10, 14, 5, 12),
        ShipLine::new(31, 5, 14, 4, 13),
        ShipLine::new(5, 0, 0, 14, 15),
        ShipLine::new(3, 0, 0, 15, 16),
        ShipLine::new(4, 0, 0, 16, 17),
        ShipLine::new(3, 0, 0, 17, 14),
    ];

    let adder_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 0.0, 39.0, 10.0),
        ShipFaceNormal::new(31.0, 0.0, -39.0, 10.0),
        ShipFaceNormal::new(31.0, 69.0, 50.0, 10.0),
        ShipFaceNormal::new(31.0, 69.0, -50.0, 10.0),
        ShipFaceNormal::new(31.0, 30.0, 52.0, 0.0),
        ShipFaceNormal::new(31.0, 30.0, -52.0, 0.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -160.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -160.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -160.0),
        ShipFaceNormal::new(31.0, -30.0, 52.0, 0.0),
        ShipFaceNormal::new(31.0, -30.0, -52.0, 0.0),
        ShipFaceNormal::new(31.0, -69.0, 50.0, 10.0),
        ShipFaceNormal::new(31.0, -69.0, -50.0, 10.0),
        ShipFaceNormal::new(31.0, 0.0, 28.0, 0.0),
        ShipFaceNormal::new(31.0, 0.0, -28.0, 0.0),
    ];

    let adder_data: ShipData = ShipData {
        name: put_into_name("Adder"),
        num_points: 18,
        num_lines: 29,
        num_faces: 15,
        max_loot: 0,
        scoop_type: 0,
        size: 2500.0,
        front_laser: 0,
        bounty: 40,
        vanish_point: 20,
        energy: 85,
        velocity: 24,
        missiles: 0,
        laser_strength: 8,
        points: adder_point,
        lines: adder_line,
        normals: adder_face_normal,
    };

    let gecko_point: Vec<ShipPoint> = vec![
        ShipPoint::new(-10.0, -4.0, 47.0, 31.0, 0, 3, 4, 5),
        ShipPoint::new(10.0, -4.0, 47.0, 31.0, 0, 1, 2, 3),
        ShipPoint::new(-16.0, 8.0, -23.0, 31.0, 0, 5, 6, 7),
        ShipPoint::new(16.0, 8.0, -23.0, 31.0, 0, 1, 7, 8),
        ShipPoint::new(-66.0, 0.0, -3.0, 31.0, 4, 5, 6, 6),
        ShipPoint::new(66.0, 0.0, -3.0, 31.0, 1, 2, 8, 8),
        ShipPoint::new(-20.0, -14.0, -23.0, 31.0, 3, 4, 6, 7),
        ShipPoint::new(20.0, -14.0, -23.0, 31.0, 2, 3, 7, 8),
        ShipPoint::new(-8.0, -6.0, 33.0, 16.0, 3, 3, 3, 3),
        ShipPoint::new(8.0, -6.0, 33.0, 17.0, 3, 3, 3, 3),
        ShipPoint::new(-8.0, -13.0, -16.0, 16.0, 3, 3, 3, 3),
        ShipPoint::new(8.0, -13.0, -16.0, 17.0, 3, 3, 3, 3),
    ];

    let gecko_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 0, 3, 0, 1),
        ShipLine::new(31, 1, 2, 1, 5),
        ShipLine::new(31, 1, 8, 5, 3),
        ShipLine::new(31, 0, 7, 3, 2),
        ShipLine::new(31, 5, 6, 2, 4),
        ShipLine::new(31, 4, 5, 4, 0),
        ShipLine::new(31, 2, 8, 5, 7),
        ShipLine::new(31, 3, 7, 7, 6),
        ShipLine::new(31, 4, 6, 6, 4),
        ShipLine::new(29, 0, 5, 0, 2),
        ShipLine::new(30, 0, 1, 1, 3),
        ShipLine::new(29, 3, 4, 0, 6),
        ShipLine::new(30, 2, 3, 1, 7),
        ShipLine::new(20, 6, 7, 2, 6),
        ShipLine::new(20, 7, 8, 3, 7),
        ShipLine::new(16, 3, 3, 8, 10),
        ShipLine::new(17, 3, 3, 9, 11),
    ];

    let gecko_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 0.0, 31.0, 5.0),
        ShipFaceNormal::new(31.0, 4.0, 45.0, 8.0),
        ShipFaceNormal::new(31.0, 25.0, -108.0, 19.0),
        ShipFaceNormal::new(31.0, 0.0, -84.0, 12.0),
        ShipFaceNormal::new(31.0, -25.0, -108.0, 19.0),
        ShipFaceNormal::new(31.0, -4.0, 45.0, 8.0),
        ShipFaceNormal::new(31.0, -88.0, 16.0, -214.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -187.0),
        ShipFaceNormal::new(31.0, 88.0, 16.0, -214.0),
    ];

    let gecko_data: ShipData = ShipData {
        name: put_into_name("Gecko"),
        num_points: 12,
        num_lines: 17,
        num_faces: 9,
        max_loot: 0,
        scoop_type: 0,
        size: 9801.0,
        front_laser: 0,
        bounty: 55,
        vanish_point: 18,
        energy: 70,
        velocity: 30,
        missiles: 0,
        laser_strength: 8,
        points: gecko_point,
        lines: gecko_line,
        normals: gecko_face_normal,
    };
    let cobra1_point: Vec<ShipPoint> = vec![
        ShipPoint::new(-18.0, -1.0, 50.0, 31.0, 0, 1, 2, 3),
        ShipPoint::new(18.0, -1.0, 50.0, 31.0, 0, 1, 4, 5),
        ShipPoint::new(-66.0, 0.0, 7.0, 31.0, 2, 3, 8, 8),
        ShipPoint::new(66.0, 0.0, 7.0, 31.0, 4, 5, 9, 9),
        ShipPoint::new(-32.0, 12.0, -38.0, 31.0, 2, 6, 7, 8),
        ShipPoint::new(32.0, 12.0, -38.0, 31.0, 4, 6, 7, 9),
        ShipPoint::new(-54.0, -12.0, -38.0, 31.0, 1, 3, 7, 8),
        ShipPoint::new(54.0, -12.0, -38.0, 31.0, 1, 5, 7, 9),
        ShipPoint::new(0.0, 12.0, -6.0, 20.0, 0, 2, 4, 6),
        ShipPoint::new(0.0, -1.0, 50.0, 2.0, 0, 1, 1, 1),
        ShipPoint::new(0.0, -1.0, 60.0, 31.0, 0, 1, 1, 1),
    ];

    let cobra1_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 0, 1, 1, 0),
        ShipLine::new(31, 2, 3, 0, 2),
        ShipLine::new(31, 3, 8, 2, 6),
        ShipLine::new(31, 1, 7, 6, 7),
        ShipLine::new(31, 5, 9, 7, 3),
        ShipLine::new(31, 4, 5, 3, 1),
        ShipLine::new(31, 2, 8, 2, 4),
        ShipLine::new(31, 6, 7, 4, 5),
        ShipLine::new(31, 4, 9, 5, 3),
        ShipLine::new(20, 0, 2, 0, 8),
        ShipLine::new(20, 0, 4, 8, 1),
        ShipLine::new(16, 2, 6, 4, 8),
        ShipLine::new(16, 4, 6, 8, 5),
        ShipLine::new(31, 7, 8, 4, 6),
        ShipLine::new(31, 7, 9, 5, 7),
        ShipLine::new(20, 1, 3, 0, 6),
        ShipLine::new(20, 1, 5, 1, 7),
        ShipLine::new(2, 0, 1, 10, 9),
    ];

    let cobra1_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 0.0, 41.0, 10.0),
        ShipFaceNormal::new(31.0, 0.0, -27.0, 3.0),
        ShipFaceNormal::new(31.0, -8.0, 46.0, 8.0),
        ShipFaceNormal::new(31.0, -12.0, -57.0, 12.0),
        ShipFaceNormal::new(31.0, 8.0, 46.0, 8.0),
        ShipFaceNormal::new(31.0, 12.0, -57.0, 12.0),
        ShipFaceNormal::new(31.0, 0.0, 49.0, 0.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -154.0),
        ShipFaceNormal::new(31.0, -121.0, 111.0, -62.0),
        ShipFaceNormal::new(31.0, 121.0, 111.0, -62.0),
    ];

    let cobra1_data: ShipData = ShipData {
        name: put_into_name("Cobra MkI"),
        num_points: 11,
        num_lines: 18,
        num_faces: 10,
        max_loot: 3,
        scoop_type: 0,
        size: 9801.0,
        front_laser: 10,
        bounty: 75,
        vanish_point: 19,
        energy: 90,
        velocity: 26,
        missiles: 2,
        laser_strength: 9,
        points: cobra1_point,
        lines: cobra1_line,
        normals: cobra1_face_normal,
    };

    let worm_point: Vec<ShipPoint> = vec![
        ShipPoint::new(10.0, -10.0, 35.0, 31.0, 0, 2, 7, 7),
        ShipPoint::new(-10.0, -10.0, 35.0, 31.0, 0, 3, 7, 7),
        ShipPoint::new(5.0, 6.0, 15.0, 31.0, 0, 1, 2, 4),
        ShipPoint::new(-5.0, 6.0, 15.0, 31.0, 0, 1, 3, 5),
        ShipPoint::new(15.0, -10.0, 25.0, 31.0, 2, 4, 7, 7),
        ShipPoint::new(-15.0, -10.0, 25.0, 31.0, 3, 5, 7, 7),
        ShipPoint::new(26.0, -10.0, -25.0, 31.0, 4, 6, 7, 7),
        ShipPoint::new(-26.0, -10.0, -25.0, 31.0, 5, 6, 7, 7),
        ShipPoint::new(8.0, 14.0, -25.0, 31.0, 1, 4, 6, 6),
        ShipPoint::new(-8.0, 14.0, -25.0, 31.0, 1, 5, 6, 6),
    ];

    let worm_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 0, 7, 0, 1),
        ShipLine::new(31, 3, 7, 1, 5),
        ShipLine::new(31, 5, 7, 5, 7),
        ShipLine::new(31, 6, 7, 7, 6),
        ShipLine::new(31, 4, 7, 6, 4),
        ShipLine::new(31, 2, 7, 4, 0),
        ShipLine::new(31, 0, 2, 0, 2),
        ShipLine::new(31, 0, 3, 1, 3),
        ShipLine::new(31, 2, 4, 4, 2),
        ShipLine::new(31, 3, 5, 5, 3),
        ShipLine::new(31, 1, 4, 2, 8),
        ShipLine::new(31, 4, 6, 8, 6),
        ShipLine::new(31, 1, 5, 3, 9),
        ShipLine::new(31, 5, 6, 9, 7),
        ShipLine::new(31, 0, 1, 2, 3),
        ShipLine::new(31, 1, 6, 8, 9),
    ];

    let worm_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 0.0, 88.0, 70.0),
        ShipFaceNormal::new(31.0, 0.0, 69.0, 14.0),
        ShipFaceNormal::new(31.0, 70.0, 66.0, 35.0),
        ShipFaceNormal::new(31.0, -70.0, 66.0, 35.0),
        ShipFaceNormal::new(31.0, 64.0, 49.0, 14.0),
        ShipFaceNormal::new(31.0, -64.0, 49.0, 14.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -200.0),
        ShipFaceNormal::new(31.0, 0.0, -80.0, 0.0),
    ];

    let worm_data: ShipData = ShipData {
        name: put_into_name("Worm"),
        num_points: 10,
        num_lines: 16,
        num_faces: 8,
        max_loot: 0,
        scoop_type: 0,
        size: 9801.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 19,
        energy: 30,
        velocity: 23,
        missiles: 0,
        laser_strength: 4,
        points: worm_point,
        lines: worm_line,
        normals: worm_face_normal,
    };

    let cobra3b_point: Vec<ShipPoint> = vec![
        ShipPoint::new(32.0, 0.0, 76.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-32.0, 0.0, 76.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(0.0, 26.0, 24.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-120.0, -3.0, -8.0, 31.0, 7, 3, 10, 10),
        ShipPoint::new(120.0, -3.0, -8.0, 31.0, 8, 4, 12, 12),
        ShipPoint::new(-88.0, 16.0, -40.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(88.0, 16.0, -40.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(128.0, -8.0, -40.0, 31.0, 9, 8, 12, 12),
        ShipPoint::new(-128.0, -8.0, -40.0, 31.0, 9, 7, 10, 10),
        ShipPoint::new(0.0, 26.0, -40.0, 31.0, 6, 5, 9, 9),
        ShipPoint::new(-32.0, -24.0, -40.0, 31.0, 10, 9, 11, 11),
        ShipPoint::new(32.0, -24.0, -40.0, 31.0, 11, 9, 12, 12),
        ShipPoint::new(-36.0, 8.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(-8.0, 12.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(8.0, 12.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(36.0, 8.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(36.0, -12.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(8.0, -16.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(-8.0, -16.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(-36.0, -12.0, -40.0, 20.0, 9, 9, 9, 9),
        ShipPoint::new(0.0, 0.0, 76.0, 6.0, 11, 0, 11, 11),
        ShipPoint::new(0.0, 0.0, 90.0, 31.0, 11, 0, 11, 11),
        ShipPoint::new(-80.0, -6.0, -40.0, 8.0, 9, 9, 9, 9),
        ShipPoint::new(-80.0, 6.0, -40.0, 8.0, 9, 9, 9, 9),
        ShipPoint::new(-88.0, 0.0, -40.0, 6.0, 9, 9, 9, 9),
        ShipPoint::new(80.0, 6.0, -40.0, 8.0, 9, 9, 9, 9),
        ShipPoint::new(88.0, 0.0, -40.0, 6.0, 9, 9, 9, 9),
        ShipPoint::new(80.0, -6.0, -40.0, 8.0, 9, 9, 9, 9),
    ];

    let cobra3b_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 11, 0, 0, 1),
        ShipLine::new(31, 12, 4, 0, 4),
        ShipLine::new(31, 10, 3, 1, 3),
        ShipLine::new(31, 10, 7, 3, 8),
        ShipLine::new(31, 12, 8, 4, 7),
        ShipLine::new(31, 9, 8, 6, 7),
        ShipLine::new(31, 9, 6, 6, 9),
        ShipLine::new(31, 9, 5, 5, 9),
        ShipLine::new(31, 9, 7, 5, 8),
        ShipLine::new(31, 5, 1, 2, 5),
        ShipLine::new(31, 6, 2, 2, 6),
        ShipLine::new(31, 7, 3, 3, 5),
        ShipLine::new(31, 8, 4, 4, 6),
        ShipLine::new(31, 1, 0, 1, 2),
        ShipLine::new(31, 2, 0, 0, 2),
        ShipLine::new(31, 10, 9, 8, 10),
        ShipLine::new(31, 11, 9, 10, 11),
        ShipLine::new(31, 12, 9, 7, 11),
        ShipLine::new(31, 11, 10, 1, 10),
        ShipLine::new(31, 12, 11, 0, 11),
        ShipLine::new(29, 3, 1, 1, 5),
        ShipLine::new(29, 4, 2, 0, 6),
        ShipLine::new(6, 11, 0, 20, 21),
        ShipLine::new(20, 9, 9, 12, 13),
        ShipLine::new(20, 9, 9, 18, 19),
        ShipLine::new(20, 9, 9, 14, 15),
        ShipLine::new(20, 9, 9, 16, 17),
        ShipLine::new(19, 9, 9, 15, 16),
        ShipLine::new(17, 9, 9, 14, 17),
        ShipLine::new(19, 9, 9, 13, 18),
        ShipLine::new(19, 9, 9, 12, 19),
        ShipLine::new(30, 6, 5, 2, 9),
        ShipLine::new(6, 9, 9, 22, 24),
        ShipLine::new(6, 9, 9, 23, 24),
        ShipLine::new(8, 9, 9, 22, 23),
        ShipLine::new(6, 9, 9, 25, 26),
        ShipLine::new(6, 9, 9, 26, 27),
        ShipLine::new(8, 9, 9, 25, 27),
    ];

    let cobra3b_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 0.0, 62.0, 31.0),
        ShipFaceNormal::new(31.0, -18.0, 55.0, 16.0),
        ShipFaceNormal::new(31.0, 18.0, 55.0, 16.0),
        ShipFaceNormal::new(31.0, -16.0, 52.0, 14.0),
        ShipFaceNormal::new(31.0, 16.0, 52.0, 14.0),
        ShipFaceNormal::new(31.0, -14.0, 47.0, 0.0),
        ShipFaceNormal::new(31.0, 14.0, 47.0, 0.0),
        ShipFaceNormal::new(31.0, -61.0, 102.0, 0.0),
        ShipFaceNormal::new(31.0, 61.0, 102.0, 0.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -80.0),
        ShipFaceNormal::new(31.0, -7.0, -42.0, 9.0),
        ShipFaceNormal::new(31.0, 0.0, -30.0, 6.0),
        ShipFaceNormal::new(31.0, 7.0, -42.0, 9.0),
    ];

    let cobra3b_data: ShipData = ShipData {
        name: put_into_name("Cobra MkIIIb"),
        num_points: 28,
        num_lines: 38,
        num_faces: 13,
        max_loot: 1,
        scoop_type: 0,
        size: 9025.0,
        front_laser: 21,
        bounty: 175,
        vanish_point: 50,
        energy: 150,
        velocity: 28,
        missiles: 2,
        laser_strength: 9,
        points: cobra3b_point,
        lines: cobra3b_line,
        normals: cobra3b_face_normal,
    };

    let asp2_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, -18.0, 0.0, 22.0, 0, 1, 2, 2),
        ShipPoint::new(0.0, -9.0, -45.0, 31.0, 1, 2, 11, 11),
        ShipPoint::new(43.0, 0.0, -45.0, 31.0, 1, 6, 11, 11),
        ShipPoint::new(69.0, -3.0, 0.0, 31.0, 1, 6, 7, 9),
        ShipPoint::new(43.0, -14.0, 28.0, 31.0, 0, 1, 7, 7),
        ShipPoint::new(-43.0, 0.0, -45.0, 31.0, 2, 5, 11, 11),
        ShipPoint::new(-69.0, -3.0, 0.0, 31.0, 2, 5, 8, 10),
        ShipPoint::new(-43.0, -14.0, 28.0, 31.0, 0, 2, 8, 8),
        ShipPoint::new(26.0, -7.0, 73.0, 31.0, 0, 4, 7, 9),
        ShipPoint::new(-26.0, -7.0, 73.0, 31.0, 0, 4, 8, 10),
        ShipPoint::new(43.0, 14.0, 28.0, 31.0, 3, 4, 6, 9),
        ShipPoint::new(-43.0, 14.0, 28.0, 31.0, 3, 4, 5, 10),
        ShipPoint::new(0.0, 9.0, -45.0, 31.0, 3, 5, 6, 11),
        ShipPoint::new(-17.0, 0.0, -45.0, 10.0, 11, 11, 11, 11),
        ShipPoint::new(17.0, 0.0, -45.0, 9.0, 11, 11, 11, 11),
        ShipPoint::new(0.0, -4.0, -45.0, 10.0, 11, 11, 11, 11),
        ShipPoint::new(0.0, 4.0, -45.0, 8.0, 11, 11, 11, 11),
        ShipPoint::new(0.0, -7.0, 73.0, 10.0, 0, 4, 0, 4),
        ShipPoint::new(0.0, -7.0, 83.0, 10.0, 0, 4, 0, 4),
    ];

    let asp2_line: Vec<ShipLine> = vec![
        ShipLine::new(22, 1, 2, 0, 1),
        ShipLine::new(22, 0, 1, 0, 4),
        ShipLine::new(22, 0, 2, 0, 7),
        ShipLine::new(31, 1, 11, 1, 2),
        ShipLine::new(31, 1, 6, 2, 3),
        ShipLine::new(16, 7, 9, 3, 8),
        ShipLine::new(31, 0, 4, 8, 9),
        ShipLine::new(16, 8, 10, 6, 9),
        ShipLine::new(31, 2, 5, 5, 6),
        ShipLine::new(31, 2, 11, 1, 5),
        ShipLine::new(31, 1, 7, 3, 4),
        ShipLine::new(31, 0, 7, 4, 8),
        ShipLine::new(31, 2, 8, 6, 7),
        ShipLine::new(31, 0, 8, 7, 9),
        ShipLine::new(31, 6, 11, 2, 12),
        ShipLine::new(31, 5, 11, 5, 12),
        ShipLine::new(22, 3, 6, 10, 12),
        ShipLine::new(22, 3, 5, 11, 12),
        ShipLine::new(22, 3, 4, 10, 11),
        ShipLine::new(31, 5, 10, 6, 11),
        ShipLine::new(31, 4, 10, 9, 11),
        ShipLine::new(31, 6, 9, 3, 10),
        ShipLine::new(31, 4, 9, 8, 10),
        ShipLine::new(10, 11, 11, 13, 15),
        ShipLine::new(9, 11, 11, 15, 14),
        ShipLine::new(8, 11, 11, 14, 16),
        ShipLine::new(8, 11, 11, 16, 13),
        ShipLine::new(10, 0, 4, 18, 17),
    ];

    let asp2_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 0.0, -35.0, 5.0),
        ShipFaceNormal::new(31.0, 8.0, -38.0, -7.0),
        ShipFaceNormal::new(31.0, -8.0, -38.0, -7.0),
        ShipFaceNormal::new(22.0, 0.0, 24.0, -1.0),
        ShipFaceNormal::new(31.0, 0.0, 43.0, 19.0),
        ShipFaceNormal::new(31.0, -6.0, 28.0, -2.0),
        ShipFaceNormal::new(31.0, 6.0, 28.0, -2.0),
        ShipFaceNormal::new(31.0, 59.0, -64.0, 31.0),
        ShipFaceNormal::new(31.0, -59.0, -64.0, 31.0),
        ShipFaceNormal::new(31.0, 80.0, 46.0, 50.0),
        ShipFaceNormal::new(31.0, -80.0, 46.0, 50.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -90.0),
    ];

    let asp2_data: ShipData = ShipData {
        name: put_into_name("Asp MkII"),
        num_points: 19,
        num_lines: 28,
        num_faces: 12,
        max_loot: 0,
        scoop_type: 0,
        size: 3600.0,
        front_laser: 8,
        bounty: 200,
        vanish_point: 40,
        energy: 150,
        velocity: 40,
        missiles: 1,
        laser_strength: 20,
        points: asp2_point,
        lines: asp2_line,
        normals: asp2_face_normal,
    };

    let pythonb_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, 0.0, 224.0, 31.0, 1, 0, 3, 2),
        ShipPoint::new(0.0, 48.0, 48.0, 31.0, 1, 0, 5, 4),
        ShipPoint::new(96.0, 0.0, -16.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-96.0, 0.0, -16.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(0.0, 48.0, -32.0, 31.0, 5, 4, 9, 8),
        ShipPoint::new(0.0, 24.0, -112.0, 31.0, 8, 9, 12, 12),
        ShipPoint::new(-48.0, 0.0, -112.0, 31.0, 11, 8, 12, 12),
        ShipPoint::new(48.0, 0.0, -112.0, 31.0, 10, 9, 12, 12),
        ShipPoint::new(0.0, -48.0, 48.0, 31.0, 3, 2, 7, 6),
        ShipPoint::new(0.0, -48.0, -32.0, 31.0, 7, 6, 11, 10),
        ShipPoint::new(0.0, -24.0, -112.0, 31.0, 11, 10, 12, 12),
    ];

    let pythonb_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 3, 2, 0, 8),
        ShipLine::new(31, 2, 0, 0, 3),
        ShipLine::new(31, 3, 1, 0, 2),
        ShipLine::new(31, 1, 0, 0, 1),
        ShipLine::new(31, 5, 9, 2, 4),
        ShipLine::new(31, 5, 1, 1, 2),
        ShipLine::new(31, 3, 7, 2, 8),
        ShipLine::new(31, 4, 0, 1, 3),
        ShipLine::new(31, 6, 2, 3, 8),
        ShipLine::new(31, 10, 7, 2, 9),
        ShipLine::new(31, 8, 4, 3, 4),
        ShipLine::new(31, 11, 6, 3, 9),
        ShipLine::new(7, 8, 8, 3, 5),
        ShipLine::new(7, 11, 11, 3, 10),
        ShipLine::new(7, 9, 9, 2, 5),
        ShipLine::new(7, 10, 10, 2, 10),
        ShipLine::new(31, 10, 9, 2, 7),
        ShipLine::new(31, 11, 8, 3, 6),
        ShipLine::new(31, 12, 8, 5, 6),
        ShipLine::new(31, 12, 9, 5, 7),
        ShipLine::new(31, 10, 12, 7, 10),
        ShipLine::new(31, 12, 11, 6, 10),
        ShipLine::new(31, 9, 8, 4, 5),
        ShipLine::new(31, 11, 10, 9, 10),
        ShipLine::new(31, 5, 4, 1, 4),
        ShipLine::new(31, 7, 6, 8, 9),
    ];

    let pythonb_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, -27.0, 40.0, 11.0),
        ShipFaceNormal::new(31.0, 27.0, 40.0, 11.0),
        ShipFaceNormal::new(31.0, -27.0, -40.0, 11.0),
        ShipFaceNormal::new(31.0, 27.0, -40.0, 11.0),
        ShipFaceNormal::new(31.0, -19.0, 38.0, 0.0),
        ShipFaceNormal::new(31.0, 19.0, 38.0, 0.0),
        ShipFaceNormal::new(31.0, -19.0, -38.0, 0.0),
        ShipFaceNormal::new(31.0, 19.0, -38.0, 0.0),
        ShipFaceNormal::new(31.0, -25.0, 37.0, -11.0),
        ShipFaceNormal::new(31.0, 25.0, 37.0, -11.0),
        ShipFaceNormal::new(31.0, 25.0, -37.0, -11.0),
        ShipFaceNormal::new(31.0, -25.0, -37.0, -11.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -112.0),
    ];

    let pythonb_data: ShipData = ShipData {
        name: put_into_name("Python"),
        num_points: 11,
        num_lines: 26,
        num_faces: 13,
        max_loot: 2,
        scoop_type: 0,
        size: 6400.0,
        front_laser: 0,
        bounty: 200,
        vanish_point: 40,
        energy: 250,
        velocity: 20,
        missiles: 3,
        laser_strength: 13,
        points: pythonb_point,
        lines: pythonb_line,
        normals: pythonb_face_normal,
    };

    let ferdlce_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, -14.0, 108.0, 31.0, 0, 1, 5, 9),
        ShipPoint::new(-40.0, -14.0, -4.0, 31.0, 1, 2, 9, 9),
        ShipPoint::new(-12.0, -14.0, -52.0, 31.0, 2, 3, 9, 9),
        ShipPoint::new(12.0, -14.0, -52.0, 31.0, 3, 4, 9, 9),
        ShipPoint::new(40.0, -14.0, -4.0, 31.0, 4, 5, 9, 9),
        ShipPoint::new(-40.0, 14.0, -4.0, 28.0, 0, 1, 2, 6),
        ShipPoint::new(-12.0, 2.0, -52.0, 28.0, 2, 3, 6, 7),
        ShipPoint::new(12.0, 2.0, -52.0, 28.0, 3, 4, 7, 8),
        ShipPoint::new(40.0, 14.0, -4.0, 28.0, 0, 4, 5, 8),
        ShipPoint::new(0.0, 18.0, -20.0, 15.0, 0, 6, 7, 8),
        ShipPoint::new(-3.0, -11.0, 97.0, 11.0, 0, 0, 0, 0),
        ShipPoint::new(-26.0, 8.0, 18.0, 9.0, 0, 0, 0, 0),
        ShipPoint::new(-16.0, 14.0, -4.0, 11.0, 0, 0, 0, 0),
        ShipPoint::new(3.0, -11.0, 97.0, 11.0, 0, 0, 0, 0),
        ShipPoint::new(26.0, 8.0, 18.0, 9.0, 0, 0, 0, 0),
        ShipPoint::new(16.0, 14.0, -4.0, 11.0, 0, 0, 0, 0),
        ShipPoint::new(0.0, -14.0, -20.0, 12.0, 9, 9, 9, 9),
        ShipPoint::new(-14.0, -14.0, 44.0, 12.0, 9, 9, 9, 9),
        ShipPoint::new(14.0, -14.0, 44.0, 12.0, 9, 9, 9, 9),
    ];

    let ferdlce_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 1, 9, 0, 1),
        ShipLine::new(31, 2, 9, 1, 2),
        ShipLine::new(31, 3, 9, 2, 3),
        ShipLine::new(31, 4, 9, 3, 4),
        ShipLine::new(31, 5, 9, 0, 4),
        ShipLine::new(28, 0, 1, 0, 5),
        ShipLine::new(28, 2, 6, 5, 6),
        ShipLine::new(28, 3, 7, 6, 7),
        ShipLine::new(28, 4, 8, 7, 8),
        ShipLine::new(28, 0, 5, 0, 8),
        ShipLine::new(15, 0, 6, 5, 9),
        ShipLine::new(11, 6, 7, 6, 9),
        ShipLine::new(11, 7, 8, 7, 9),
        ShipLine::new(15, 0, 8, 8, 9),
        ShipLine::new(14, 1, 2, 1, 5),
        ShipLine::new(14, 2, 3, 2, 6),
        ShipLine::new(14, 3, 4, 3, 7),
        ShipLine::new(14, 4, 5, 4, 8),
        ShipLine::new(8, 0, 0, 10, 11),
        ShipLine::new(9, 0, 0, 11, 12),
        ShipLine::new(11, 0, 0, 10, 12),
        ShipLine::new(8, 0, 0, 13, 14),
        ShipLine::new(9, 0, 0, 14, 15),
        ShipLine::new(11, 0, 0, 13, 15),
        ShipLine::new(12, 9, 9, 16, 17),
        ShipLine::new(12, 9, 9, 16, 18),
        ShipLine::new(8, 9, 9, 17, 18),
    ];

    let ferdlce_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(28.0, 0.0, 24.0, 6.0),
        ShipFaceNormal::new(31.0, -68.0, 0.0, 24.0),
        ShipFaceNormal::new(31.0, -63.0, 0.0, -37.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -104.0),
        ShipFaceNormal::new(31.0, 63.0, 0.0, -37.0),
        ShipFaceNormal::new(31.0, 68.0, 0.0, 24.0),
        ShipFaceNormal::new(28.0, -12.0, 46.0, -19.0),
        ShipFaceNormal::new(28.0, 0.0, 45.0, -22.0),
        ShipFaceNormal::new(28.0, 12.0, 46.0, -19.0),
        ShipFaceNormal::new(31.0, 0.0, -28.0, 0.0),
    ];

    let ferdlce_data: ShipData = ShipData {
        name: put_into_name("Fer-de-Lance"),
        num_points: 19,
        num_lines: 27,
        num_faces: 10,
        max_loot: 0,
        scoop_type: 0,
        size: 1600.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 40,
        energy: 160,
        velocity: 30,
        missiles: 2,
        laser_strength: 9,
        points: ferdlce_point,
        lines: ferdlce_line,
        normals: ferdlce_face_normal,
    };

    let moray_point: Vec<ShipPoint> = vec![
        ShipPoint::new(15.0, 0.0, 65.0, 31.0, 0, 2, 7, 8),
        ShipPoint::new(-15.0, 0.0, 65.0, 31.0, 0, 1, 6, 7),
        ShipPoint::new(0.0, 18.0, -40.0, 17.0, 15, 15, 15, 15),
        ShipPoint::new(-60.0, 0.0, 0.0, 31.0, 1, 3, 6, 6),
        ShipPoint::new(60.0, 0.0, 0.0, 31.0, 2, 5, 8, 8),
        ShipPoint::new(30.0, -27.0, -10.0, 24.0, 4, 5, 7, 8),
        ShipPoint::new(-30.0, -27.0, -10.0, 24.0, 3, 4, 6, 7),
        ShipPoint::new(-9.0, -4.0, -25.0, 7.0, 4, 4, 4, 4),
        ShipPoint::new(9.0, -4.0, -25.0, 7.0, 4, 4, 4, 4),
        ShipPoint::new(0.0, -18.0, -16.0, 7.0, 4, 4, 4, 4),
        ShipPoint::new(13.0, 3.0, 49.0, 5.0, 0, 0, 0, 0),
        ShipPoint::new(6.0, 0.0, 65.0, 5.0, 0, 0, 0, 0),
        ShipPoint::new(-13.0, 3.0, 49.0, 5.0, 0, 0, 0, 0),
        ShipPoint::new(-6.0, 0.0, 65.0, 5.0, 0, 0, 0, 0),
    ];

    let moray_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 0, 7, 0, 1),
        ShipLine::new(31, 1, 6, 1, 3),
        ShipLine::new(24, 3, 6, 3, 6),
        ShipLine::new(24, 4, 7, 5, 6),
        ShipLine::new(24, 5, 8, 4, 5),
        ShipLine::new(31, 2, 8, 0, 4),
        ShipLine::new(15, 6, 7, 1, 6),
        ShipLine::new(15, 7, 8, 0, 5),
        ShipLine::new(15, 0, 2, 0, 2),
        ShipLine::new(15, 0, 1, 1, 2),
        ShipLine::new(17, 1, 3, 2, 3),
        ShipLine::new(17, 2, 5, 2, 4),
        ShipLine::new(13, 4, 5, 2, 5),
        ShipLine::new(13, 3, 4, 2, 6),
        ShipLine::new(5, 4, 4, 7, 8),
        ShipLine::new(7, 4, 4, 7, 9),
        ShipLine::new(7, 4, 4, 8, 9),
        ShipLine::new(5, 0, 0, 10, 11),
        ShipLine::new(5, 0, 0, 12, 13),
    ];

    let moray_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 0.0, 43.0, 7.0),
        ShipFaceNormal::new(31.0, -10.0, 49.0, 7.0),
        ShipFaceNormal::new(31.0, 10.0, 49.0, 7.0),
        ShipFaceNormal::new(24.0, -59.0, -28.0, -101.0),
        ShipFaceNormal::new(24.0, 0.0, -52.0, -78.0),
        ShipFaceNormal::new(24.0, 59.0, -28.0, -101.0),
        ShipFaceNormal::new(31.0, -72.0, -99.0, 50.0),
        ShipFaceNormal::new(31.0, 0.0, -83.0, 30.0),
        ShipFaceNormal::new(31.0, 72.0, -99.0, 50.0),
    ];

    let moray_data: ShipData = ShipData {
        name: put_into_name("Moray Star Boat"),
        num_points: 14,
        num_lines: 19,
        num_faces: 9,
        max_loot: 1,
        scoop_type: 0,
        size: 900.0,
        front_laser: 0,
        bounty: 50,
        vanish_point: 40,
        energy: 100,
        velocity: 25,
        missiles: 0,
        laser_strength: 8,
        points: moray_point,
        lines: moray_line,
        normals: moray_face_normal,
    };

    let thargoid_point: Vec<ShipPoint> = vec![
        ShipPoint::new(32.0, -48.0, 48.0, 31.0, 4, 0, 8, 8),
        ShipPoint::new(32.0, -68.0, 0.0, 31.0, 1, 0, 4, 4),
        ShipPoint::new(32.0, -48.0, -48.0, 31.0, 2, 1, 4, 4),
        ShipPoint::new(32.0, 0.0, -68.0, 31.0, 3, 2, 4, 4),
        ShipPoint::new(32.0, 48.0, -48.0, 31.0, 4, 3, 5, 5),
        ShipPoint::new(32.0, 68.0, 0.0, 31.0, 5, 4, 6, 6),
        ShipPoint::new(32.0, 48.0, 48.0, 31.0, 6, 4, 7, 7),
        ShipPoint::new(32.0, 0.0, 68.0, 31.0, 7, 4, 8, 8),
        ShipPoint::new(-24.0, -116.0, 116.0, 31.0, 8, 0, 9, 9),
        ShipPoint::new(-24.0, -164.0, 0.0, 31.0, 1, 0, 9, 9),
        ShipPoint::new(-24.0, -116.0, -116.0, 31.0, 2, 1, 9, 9),
        ShipPoint::new(-24.0, 0.0, -164.0, 31.0, 3, 2, 9, 9),
        ShipPoint::new(-24.0, 116.0, -116.0, 31.0, 5, 3, 9, 9),
        ShipPoint::new(-24.0, 164.0, 0.0, 31.0, 6, 5, 9, 9),
        ShipPoint::new(-24.0, 116.0, 116.0, 31.0, 7, 6, 9, 9),
        ShipPoint::new(-24.0, 0.0, 164.0, 31.0, 8, 7, 9, 9),
        ShipPoint::new(-24.0, 64.0, 80.0, 30.0, 9, 9, 9, 9),
        ShipPoint::new(-24.0, 64.0, -80.0, 30.0, 9, 9, 9, 9),
        ShipPoint::new(-24.0, -64.0, -80.0, 30.0, 9, 9, 9, 9),
        ShipPoint::new(-24.0, -64.0, 80.0, 30.0, 9, 9, 9, 9),
    ];

    let thargoid_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 8, 4, 0, 7),
        ShipLine::new(31, 4, 0, 0, 1),
        ShipLine::new(31, 4, 1, 1, 2),
        ShipLine::new(31, 4, 2, 2, 3),
        ShipLine::new(31, 4, 3, 3, 4),
        ShipLine::new(31, 5, 4, 4, 5),
        ShipLine::new(31, 6, 4, 5, 6),
        ShipLine::new(31, 7, 4, 6, 7),
        ShipLine::new(31, 8, 0, 0, 8),
        ShipLine::new(31, 1, 0, 1, 9),
        ShipLine::new(31, 2, 1, 2, 10),
        ShipLine::new(31, 3, 2, 3, 11),
        ShipLine::new(31, 5, 3, 4, 12),
        ShipLine::new(31, 6, 5, 5, 13),
        ShipLine::new(31, 7, 6, 6, 14),
        ShipLine::new(31, 8, 7, 7, 15),
        ShipLine::new(31, 9, 8, 8, 15),
        ShipLine::new(31, 9, 0, 8, 9),
        ShipLine::new(31, 9, 1, 9, 10),
        ShipLine::new(31, 9, 2, 10, 11),
        ShipLine::new(31, 9, 3, 11, 12),
        ShipLine::new(31, 9, 5, 12, 13),
        ShipLine::new(31, 9, 6, 13, 14),
        ShipLine::new(31, 9, 7, 14, 15),
        ShipLine::new(30, 9, 9, 16, 17),
        ShipLine::new(30, 9, 9, 18, 19),
    ];

    let thargoid_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 103.0, -60.0, 25.0),
        ShipFaceNormal::new(31.0, 103.0, -60.0, -25.0),
        ShipFaceNormal::new(31.0, 103.0, -25.0, -60.0),
        ShipFaceNormal::new(31.0, 103.0, 25.0, -60.0),
        ShipFaceNormal::new(31.0, 64.0, 0.0, 0.0),
        ShipFaceNormal::new(31.0, 103.0, 60.0, -25.0),
        ShipFaceNormal::new(31.0, 103.0, 60.0, 25.0),
        ShipFaceNormal::new(31.0, 103.0, 25.0, 60.0),
        ShipFaceNormal::new(31.0, 103.0, -25.0, 60.0),
        ShipFaceNormal::new(31.0, -48.0, 0.0, 0.0),
    ];

    let thargoid_data: ShipData = ShipData {
        name: put_into_name("Thargoid"),
        num_points: 20,
        num_lines: 26,
        num_faces: 10,
        max_loot: 0,
        scoop_type: 0,
        size: 9801.0,
        front_laser: 15,
        bounty: 500,
        vanish_point: 55,
        energy: 240,
        velocity: 39,
        missiles: 6,
        laser_strength: 11,
        points: thargoid_point,
        lines: thargoid_line,
        normals: thargoid_face_normal,
    };

    let thargon_point: Vec<ShipPoint> = vec![
        ShipPoint::new(-9.0, 0.0, 40.0, 31.0, 0, 1, 5, 5),
        ShipPoint::new(-9.0, -38.0, 12.0, 31.0, 0, 1, 2, 2),
        ShipPoint::new(-9.0, -24.0, -32.0, 31.0, 0, 2, 3, 3),
        ShipPoint::new(-9.0, 24.0, -32.0, 31.0, 0, 3, 4, 4),
        ShipPoint::new(-9.0, 38.0, 12.0, 31.0, 0, 4, 5, 5),
        ShipPoint::new(9.0, 0.0, -8.0, 31.0, 1, 5, 6, 6),
        ShipPoint::new(9.0, -10.0, -15.0, 31.0, 1, 2, 6, 6),
        ShipPoint::new(9.0, -6.0, -26.0, 31.0, 2, 3, 6, 6),
        ShipPoint::new(9.0, 6.0, -26.0, 31.0, 3, 4, 6, 6),
        ShipPoint::new(9.0, 10.0, -15.0, 31.0, 4, 5, 6, 6),
    ];

    let thargon_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 1, 0, 0, 1),
        ShipLine::new(31, 2, 0, 1, 2),
        ShipLine::new(31, 3, 0, 2, 3),
        ShipLine::new(31, 4, 0, 3, 4),
        ShipLine::new(31, 5, 0, 0, 4),
        ShipLine::new(31, 5, 1, 0, 5),
        ShipLine::new(31, 2, 1, 1, 6),
        ShipLine::new(31, 3, 2, 2, 7),
        ShipLine::new(31, 4, 3, 3, 8),
        ShipLine::new(31, 5, 4, 4, 9),
        ShipLine::new(31, 6, 1, 5, 6),
        ShipLine::new(31, 6, 2, 6, 7),
        ShipLine::new(31, 6, 3, 7, 8),
        ShipLine::new(31, 6, 4, 8, 9),
        ShipLine::new(31, 6, 5, 9, 5),
    ];

    let thargon_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, -36.0, 0.0, 0.0),
        ShipFaceNormal::new(31.0, 20.0, -5.0, 7.0),
        ShipFaceNormal::new(31.0, 46.0, -42.0, -14.0),
        ShipFaceNormal::new(31.0, 36.0, 0.0, -104.0),
        ShipFaceNormal::new(31.0, 46.0, 42.0, -14.0),
        ShipFaceNormal::new(31.0, 20.0, 5.0, 7.0),
        ShipFaceNormal::new(31.0, 36.0, 0.0, 0.0),
    ];

    let thargon_data: ShipData = ShipData {
        name: put_into_name("Thargon"),
        num_points: 10,
        num_lines: 15,
        num_faces: 7,
        max_loot: 0,
        scoop_type: 15,
        size: 1600.0,
        front_laser: 0,
        bounty: 50,
        vanish_point: 20,
        energy: 20,
        velocity: 30,
        missiles: 0,
        laser_strength: 8,
        points: thargon_point,
        lines: thargon_line,
        normals: thargon_face_normal,
    };

    let constrct_point: Vec<ShipPoint> = vec![
        ShipPoint::new(20.0, -7.0, 80.0, 31.0, 0, 2, 9, 9),
        ShipPoint::new(-20.0, -7.0, 80.0, 31.0, 0, 1, 9, 9),
        ShipPoint::new(-54.0, -7.0, 40.0, 31.0, 1, 4, 9, 9),
        ShipPoint::new(-54.0, -7.0, -40.0, 31.0, 4, 5, 8, 9),
        ShipPoint::new(-20.0, 13.0, -40.0, 31.0, 5, 6, 8, 8),
        ShipPoint::new(20.0, 13.0, -40.0, 31.0, 6, 7, 8, 8),
        ShipPoint::new(54.0, -7.0, -40.0, 31.0, 3, 7, 8, 9),
        ShipPoint::new(54.0, -7.0, 40.0, 31.0, 2, 3, 9, 9),
        ShipPoint::new(20.0, 13.0, 5.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(-20.0, 13.0, 5.0, 31.0, 15, 15, 15, 15),
        ShipPoint::new(20.0, -7.0, 62.0, 18.0, 9, 9, 9, 9),
        ShipPoint::new(-20.0, -7.0, 62.0, 18.0, 9, 9, 9, 9),
        ShipPoint::new(25.0, -7.0, -25.0, 18.0, 9, 9, 9, 9),
        ShipPoint::new(-25.0, -7.0, -25.0, 18.0, 9, 9, 9, 9),
        ShipPoint::new(15.0, -7.0, -15.0, 10.0, 9, 9, 9, 9),
        ShipPoint::new(-15.0, -7.0, -15.0, 10.0, 9, 9, 9, 9),
        ShipPoint::new(0.0, -7.0, 0.0, 0.0, 9, 15, 0, 1),
    ];

    let constrct_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 0, 9, 0, 1),
        ShipLine::new(31, 1, 9, 1, 2),
        ShipLine::new(31, 0, 1, 1, 9),
        ShipLine::new(31, 0, 2, 0, 8),
        ShipLine::new(31, 2, 9, 0, 7),
        ShipLine::new(31, 2, 3, 7, 8),
        ShipLine::new(31, 1, 4, 2, 9),
        ShipLine::new(31, 4, 9, 2, 3),
        ShipLine::new(31, 3, 9, 6, 7),
        ShipLine::new(31, 3, 7, 6, 8),
        ShipLine::new(31, 6, 7, 5, 8),
        ShipLine::new(31, 5, 6, 4, 9),
        ShipLine::new(31, 4, 5, 3, 9),
        ShipLine::new(31, 5, 8, 3, 4),
        ShipLine::new(31, 6, 8, 4, 5),
        ShipLine::new(31, 7, 8, 5, 6),
        ShipLine::new(31, 8, 9, 3, 6),
        ShipLine::new(31, 0, 6, 8, 9),
        ShipLine::new(18, 9, 9, 10, 12),
        ShipLine::new(5, 9, 9, 12, 14),
        ShipLine::new(10, 9, 9, 14, 10),
        ShipLine::new(10, 9, 9, 11, 15),
        ShipLine::new(5, 9, 9, 13, 15),
        ShipLine::new(18, 9, 9, 11, 13),
    ];

    let constrct_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 0.0, 55.0, 15.0),
        ShipFaceNormal::new(31.0, -24.0, 75.0, 20.0),
        ShipFaceNormal::new(31.0, 24.0, 75.0, 20.0),
        ShipFaceNormal::new(31.0, 44.0, 75.0, 0.0),
        ShipFaceNormal::new(31.0, -44.0, 75.0, 0.0),
        ShipFaceNormal::new(31.0, -44.0, 75.0, 0.0),
        ShipFaceNormal::new(31.0, 0.0, 53.0, 0.0),
        ShipFaceNormal::new(31.0, 44.0, 75.0, 0.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -160.0),
        ShipFaceNormal::new(31.0, 0.0, -27.0, 0.0),
    ];

    let constrct_data: ShipData = ShipData {
        name: put_into_name("Constrictor"),
        num_points: 17,
        num_lines: 24,
        num_faces: 10,
        max_loot: 3,
        scoop_type: 0,
        size: 4225.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 45,
        energy: 252,
        velocity: 36,
        missiles: 4,
        laser_strength: 26,
        points: constrct_point,
        lines: constrct_line,
        normals: constrct_face_normal,
    };

    let cougar_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, 5.0, 67.0, 31.0, 0, 2, 4, 4),
        ShipPoint::new(-20.0, 0.0, 40.0, 31.0, 0, 1, 2, 2),
        ShipPoint::new(-40.0, 0.0, -40.0, 31.0, 0, 1, 5, 5),
        ShipPoint::new(0.0, 14.0, -40.0, 30.0, 0, 4, 5, 5),
        ShipPoint::new(0.0, -14.0, -40.0, 30.0, 1, 2, 3, 5),
        ShipPoint::new(20.0, 0.0, 40.0, 31.0, 2, 3, 4, 4),
        ShipPoint::new(40.0, 0.0, -40.0, 31.0, 3, 4, 5, 5),
        ShipPoint::new(-36.0, 0.0, 56.0, 31.0, 0, 1, 1, 1),
        ShipPoint::new(-60.0, 0.0, -20.0, 31.0, 0, 1, 1, 1),
        ShipPoint::new(36.0, 0.0, 56.0, 31.0, 3, 4, 4, 4),
        ShipPoint::new(60.0, 0.0, -20.0, 31.0, 3, 4, 4, 4),
        ShipPoint::new(0.0, 7.0, 35.0, 18.0, 0, 0, 4, 4),
        ShipPoint::new(0.0, 8.0, 25.0, 20.0, 0, 0, 4, 4),
        ShipPoint::new(-12.0, 2.0, 45.0, 20.0, 0, 0, 0, 0),
        ShipPoint::new(12.0, 2.0, 45.0, 20.0, 4, 4, 4, 4),
        ShipPoint::new(-10.0, 6.0, -40.0, 20.0, 5, 5, 5, 5),
        ShipPoint::new(-10.0, -6.0, -40.0, 20.0, 5, 5, 5, 5),
        ShipPoint::new(10.0, -6.0, -40.0, 20.0, 5, 5, 5, 5),
        ShipPoint::new(10.0, 6.0, -40.0, 20.0, 5, 5, 5, 5),
    ];

    let cougar_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 0, 2, 0, 1),
        ShipLine::new(31, 0, 1, 1, 7),
        ShipLine::new(31, 0, 1, 7, 8),
        ShipLine::new(31, 0, 1, 8, 2),
        ShipLine::new(30, 0, 5, 2, 3),
        ShipLine::new(30, 4, 5, 3, 6),
        ShipLine::new(30, 1, 5, 2, 4),
        ShipLine::new(30, 3, 5, 4, 6),
        ShipLine::new(31, 3, 4, 6, 10),
        ShipLine::new(31, 3, 4, 10, 9),
        ShipLine::new(31, 3, 4, 9, 5),
        ShipLine::new(31, 2, 4, 5, 0),
        ShipLine::new(27, 0, 4, 0, 3),
        ShipLine::new(27, 1, 2, 1, 4),
        ShipLine::new(27, 2, 3, 5, 4),
        ShipLine::new(26, 0, 1, 1, 2),
        ShipLine::new(26, 3, 4, 5, 6),
        ShipLine::new(20, 0, 0, 12, 13),
        ShipLine::new(18, 0, 0, 13, 11),
        ShipLine::new(18, 4, 4, 11, 14),
        ShipLine::new(20, 4, 4, 14, 12),
        ShipLine::new(18, 5, 5, 15, 16),
        ShipLine::new(20, 5, 5, 16, 18),
        ShipLine::new(18, 5, 5, 18, 17),
        ShipLine::new(20, 5, 5, 17, 15),
    ];

    let cougar_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, -16.0, 46.0, 4.0),
        ShipFaceNormal::new(31.0, -16.0, -46.0, 4.0),
        ShipFaceNormal::new(31.0, 0.0, -27.0, 5.0),
        ShipFaceNormal::new(31.0, 16.0, -46.0, 4.0),
        ShipFaceNormal::new(31.0, 16.0, 46.0, 4.0),
        ShipFaceNormal::new(30.0, 0.0, 0.0, -160.0),
    ];

    let cougar_data: ShipData = ShipData {
        name: put_into_name("Cougar"),
        num_points: 19,
        num_lines: 25,
        num_faces: 6,
        max_loot: 3,
        scoop_type: 0,
        size: 4900.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 34,
        energy: 252,
        velocity: 40,
        missiles: 4,
        laser_strength: 26,
        points: cougar_point,
        lines: cougar_line,
        normals: cougar_face_normal,
    };

    let dodec_point: Vec<ShipPoint> = vec![
        ShipPoint::new(0.0, 150.0, 196.0, 31.0, 0, 1, 5, 5),
        ShipPoint::new(143.0, 46.0, 196.0, 31.0, 0, 1, 2, 2),
        ShipPoint::new(88.0, -121.0, 196.0, 31.0, 0, 2, 3, 3),
        ShipPoint::new(-88.0, -121.0, 196.0, 31.0, 0, 3, 4, 4),
        ShipPoint::new(-143.0, 46.0, 196.0, 31.0, 0, 4, 5, 5),
        ShipPoint::new(0.0, 243.0, 46.0, 31.0, 1, 5, 6, 6),
        ShipPoint::new(231.0, 75.0, 46.0, 31.0, 1, 2, 7, 7),
        ShipPoint::new(143.0, -196.0, 46.0, 31.0, 2, 3, 8, 8),
        ShipPoint::new(-143.0, -196.0, 46.0, 31.0, 3, 4, 9, 9),
        ShipPoint::new(-231.0, 75.0, 46.0, 31.0, 4, 5, 10, 10),
        ShipPoint::new(143.0, 196.0, -46.0, 31.0, 1, 6, 7, 7),
        ShipPoint::new(231.0, -75.0, -46.0, 31.0, 2, 7, 8, 8),
        ShipPoint::new(0.0, -243.0, -46.0, 31.0, 3, 8, 9, 9),
        ShipPoint::new(-231.0, -75.0, -46.0, 31.0, 4, 9, 10, 10),
        ShipPoint::new(-143.0, 196.0, -46.0, 31.0, 5, 6, 10, 10),
        ShipPoint::new(88.0, 121.0, -196.0, 31.0, 6, 7, 11, 11),
        ShipPoint::new(143.0, -46.0, -196.0, 31.0, 7, 8, 11, 11),
        ShipPoint::new(0.0, -150.0, -196.0, 31.0, 8, 9, 11, 11),
        ShipPoint::new(-143.0, -46.0, -196.0, 31.0, 9, 10, 11, 11),
        ShipPoint::new(-88.0, 121.0, -196.0, 31.0, 6, 10, 11, 11),
        ShipPoint::new(-16.0, 32.0, 196.0, 30.0, 0, 0, 0, 0),
        ShipPoint::new(-16.0, -32.0, 196.0, 30.0, 0, 0, 0, 0),
        ShipPoint::new(16.0, 32.0, 196.0, 23.0, 0, 0, 0, 0),
        ShipPoint::new(16.0, -32.0, 196.0, 23.0, 0, 0, 0, 0),
    ];

    let dodec_line: Vec<ShipLine> = vec![
        ShipLine::new(31, 0, 1, 0, 1),
        ShipLine::new(31, 0, 2, 1, 2),
        ShipLine::new(31, 0, 3, 2, 3),
        ShipLine::new(31, 0, 4, 3, 4),
        ShipLine::new(31, 0, 5, 4, 0),
        ShipLine::new(31, 1, 6, 5, 10),
        ShipLine::new(31, 1, 7, 10, 6),
        ShipLine::new(31, 2, 7, 6, 11),
        ShipLine::new(31, 2, 8, 11, 7),
        ShipLine::new(31, 3, 8, 7, 12),
        ShipLine::new(31, 3, 9, 12, 8),
        ShipLine::new(31, 4, 9, 8, 13),
        ShipLine::new(31, 4, 10, 13, 9),
        ShipLine::new(31, 5, 10, 9, 14),
        ShipLine::new(31, 5, 6, 14, 5),
        ShipLine::new(31, 7, 11, 15, 16),
        ShipLine::new(31, 8, 11, 16, 17),
        ShipLine::new(31, 9, 11, 17, 18),
        ShipLine::new(31, 10, 11, 18, 19),
        ShipLine::new(31, 6, 11, 19, 15),
        ShipLine::new(31, 1, 5, 0, 5),
        ShipLine::new(31, 1, 2, 1, 6),
        ShipLine::new(31, 2, 3, 2, 7),
        ShipLine::new(31, 3, 4, 3, 8),
        ShipLine::new(31, 4, 5, 4, 9),
        ShipLine::new(31, 6, 7, 10, 15),
        ShipLine::new(31, 7, 8, 11, 16),
        ShipLine::new(31, 8, 9, 12, 17),
        ShipLine::new(31, 9, 10, 13, 18),
        ShipLine::new(31, 6, 10, 14, 19),
        ShipLine::new(30, 0, 0, 20, 21),
        ShipLine::new(20, 0, 0, 21, 23),
        ShipLine::new(23, 0, 0, 23, 22),
        ShipLine::new(20, 0, 0, 22, 20),
    ];

    let dodec_face_normal: Vec<ShipFaceNormal> = vec![
        ShipFaceNormal::new(31.0, 0.0, 0.0, 196.0),
        ShipFaceNormal::new(31.0, 103.0, 142.0, 88.0),
        ShipFaceNormal::new(31.0, 169.0, -55.0, 89.0),
        ShipFaceNormal::new(31.0, 0.0, -176.0, 88.0),
        ShipFaceNormal::new(31.0, -169.0, -55.0, 89.0),
        ShipFaceNormal::new(31.0, -103.0, 142.0, 88.0),
        ShipFaceNormal::new(31.0, 0.0, 176.0, -88.0),
        ShipFaceNormal::new(31.0, 169.0, 55.0, -89.0),
        ShipFaceNormal::new(31.0, 103.0, -142.0, -88.0),
        ShipFaceNormal::new(31.0, -103.0, -142.0, -88.0),
        ShipFaceNormal::new(31.0, -169.0, 55.0, -89.0),
        ShipFaceNormal::new(31.0, 0.0, 0.0, -196.0),
    ];

    let dodec_data: ShipData = ShipData {
        name: put_into_name("Dodec Space Station"),
        num_points: 24,
        num_lines: 34,
        num_faces: 12,
        max_loot: 0,
        scoop_type: 0,
        size: 32400.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 125,
        energy: 240,
        velocity: 0,
        missiles: 0,
        laser_strength: 0,
        points: dodec_point,
        lines: dodec_line,
        normals: dodec_face_normal,
    };
    let null_data: ShipData = ShipData {
        name: put_into_name("NULL"),
        num_points: 0,
        num_lines: 0,
        num_faces: 0,
        max_loot: 0,
        scoop_type: 0,
        size: 1.0,
        front_laser: 0,
        bounty: 0,
        vanish_point: 125,
        energy: 0,
        velocity: 0,
        missiles: 0,
        laser_strength: 0,
        points: vec![],
        lines: vec![],
        normals: vec![],
    };

    let mut ship_list: [ShipData; NO_OF_SHIPS + 1] = [
        null_data,
        missile_data,
        coriolis_data,
        esccaps_data,
        alloy_data,
        cargo_data,
        boulder_data,
        asteroid_data,
        rock_data,
        orbit_data,
        transp_data,
        cobra3a_data,
        pythona_data,
        boa_data,
        anacnda_data,
        hermit_data,
        viper_data,
        sidewnd_data,
        mamba_data,
        krait_data,
        adder_data,
        gecko_data,
        cobra1_data,
        worm_data,
        cobra3b_data,
        asp2_data,
        pythonb_data,
        ferdlce_data,
        moray_data,
        thargoid_data,
        thargon_data,
        constrct_data,
        cougar_data,
        dodec_data,
    ];

    let mut universe: Vec<UnivObject> = vec![];
    for _ in 0..MAX_UNIV_OBJECTS {
        universe.push(UnivObject::new());
    }
    let frame_duration = time::Duration::from_millis(60);
    let debug_duration = time::Duration::from_millis(60);
    let config: Config = Config::new();
    let mut cmdr = Commander::get_saved();
    let mut params: GameParams = GameParams::new();
    let mut da_stars: Stars = Stars::new();
    // let overlay: Texture2D = load_texture("assets/scanner.png").await.unwrap();
    let mut message_width;
    let mut message_x_pos;
    let mut text_params = TextParams {
        font: Some(&font),
        font_size: 12,
        font_scale: GFX_SCALE,
        font_scale_aspect: 1.0,
        rotation: 0.0,
        color: WHITE,
    };

    while !params.finish {
        params.game_over = false;
        initialise_game(
            &mut params,
            &mut da_stars,
            &mut universe,
            &mut ship_count,
            &mut cmdr,
        );
        dock_player(&mut params);

        update_console(&params, &ship_list, &ship_count, &universe, &cmdr, &labels);

        params.current_screen = SCR_FRONT_VIEW;
        run_first_intro_screen(
            &mut universe,
            &mut ship_list,
            &mut params,
            &mut cmdr,
            &mut ship_count,
            &config,
            &text_params,
            &font,
            &sample_list,
        );
        audio::play_sound_once(&sample_list[SND_ELITE_THEME]);
        loop {
            params.update_screen_params(); // my macroquad admin stuff
            update_intro1(
                &mut universe,
                &mut params,
                &mut cmdr,
                &mut ship_list,
                &mut ship_count,
                &config,
                &text_params,
                &font,
                &sample_list
            );
            update_console(&params, &ship_list, &ship_count, &universe, &cmdr, &labels);
            if is_key_down(KeyCode::Y) {
                // snd_stop_midi();
                // load_commander_screen();
                break;
            }
            if is_key_down(KeyCode::N) {
                // snd_stop_midi();
                break;
            }
            thread::sleep(frame_duration);
            next_frame().await;
        }
        audio::stop_sound(&sample_list[SND_ELITE_THEME]);
        run_second_intro_screen(
            &mut da_stars,
            &mut params,
            &mut universe,
            &mut ship_count,
            &mut ship_list,
            &config,
            &mut cmdr,
        );
        audio::play_sound_once(&sample_list[SND_BLUE_DANUBE]);
        loop {
            params.update_screen_params();
            update_intro2(
                &mut universe,
                &mut da_stars,
                &mut params,
                &mut ship_count,
                &mut ship_list,
                &mut cmdr,
                &config,
                &text_params,
                &font,
                &sample_list
            );
            update_console(&params, &ship_list, &ship_count, &universe, &cmdr, &labels);
            if is_key_down(KeyCode::Space) {
                break;
            }
            thread::sleep(frame_duration);
            next_frame().await;
        }
        audio::stop_sound(&sample_list[SND_BLUE_DANUBE]);
        params.old_cross_x = -1;
        params.old_cross_y = -1;

        dock_player(&mut params);
        params.current_screen = SCR_CMDR_STATUS;
        while !params.game_over {
            params.update_screen_params(); // my macroquad admin stuff
            update_console(&params, &ship_list, &ship_count, &universe, &cmdr, &labels);
            params.rolling = false;
            params.climbing = false;

            handle_flight_keys(
                &mut params,
                &config,
                &mut cmdr,
                &mut da_stars,
                &mut universe,
                &mut ship_count,
                &mut ship_list,
                &sample_list,
                &font,
                &mut text_params,
            );
            display_screens(
                &mut params,
                &config,
                &mut cmdr,
                &mut da_stars,
                &mut universe,
                &mut ship_count,
                &mut ship_list,
                &sample_list,
                &font,
                &mut text_params,
            );
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
            if params.current_screen == SCR_CMDR_STATUS {
                display_commander_status(&cmdr, &mut params, &universe, &font, &mut text_params);
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
                    update_starfield(&mut da_stars, &params);
                }

                if params.auto_pilot {
                    auto_dock(&mut params, &ship_count, &mut universe);
                    if (params.mcount & 127) == 0 {
                        info_message("Docking Computers On".to_string(), &mut params,&sample_list);
                    }
                }

                update_universe(
                    &mut universe,
                    &mut cmdr,
                    &mut ship_list,
                    &mut params,
                    &mut ship_count,
                    &config,
                    &sample_list,
                );

                // if params.docked {
                //     update_console(&params, &ship_list, &ship_count, &universe, &cmdr, &labels);
                //     continue;
                // }

                if (params.current_screen == SCR_FRONT_VIEW)
                    || (params.current_screen == SCR_REAR_VIEW)
                    || (params.current_screen == SCR_LEFT_VIEW)
                    || (params.current_screen == SCR_RIGHT_VIEW)
                {
                    if params.draw_lasers != 0 {
                        draw_laser_lines(&params, &config);
                        params.draw_lasers -= 1;
                    }

                    draw_laser_sights(&params, &cmdr, &font, &text_params);
                }
                if params.message_count > 0 {
                    message_width =
                        measure_text(&params.message_string, Some(&font), 18, GFX_SCALE).width;
                    message_x_pos = (params.screen_width - message_width) * 0.5;
                    draw_text_ex(
                        &params.message_string,
                        message_x_pos,
                        params.screen_height * 0.6,
                        text_params.clone(),
                    );
                }

                if params.hyper_ready {
                    display_hyper_status(&mut params, &text_params, &font);
                    if (params.mcount & 3) == 0 {
                        countdown_hyperspace(&mut params,&mut cmdr,&mut da_stars, &mut universe,&mut ship_count,&mut ship_list, &sample_list);
                    }
                }

                if params.mcount == 0 {
                    params.mcount = 255;
                } else {
                    params.mcount -= 1;
                }

                if (params.mcount & 7) == 0 {
                    regenerate_shields(&mut params, &cmdr);
                }

                if (params.mcount & 31) == 10 {
                    if params.energy < 50 {
                        info_message("ENERGY LOW".to_string(), &mut params,&sample_list);
                        snd_play_sample(&sample_list,SND_BEEP);
                    }
                    update_altitude(&mut params, &universe);
                }

                if (params.mcount & 31) == 20 {
                    update_cabin_temp(&mut params, &universe, &mut ship_count, &mut cmdr,&sample_list);
                }

                if (params.mcount == 0) && (!params.witchspace) {
                    random_encounter(
                        &mut ship_count,
                        &mut universe,
                        &mut params,
                        &cmdr,
                        &mut ship_list,
                    );
                }

                cool_laser(&mut params);
                // crst
                // time_ecm();

                update_console(&params, &ship_list, &ship_count, &universe, &cmdr, &labels);
            } else {
                audio::stop_sound(&sample_list[SND_BLUE_DANUBE]);
            }

            if params.current_screen == SCR_BREAK_PATTERN {
                for i in 1..12 {
                    for j in 0..i {
                        draw_circle_lines(
                            0.5 * params.screen_width * GFX_SCALE,
                            0.5 * params.screen_height * GFX_SCALE,
                            ((70.0 + j as f32 * 35.0) * GFX_SCALE),
                            THICKNESS * GFX_SCALE,
                            WHITE,
                        );
                    }
                    thread::sleep(frame_duration);
                    next_frame().await;
                }

                if params.docked {
                    // crst
                    // check_mission_brief();
                    display_commander_status(
                        &cmdr,
                        &mut params,
                        &universe,
                        &font,
                        &mut text_params,
                    );
                    // update_console(&params, &ship_list, &ship_count, &universe, &cmdr);
                } else {
                    params.current_screen = SCR_FRONT_VIEW;
                }
                // display_break_pattern(frame_duration,&mut params,&universe,&cmdr);
            }

            if params.cross_timer > 0 {
                params.cross_timer -= 1;
                if params.cross_timer == 0 {
                    show_distance_to_planet(&mut params, &text_params,&font, &mut cmdr);
                }
            }
            show_distance_to_planet(&mut params, &text_params,&font, &mut cmdr);

            // xyz
            // if (params.cross_x != params.old_cross_x) || (params.cross_y != params.old_cross_y) {
            //     if params.old_cross_x != -1 {
            //         draw_cross(&params, params.old_cross_x, params.old_cross_y);
            //     }

            //     params.old_cross_x = params.cross_x;
            //     params.old_cross_y = params.cross_y;

            //     draw_cross(&params, params.old_cross_x, params.old_cross_y);
            // }
            thread::sleep(debug_duration);
            next_frame().await
        }

        if !params.finish {
            // run_game_over_screen();
        }
        thread::sleep(frame_duration);
        next_frame().await
    }
}

fn put_into_name(new_name: &str) -> [char; 32] {
    let mut result = [' '; 32];
    for (i, c) in new_name.chars().enumerate() {
        if i < result.len() {
            result[i] = c;
        }
    }
    result
}
/*
 * Initialise the game parameters.
 */

fn initialise_game(
    params: &mut GameParams,
    da_stars: &mut Stars,
    universe: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    cmdr: &mut Commander,
) {
    params.current_screen = SCR_INTRO_ONE;

    restore_saved_commander(cmdr, params);

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

    create_new_stars(da_stars, params);
    clear_universe(universe, ship_count, &mut params.in_battle);

    params.old_cross_x = -1;
    params.old_cross_y = -1;
    params.cross_x = -1;
    params.cross_y = -1;
    params.cross_timer = 0;

    params.myship.max_speed = 40; /* 0.27 Light Mach */
    params.myship.max_roll = 31;
    params.myship.max_climb = 8; /* CF 8 */
    params.myship.max_fuel = 70; /* 7.0 Light Years */
    params.message_string = "".to_string();
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

fn move_cross(params: &mut GameParams, dx: My, dy: My) {
    params.cross_timer = 5;
    if params.current_screen == SCR_SHORT_RANGE {
        params.cross_x += dx * 4;
        params.cross_y += dy * 4;
        return;
    }
    // xyz
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

fn draw_cross(params: &GameParams, cx: My, cy: My) {
    if params.current_screen == SCR_SHORT_RANGE {
        // crst
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
        // crst
        // xor_mode(FALSE);
        // gfx_set_clip_region(1, 1, 510, 383);
        return;
    }
    if params.current_screen == SCR_GALACTIC_CHART {
        // crst
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
        // crst
        // xor_mode(FALSE);
        // gfx_set_clip_region(1, 1, 510, 383);
    }
}

fn draw_laser_sights(params: &GameParams, cmdr: &Commander, font: &Font, text_params: &TextParams) {
    let mut laser: My = 0;
    let mut x1: f32;
    let mut y1: f32;
    let mut x2: f32;
    let mut y2: f32;
    let mut msg = "".to_string();

    match params.current_screen {
        SCR_FRONT_VIEW => {
            msg = "Front View".to_string();
            laser = cmdr.front_laser;
        }
        SCR_REAR_VIEW => {
            msg = "Rear View".to_string();
            laser = cmdr.rear_laser;
        }
        SCR_LEFT_VIEW => {
            msg = "Left View".to_string();
            laser = cmdr.left_laser;
        }
        SCR_RIGHT_VIEW => {
            msg = "Right View".to_string();
            laser = cmdr.right_laser;
        }
        _ => (),
    }

    let msg_width = measure_text(&msg, Some(font), 18, GFX_SCALE).width;
    let msg_x_pos = (params.screen_width - msg_width) * 0.5;
    draw_text_ex(
        &msg,
        msg_x_pos,
        params.screen_height * 0.1,
        text_params.clone(),
    );

    let seg_length = params.screen_height / 10.0 * (1.0 - SCANNER_Y_PROPORTION);
    if laser != 0 {
        x1 = params.screen_width * 0.5;
        y1 = params.row_y_pos * 0.5 - 1.5 * seg_length;
        y2 = params.row_y_pos * 0.5 - 0.5 * seg_length;
        draw_line(x1, y1, x1, y2, THICKNESS, WHITE);
        y1 = params.row_y_pos * 0.5 + 1.5 * seg_length;
        y2 = params.row_y_pos * 0.5 + 0.5 * seg_length;
        draw_line(x1, y1, x1, y2, THICKNESS, WHITE);
        y1 = params.row_y_pos * 0.5;
        x1 = params.screen_width * 0.5 - 1.5 * seg_length;
        x2 = params.screen_width * 0.5 - 0.5 * seg_length;
        draw_line(x1, y1, x2, y1, THICKNESS, WHITE);
        x1 = params.screen_width * 0.5 + 0.5 * seg_length;
        x2 = params.screen_width * 0.5 + 1.5 * seg_length;
        draw_line(x1, y1, x2, y1, THICKNESS, WHITE);
    }
}

fn arrow_right(params: &mut GameParams) {
    match params.current_screen {
        SCR_MARKET_PRICES => {
            // crst
            // buy_stock();
        }

        SCR_SETTINGS => {
            // crst
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
            // crst
            // sell_stock();
        }

        SCR_SETTINGS => {
            // crst
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
            // crst
            // select_previous_stock();
        }

        SCR_EQUIP_SHIP => {
            // crst
            // select_previous_equip();
        }

        SCR_OPTIONS => {
            // crst
            // select_previous_option();
        }

        SCR_SETTINGS => {
            // crst
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
    if params.current_screen == SCR_MARKET_PRICES {
        // crst
        // select_next_stock();
    } else if params.current_screen == SCR_EQUIP_SHIP {
        // crst
        // select_next_equip();
    } else if params.current_screen == SCR_OPTIONS {
        // crst
        // select_next_option();
    } else if params.current_screen == SCR_SETTINGS {
        // crst
        // select_down_setting();
    } else if params.current_screen == SCR_SHORT_RANGE
        || params.current_screen == SCR_GALACTIC_CHART
    {
        // xyz
        move_cross(params, 0, 1);
    } else if params.current_screen == SCR_FRONT_VIEW
        || params.current_screen == SCR_REAR_VIEW
        || params.current_screen == SCR_RIGHT_VIEW
        || params.current_screen == SCR_LEFT_VIEW
    {
        if params.flight_climb < 0 {
            params.flight_climb = 0;
        } else {
            params.increase_flight_climb();
        }
        params.climbing = true;
    }
}

fn display_screens(
    params: &mut GameParams,
    config: &Config,
    cmdr: &mut Commander,
    da_stars: &mut Stars,
    universe: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
    sample_list: &[Sound],
    font: &Font,
    text_params: &mut TextParams,
) {
    if params.current_screen == SCR_EQUIP_SHIP {
        if params.docked {
            // crst
            // equip_ship();
        }
    } else if params.current_screen == SCR_GALACTIC_CHART {
        // xyz move_cross?
        display_galactic_chart(params, text_params, font, cmdr);
        draw_cross(&params, params.cross_x, params.cross_y);
    } else if params.current_screen == SCR_SHORT_RANGE {
        // xyz move_cross?
        display_short_range_chart(params, cmdr, text_params, font);
        draw_cross(&params, params.cross_x, params.cross_y);
    } else if params.current_screen == SCR_PLANET_DATA {
        //crst
        display_data_on_planet(params, text_params, font, cmdr, config);
    } else if params.current_screen == SCR_MARKET_PRICES {
        // crst
        // display_market_prices();
    } else if params.current_screen == SCR_CMDR_STATUS {
        display_commander_status(cmdr, params, universe, font, text_params);
    } else if params.current_screen == SCR_INVENTORY {
        // crst
        // display_inventory();
    } else if params.current_screen == SCR_OPTIONS {
        // crst
        // display_options();
    }
}
fn handle_flight_keys(
    params: &mut GameParams,
    config: &Config,
    cmdr: &mut Commander,
    da_stars: &mut Stars,
    universe: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
    sample_list: &[Sound],
    font: &Font,
    text_params: &mut TextParams,
) {
    let mut keyasc;

    if params.docked
        && ((params.current_screen == SCR_MARKET_PRICES)
            || (params.current_screen == SCR_OPTIONS)
            || (params.current_screen == SCR_SETTINGS)
            || (params.current_screen == SCR_EQUIP_SHIP))
    {
        // crst
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
            launch_player(
                params, cmdr, da_stars, universe, ship_count, ship_list, sample_list,
            );
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
            params.current_screen = SCR_EQUIP_SHIP;
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
        params.current_screen = SCR_GALACTIC_CHART;
        params.hyperspace_planet = params.docked_planet;
        // params.cross_x = (params.hyperspace_planet.d as f32 * GFX_SCALE) as My;
        // params.cross_y =
        //     ((params.hyperspace_planet.b as f32 / (2.0 / GFX_SCALE)) + (18.0 * GFX_SCALE) + 1.0) as My;
    }

    if is_key_down(KeyCode::F6) {
        params.find_input = false;
        params.old_cross_x = -1;
        params.current_screen = SCR_SHORT_RANGE;
        params.hyperspace_planet = params.docked_planet;
        // params.cross_x =
        //     (((params.hyperspace_planet.d as f32 - params.docked_planet.d as f32) * 1.0 * GFX_SCALE)
        //          ) as My;
        // params.cross_y =
        //     (((params.hyperspace_planet.b as f32 - params.docked_planet.b as f32) * 0.5 * GFX_SCALE)
        //         ) as My;
    }

    if is_key_down(KeyCode::F7) {
        params.find_input = false;
        params.current_screen = SCR_PLANET_DATA;
    }

    if is_key_down(KeyCode::F8) && (!params.witchspace) {
        params.find_input = false;
        params.current_screen = SCR_MARKET_PRICES;
    }

    if is_key_down(KeyCode::F9) {
        params.find_input = false;
        params.current_screen = SCR_CMDR_STATUS;
    }

    if is_key_down(KeyCode::F10) {
        params.current_screen = SCR_INVENTORY;
        params.find_input = false;
    }

    if is_key_down(KeyCode::F11) {
        params.find_input = false;
        params.current_screen = SCR_OPTIONS;
    }

    if params.find_input {
        keyasc = kbd_read_key();

        if is_key_down(KeyCode::Enter) {
            params.find_input = false;
            // crst
            // find_planet_by_name (find_name);
            return;
        }

        if is_key_down(KeyCode::Backspace) {
            // crst
            // delete_find_char();
            return;
        }

        if isalpha(keyasc) {
            // crst
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
            params.draw_lasers = fire_laser(params, cmdr, sample_list);
        }
    }

    if is_key_down(DOCK_KEY) {
        if !params.docked && cmdr.docking_computer != 0 {
            if config.instant_dock != 0 {
                engage_docking_computer(params, ship_count, sample_list);
            } else {
                engage_auto_pilot(params, sample_list);
            }
        }
    }

    if is_key_down(KeyCode::D) {
        d_pressed();
    }

    if is_key_down(ECM_KEY) {
        if !params.docked && cmdr.ecm != 0 {
            // crst
            // activate_ecm(1);
        }
    }

    if is_key_down(FIND_KEY) {
        // crst
        // f_pressed ();
    }

    if is_key_down(HYPERSPACE_KEY) && (!params.docked) {
        if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
            start_galactic_hyperspace(params,cmdr);
        } else {
            start_hyperspace(params,cmdr);
        }
    }

    if is_key_down(JUMP_KEY) && (!params.docked) && (!params.witchspace) {
        jump_warp(universe, params,sample_list);
    }

    if is_key_down(FIRE_MISSILE_KEY) {
        if !params.docked {
            fire_missile(universe, params, cmdr, ship_list, ship_count, sample_list);
        }
    }

    if is_key_down(KeyCode::O) {
        o_pressed(params);
    }

    if is_key_down(PAUSE_KEY) {
        params.game_paused = true;
    }

    if is_key_down(TARGET_MISSILE_KEY) {
        if !params.docked {
            arm_missile(cmdr, params);
        }
    }

    if is_key_down(UNARM_MISSILE_KEY) {
        if !params.docked {
            unarm_missile(params, sample_list);
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

    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::S) {
        arrow_down(params);
    }

    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::X) {
        arrow_up(params);
    }

    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::Comma) {
        arrow_left(params);
    }

    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::Period) {
        arrow_right(params);
    }

    if is_key_down(KeyCode::Enter) {
        // crst
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
            // crst
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

pub fn info_message(message: String, params: &mut GameParams,sample_list: &[Sound]) {
    params.message_string = message;
    params.message_count = 37;
	snd_play_sample (sample_list,SND_BEEP);
}
pub fn auto_dock(
    params: &mut GameParams,
    ship_count: &[My; NO_OF_SHIPS + 1],
    universe: &mut [UnivObject],
) {
    let mut ship: UnivObject = UnivObject::new();

    ship.location.x = 0.0;
    ship.location.y = 0.0;
    ship.location.z = 0.0;

    ship.rotmat = START_MATRIX;
    ship.rotmat[2].z = 1.0;
    ship.rotmat[0].x = -1.0;
    // crst
    // ship.da_type = -96;
    // **warning
    ship.da_type = 170;
    ship.velocity = params.flight_speed;
    ship.acceleration = 0;
    ship.bravery = 0;
    ship.rotz = 0;
    ship.rotx = 0;

    auto_pilot_ship(&mut ship, universe, ship_count);

    if (ship.velocity > 22) {
        params.flight_speed = 22;
    } else {
        params.flight_speed = ship.velocity;
    }

    if (ship.acceleration > 0) {
        params.flight_speed += 1;
        if (params.flight_speed > 22) {
            params.flight_speed = 22;
        }
    }

    if (ship.acceleration < 0) {
        params.flight_speed -= 1;
        if (params.flight_speed < 1) {
            params.flight_speed = 1;
        }
    }

    if (ship.rotx == 0) {
        params.flight_climb = 0;
    }

    if (ship.rotx < 0) {
        params.increase_flight_climb();

        if (ship.rotx < -1) {
            params.increase_flight_climb();
        }
    }

    if (ship.rotx > 0) {
        params.decrease_flight_climb();

        if (ship.rotx > 1) {
            params.decrease_flight_climb();
        }
    }

    if (ship.rotz == 127) {
        params.flight_roll = -14;
    } else {
        if (ship.rotz == 0) {
            params.flight_roll = 0;
        }

        if (ship.rotz > 0) {
            params.increase_flight_roll();

            if (ship.rotz > 1) {
                params.increase_flight_roll();
            }
        }

        if (ship.rotz < 0) {
            params.decrease_flight_roll();

            if (ship.rotz < -1) {
                params.decrease_flight_roll();
            }
        }
    }
}
/*
 * Fly a ship to the planet or to the space station and dock it.
 */

pub fn auto_pilot_ship(
    ship: &mut UnivObject,
    universe: &mut [UnivObject],
    ship_count: &[My; NO_OF_SHIPS + 1],
) {
    let mut diff = START_VECTOR;
    let mut vec = START_VECTOR;

    if ((ship.flags & FLG_FLY_TO_PLANET) != 0
        || ((ship_count[SHIP_CORIOLIS as usize] == 0) && (ship_count[SHIP_DODEC as usize] == 0)))
    {
        fly_to_planet(ship, universe);
        return;
    }

    diff.x = ship.location.x - universe[1].location.x;
    diff.y = ship.location.y - universe[1].location.y;
    diff.z = ship.location.z - universe[1].location.z;

    let dist = (diff.x * diff.x + diff.y * diff.y + diff.z * diff.z).sqrt();

    if (dist < 160.0) {
        ship.flags |= FLG_REMOVE; // ship has docked.
        return;
    }

    vec = unit_vector(&diff);
    let mut dir = vector_dot_product(&universe[1].rotmat[2], &vec);

    if (dir < 0.9722) {
        fly_to_station_front(ship, universe);
        return;
    }

    dir = vector_dot_product(&ship.rotmat[2], &vec);

    if (dir < -0.9444) {
        fly_to_docking_bay(ship, universe);
        return;
    }

    fly_to_station(ship, universe);
}
fn run_first_intro_screen(
    universe: &mut [UnivObject],
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
    params: &mut GameParams,
    cmdr: &mut Commander,
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    config: &Config,
    text_params: &TextParams,
    font: &Font,
    sample_list: &[Sound],
) {
    params.current_screen = SCR_INTRO_ONE;
    snd_play_sample(sample_list,SND_ELITE_THEME);
    initialise_intro1(params, universe, ship_count, ship_list);
}

fn run_second_intro_screen(
    da_stars: &mut Stars,
    params: &mut GameParams,
    universe: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
    config: &Config,
    cmdr: &mut Commander,
) {
    params.current_screen = SCR_INTRO_TWO;
    initialise_intro2(da_stars, params, universe, ship_count, ship_list);
    params.flight_speed = 3;
    params.flight_roll = 0;
    params.flight_climb = 0;
}

fn initialise_intro1(
    params: &mut GameParams,
    universe: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
) {
    let mut intro_ship_matrix = START_MATRIX;
    clear_universe(universe, ship_count, &mut params.in_battle);
    set_init_matrix(&mut intro_ship_matrix);
    add_new_ship(
        SHIP_COBRA3,
        0.0,
        0.0,
        4500.0,
        &mut intro_ship_matrix,
        -127,
        -127,
        universe,
        ship_list,
        ship_count,
    );
}

fn initialise_intro2(
    da_stars: &mut Stars,
    params: &mut GameParams,
    universe: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
) {
    let mut intro_ship_matrix = START_MATRIX;
    params.ship_no = 0;
    params.show_time = 0;
    params.direction = 100.0;

    clear_universe(universe, ship_count, &mut params.in_battle);
    create_new_stars(da_stars, params);
    set_init_matrix(&mut intro_ship_matrix);
    add_new_ship(
        1,
        0.0,
        0.0,
        5000.0,
        &mut intro_ship_matrix,
        -127,
        -127,
        universe,
        ship_list,
        ship_count,
    );
}

fn update_intro1(
    universe: &mut [UnivObject],
    params: &mut GameParams,
    cmdr: &mut Commander,
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    config: &Config,
    text_params: &TextParams,
    font: &Font,
    sample_list:&[Sound]
) {
    universe[0].location.z -= 100.0;

    if (universe[0].location.z < 984.0) {
        universe[0].location.z = 984.0;
    }

    params.flight_roll = 1;
    update_universe(
        universe,
        cmdr,
        ship_list,
        params,
        ship_count,
        config,
        sample_list
    );

    // gfx_draw_sprite(IMG_ELITE_TXT, -1, 10);

    let mut msg = "Original Game (C) I.Bell & D.Braben.";
    let mut msg_width = measure_text(&msg, Some(&font), 12, GFX_SCALE).width;
    let mut msg_x_pos = (params.screen_width - msg_width) * 0.5;
    draw_text_ex(&msg, msg_x_pos, 110.0 * GFX_SCALE, text_params.clone());
    msg = "Re-engineered by C.J.Pinder.";
    msg_width = measure_text(&msg, Some(&font), 12, GFX_SCALE).width;
    msg_x_pos = (params.screen_width - msg_width) * 0.5;
    draw_text_ex(&msg, msg_x_pos, 130.0 * GFX_SCALE, text_params.clone());
    msg = "Load New Commander (Y/N)?";
    msg_width = measure_text(&msg, Some(&font), 12, GFX_SCALE).width;
    msg_x_pos = (params.screen_width - msg_width) * 0.5;
    draw_text_ex(&msg, msg_x_pos, 160.0 * GFX_SCALE, text_params.clone());
}

fn update_intro2(
    universe: &mut [UnivObject],
    da_stars: &mut Stars,
    params: &mut GameParams,
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
    cmdr: &mut Commander,
    config: &Config,
    text_params: &TextParams,
    font: &Font,
    sample_list: &[Sound],
) {
    let mut intro_ship_matrix = START_MATRIX;
    params.show_time += 1;

    if ((params.show_time >= 140) && (params.direction < 0.0)) {
        params.direction = -params.direction;
    }

    universe[0].location.z += params.direction;

    if (universe[0].location.z < MIN_DIST[params.ship_no as usize]) {
        universe[0].location.z = MIN_DIST[params.ship_no as usize];
    }

    if (universe[0].location.z > 4500.0) {
        params.ship_no += 1;
        if (params.ship_no as usize > NO_OF_SHIPS) {
            params.ship_no = 1;
        }

        params.show_time = 0;
        params.direction = -100.0;

        ship_count[universe[0].da_type as usize] = 0;
        universe[0].da_type = 0;

        add_new_ship(
            params.ship_no,
            0.0,
            0.0,
            4500.0,
            &mut intro_ship_matrix,
            -127,
            -127,
            universe,
            ship_list,
            ship_count,
        );
    }

    update_starfield(da_stars, params);
    update_universe(
        universe,
        cmdr,
        ship_list,
        params,
        ship_count,
        config,
        sample_list
    );

    // gfx_draw_sprite (IMG_ELITE_TXT, -1, 10);
    let mut msg = "Press Fire or Space, Commander.";
    let mut msg_width = measure_text(&msg, Some(&font), 12, GFX_SCALE).width;
    let mut msg_x_pos = (params.screen_width - msg_width) * 0.5;
    draw_text_ex(&msg, msg_x_pos, 160.0 * GFX_SCALE, text_params.clone());

    let ship_name = ship_list[params.ship_no as usize].get_name();
    msg = &ship_name;
    msg_width = measure_text(&msg, Some(&font), 12, GFX_SCALE).width;
    msg_x_pos = (params.screen_width - msg_width) * 0.5;
    draw_text_ex(&msg, msg_x_pos, 130.0 * GFX_SCALE, text_params.clone());
}
fn o_pressed(params:&mut GameParams)
{
	if params.current_screen == SCR_GALACTIC_CHART
	|| params.current_screen == SCR_SHORT_RANGE{
		move_cursor_to_origin(params);
	}
}
