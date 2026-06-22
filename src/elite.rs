/*
 * Elite - The New Kind.
 *
 * Reverse engineered from the BBC disk version of Elite.
 * Additional material by C.J.Pinder.
 *
 * The original Elite code is (C) I.Bell & D.Braben 1984.
 * This version re-engineered in C by C.J.Pinder 1999-2001.
 *
 * email: <christian@newkind.co.uk>
 *
 *
 */

use crate::{
    planet::GalaxySeed, shipdata::NO_OF_SHIPS, swat::MISSILE_UNARMED, trade::NO_OF_STOCK_ITEMS,
};

#[derive(Copy, Clone)]
pub struct ShipPoint {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub dist: i16,
    pub face1: i16,
    pub face2: i16,
    pub face3: i16,
    pub face4: i16,
}

impl ShipPoint {
    pub fn new(
        x: i16,
        y: i16,
        z: i16,
        dist: i16,
        face1: i16,
        face2: i16,
        face3: i16,
        face4: i16,
    ) -> Self {
        Self {
            x,
            y,
            z,
            dist,
            face1,
            face2,
            face3,
            face4,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ShipLine {
    pub dist: i16,
    pub face1: i16,
    pub face2: i16,
    pub start_point: i16,
    pub end_point: i16,
}

impl ShipLine {
    pub fn new(dist: i16, face1: i16, face2: i16, start_point: i16, end_point: i16) -> Self {
        Self {
            dist,
            face1,
            face2,
            start_point,
            end_point,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ShipFaceNormal {
    pub dist: i16,
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl ShipFaceNormal {
    pub fn new(dist: i16, x: i16, y: i16, z: i16) -> Self {
        Self { dist, x, y, z }
    }
}
pub struct ShipData {
    pub name: [char; 32],
    pub num_points: i16,
    pub num_lines: i16,
    pub num_faces: i16,
    pub max_loot: i16,
    pub scoop_type: i16,
    pub size: f32,
    pub front_laser: i16,
    pub bounty: i16,
    pub vanish_point: i16,
    pub energy: i16,
    pub velocity: i16,
    pub missiles: i16,
    pub laser_strength: i16,
    pub points: Vec<ShipPoint>,
    pub lines: Vec<ShipLine>,
    pub normals: Vec<ShipFaceNormal>,
}

impl Clone for ShipData {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            num_points: self.num_points.clone(),
            num_lines: self.num_lines.clone(),
            num_faces: self.num_faces.clone(),
            max_loot: self.max_loot.clone(),
            scoop_type: self.scoop_type.clone(),
            size: self.size.clone(),
            front_laser: self.front_laser.clone(),
            bounty: self.bounty.clone(),
            vanish_point: self.vanish_point.clone(),
            energy: self.energy.clone(),
            velocity: self.velocity.clone(),
            missiles: self.missiles.clone(),
            laser_strength: self.laser_strength.clone(),
            points: self.points.clone(),
            lines: self.lines.clone(),
            normals: self.normals.clone(),
        }
    }
}
pub const SCR_INTRO_ONE: i16 = 1;
pub const SCR_INTRO_TWO: i16 = 2;
pub const SCR_GALACTIC_CHART: i16 = 3;
pub const SCR_SHORT_RANGE: i16 = 4;
pub const SCR_PLANET_DATA: i16 = 5;
pub const SCR_MARKET_PRICES: i16 = 6;
pub const SCR_CMDR_STATUS: i16 = 7;
pub const SCR_FRONT_VIEW: i16 = 8;
pub const SCR_REAR_VIEW: i16 = 9;
pub const SCR_LEFT_VIEW: i16 = 10;
pub const SCR_RIGHT_VIEW: i16 = 11;
pub const SCR_BREAK_PATTERN: i16 = 12;
pub const SCR_INVENTORY: i16 = 13;
pub const SCR_EQUIP_SHIP: i16 = 14;
pub const SCR_OPTIONS: i16 = 15;
pub const SCR_LOAD_CMDR: i16 = 16;
pub const SCR_SAVE_CMDR: i16 = 17;
pub const SCR_QUIT: i16 = 18;
pub const SCR_GAME_OVER: i16 = 19;
pub const SCR_SETTINGS: i16 = 20;
pub const SCR_ESCAPE_POD: i16 = 21;

pub const PULSE_LASER: i16 = 0x0F;
pub const BEAM_LASER: i16 = 0x8F;
pub const MILITARY_LASER: i16 = 0x97;
pub const MINING_LASER: i16 = 0x32;

pub const FLG_DEAD: i16 = 1;
pub const FLG_REMOVE: i16 = 2;
pub const FLG_EXPLOSION: i16 = 4;
pub const FLG_ANGRY: i16 = 8;
pub const FLG_FIRING: i16 = 16;
pub const FLG_HAS_ECM: i16 = 32;
pub const FLG_HOSTILE: i16 = 64;
pub const FLG_CLOAKED: i16 = 128;
pub const FLG_FLY_TO_PLANET: i16 = 256;
pub const FLG_FLY_TO_STATION: i16 = 512;
pub const FLG_INACTIVE: i16 = 1024;
pub const FLG_SLOW: i16 = 2048;
pub const FLG_BOLD: i16 = 4096;
pub const FLG_POLICE: i16 = 8192;

pub const MAX_UNIV_OBJECTS: i16 = 20;

pub struct Commander {
    pub name: [char; 32],
    pub galaxy_seed: GalaxySeed,
    pub mission: i16,
    pub ship_x: i16,
    pub ship_y: i16,
    pub credits: i16,
    pub fuel: i16,
    pub unused1: i16,
    pub galaxy_number: i16,
    pub front_laser: i16,
    pub rear_laser: i16,
    pub left_laser: i16,
    pub right_laser: i16,
    pub unused2: i16,
    pub unused3: i16,
    pub cargo_capacity: i16,
    pub ecm: i16,
    pub fuel_scoop: i16,
    pub energy_bomb: i16,
    pub energy_unit: i16,
    pub docking_computer: i16,
    pub galactic_hyperdrive: i16,
    pub escape_pod: i16,
    pub unused4: i16,
    pub unused5: i16,
    pub unused6: i16,
    pub unused7: i16,
    pub missiles: i16,
    pub legal_status: i16,
    pub market_rnd: i16,
    pub score: i16,
    pub saved: i16,
    pub station_stock: [i16; NO_OF_STOCK_ITEMS],
    pub current_cargo: [i16; NO_OF_STOCK_ITEMS],
}

impl Commander {
    pub fn set_name(&mut self, new_name: &str) {
        for i in 0..self.name.len() {
            self.name[i] = ' ';
        }
        for (n, c) in new_name.chars().enumerate() {
            if n < self.name.len() {
                self.name[n] = c;
            }
        }
    }
    pub fn new() -> Self {
        Self {
            name: ['x'; 32],
            mission: 1,
            galaxy_seed: GalaxySeed::new(),
            ship_x: 1,
            ship_y: 1,
            credits: 1,
            fuel: 1,
            unused1: 1,
            galaxy_number: 1,
            front_laser: 1,
            rear_laser: 1,
            left_laser: 1,
            right_laser: 1,
            unused2: 1,
            unused3: 1,
            cargo_capacity: 1,
            current_cargo: [1; NO_OF_STOCK_ITEMS],
            ecm: 1,
            fuel_scoop: 1,
            energy_bomb: 1,
            energy_unit: 1,
            docking_computer: 1,
            galactic_hyperdrive: 1,
            escape_pod: 1,
            unused4: 1,
            unused5: 1,
            unused6: 1,
            unused7: 1,
            missiles: 1,
            legal_status: 1,
            station_stock: [1; NO_OF_STOCK_ITEMS],
            market_rnd: 1,
            score: 1,
            saved: 1,
        }
    }
    pub fn get_saved() -> Self {
        let mut result = Self {
            name: ['x'; 32],
            mission: 1,
            galaxy_seed: GalaxySeed::new(),
            ship_x: 1,
            ship_y: 1,
            credits: 1,
            fuel: 1,
            unused1: 1,
            galaxy_number: 1,
            front_laser: 1,
            rear_laser: 1,
            left_laser: 1,
            right_laser: 1,
            unused2: 1,
            unused3: 1,
            cargo_capacity: 1,
            current_cargo: [1; NO_OF_STOCK_ITEMS],
            ecm: 1,
            fuel_scoop: 1,
            energy_bomb: 1,
            energy_unit: 1,
            docking_computer: 1,
            galactic_hyperdrive: 1,
            escape_pod: 1,
            unused4: 1,
            unused5: 1,
            unused6: 1,
            unused7: 1,
            missiles: 1,
            legal_status: 1,
            station_stock: [1; NO_OF_STOCK_ITEMS],
            market_rnd: 1,
            score: 1,
            saved: 1,
        };
        result.set_name("JAMESON"); /* Name */
        result.mission = 0; /* Mission Number */
        result.ship_x = 0x14; /* Ship X,Y */
        result.ship_y = 0xAD; /* Ship X,Y */
        result.galaxy_seed = GalaxySeed::new();
        result.galaxy_seed.set(0x4a, 0x5a, 0x48, 0x02, 0x53, 0xb7); /* Galaxy Seed */
        result.credits = 1000; /* Credits * 10 */
        result.fuel = 70; /* Fuel * 10 */
        result.unused1 = 0;
        result.galaxy_number = 0; /* Galaxy - 1		*/
        result.front_laser = PULSE_LASER; /* Front Laser		*/
        result.rear_laser = 0; /* Rear Laser		*/
        result.left_laser = 0; /* Left Laser		*/
        result.right_laser = 0; /* Right Laser		*/
        result.unused2 = 0;
        result.unused3 = 0;
        result.cargo_capacity = 20; /* Cargo Capacity	*/
        result.current_cargo = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; /* Current Cargo	*/
        result.ecm = 0; /* ECM				*/
        result.fuel_scoop = 0; /* Fuel Scoop		*/
        result.energy_bomb = 0; /* Energy Bomb		*/
        result.energy_unit = 0; /* Energy Unit		*/
        result.docking_computer = 0; /* Docking Computer */
        result.galactic_hyperdrive = 0; /* Galactic H'Drive	*/
        result.escape_pod = 0; /* Escape Pod		*/
        result.unused4 = 0;
        result.unused5 = 0;
        result.unused6 = 0;
        result.unused7 = 0;
        result.missiles = 3; /* No. of Missiles	*/
        result.legal_status = 0; /* Legal Status		*/
        result.station_stock = [
            0x10, 0x0F, 0x11, 0x00, 0x03, 0x1C, /* Station Stock	*/
            0x0E, 0x00, 0x00, 0x0A, 0x00, 0x11, 0x3A, 0x07, 0x09, 0x08, 0x00,
        ];
        result.market_rnd = 0; /* Fluctuation		*/
        result.score = 0; /* Score			*/
        result.saved = 0x80; /* Saved			*/
        result
    }

    pub fn _set(
        name: [char; 32],
        galaxy_seed: GalaxySeed,
        mission: i16,
        ship_x: i16,
        ship_y: i16,
        credits: i16,
        fuel: i16,
        unused1: i16,
        galaxy_number: i16,
        front_laser: i16,
        rear_laser: i16,
        left_laser: i16,
        right_laser: i16,
        unused2: i16,
        unused3: i16,
        cargo_capacity: i16,
        ecm: i16,
        fuel_scoop: i16,
        energy_bomb: i16,
        energy_unit: i16,
        docking_computer: i16,
        galactic_hyperdrive: i16,
        escape_pod: i16,
        unused4: i16,
        unused5: i16,
        unused6: i16,
        unused7: i16,
        missiles: i16,
        legal_status: i16,
        market_rnd: i16,
        score: i16,
        saved: i16,
        station_stock: [i16; NO_OF_STOCK_ITEMS],
        current_cargo: [i16; NO_OF_STOCK_ITEMS],
    ) -> Self {
        Self {
            name,
            galaxy_seed,
            mission,
            ship_x,
            ship_y,
            credits,
            fuel,
            unused1,
            galaxy_number,
            front_laser,
            rear_laser,
            left_laser,
            right_laser,
            unused2,
            unused3,
            cargo_capacity,
            ecm,
            fuel_scoop,
            energy_bomb,
            energy_unit,
            docking_computer,
            galactic_hyperdrive,
            escape_pod,
            unused4,
            unused5,
            unused6,
            unused7,
            missiles,
            legal_status,
            market_rnd,
            score,
            saved,
            station_stock,
            current_cargo,
        }
    }
}

pub struct PlayerShip {
    pub max_speed: i16,
    pub max_roll: i16,
    pub max_climb: i16,
    pub max_fuel: i16,
    pub altitude: i16,
    pub cabtemp: i16,
    pub laser_temp: i16,
    pub laser_counter: i16,
    pub laser: i16,
    pub laser_x: u8,
    pub laser_y: u8,
    pub ecm_active: i16,
    pub missile_target: i16,
}

impl PlayerShip {
    pub fn new() -> Self {
        Self {
            max_speed: 0,
            max_roll: 0,
            max_climb: 0,
            max_fuel: 0,
            altitude: 0,
            cabtemp: 0,
            laser_temp: 0,
            laser_counter: 0,
            laser: 0,
            laser_x: 0,
            laser_y: 0,
            ecm_active: 0,
            missile_target: MISSILE_UNARMED,
        }
    }
    fn _set(
        max_speed: i16,
        max_roll: i16,
        max_climb: i16,
        max_fuel: i16,
        altitude: i16,
        cabtemp: i16,
        laser_temp: i16,
        laser_counter: i16,
        laser: i16,
        laser_x: u8,
        laser_y: u8,
        ecm_active: i16,
        missile_target: i16,
    ) -> Self {
        Self {
            max_speed,
            max_roll,
            max_climb,
            max_fuel,
            altitude,
            cabtemp,
            laser_temp,
            laser_counter,
            laser,
            laser_x,
            laser_y,
            ecm_active,
            missile_target,
        }
    }
}

/*
const SSHIP_LIST: [ship_data; NO_OF_SHIPS + 1] = [
    NULL,
    &missile_data,
    &coriolis_data,
    &esccaps_data,
    &alloy_data,
    &cargo_data,
    &boulder_data,
    &asteroid_data,
    &rock_data,
    &orbit_data,
    &transp_data,
    &cobra3a_data,
    &pythona_data,
    &boa_data,
    &anacnda_data,
    &hermit_data,
    &viper_data,
    &sidewnd_data,
    &mamba_data,
    &krait_data,
    &adder_data,
    &gecko_data,
    &cobra1_data,
    &worm_data,
    &cobra3b_data,
    &asp2_data,
    &pythonb_data,
    &ferdlce_data,
    &moray_data,
    &thargoid_data,
    &thargon_data,
    &constrct_data,
    &cougar_data,
    &dodec_data,
];
*/
// extern struct player_ship myship;

// extern struct commander cmdr;
// extern struct commander saved_cmdr;

// extern struct galaxy_seed docked_planet;

// extern struct galaxy_seed hyperspace_planet;

// extern struct planet_data current_planet_data;

// extern int carry_flag;
// extern int current_screen;

// extern struct ship_data *ship_list[];

// extern int wireframe;
// extern int anti_alias_gfx;
// extern char scanner_filename[256];
// extern int hoopy_casinos;
// extern int instant_dock;
// extern int speed_cap;
// extern int scanner_cx;
// extern int scanner_cy;
// extern int compass_centre_x;
// extern int compass_centre_y;

// extern int planet_render_style;

// extern int game_over;
// extern int docked;
// extern int finish;
// extern int flight_speed;
// extern int flight_roll;
// extern int flight_climb;
// extern int front_shield;
// extern int aft_shield;
// extern int energy;
// extern int laser_temp;
// extern int mcount;
// extern int detonate_bomb;
// extern int witchspace;
// extern int auto_pilot;

// void restore_saved_commander (void);

// #endif
