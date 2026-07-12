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
    GameParams, My,
    planet::{GalaxySeed, find_planet, generate_planet_data},
    space::DaType,
    swat::MISSILE_UNARMED,
    trade::NO_OF_STOCK_ITEMS,
};

#[derive(Copy, Clone)]
pub struct ShipPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub dist: f32,
    pub face1: usize,
    pub face2: usize,
    pub face3: usize,
    pub face4: usize,
}

impl ShipPoint {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        dist: f32,
        face1: usize,
        face2: usize,
        face3: usize,
        face4: usize,
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
    pub dist: i32,
    pub face1: usize,
    pub face2: usize,
    pub start_point: usize,
    pub end_point: usize,
}

impl ShipLine {
    pub fn new(dist: My, face1: usize, face2: usize, start_point: usize, end_point: usize) -> Self {
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
    pub dist: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ShipFaceNormal {
    pub fn new(dist: f32, x: f32, y: f32, z: f32) -> Self {
        Self { dist, x, y, z }
    }
}
pub struct ShipData {
    pub name: [char; 32],
    pub num_points: usize,
    pub num_lines: usize,
    pub num_faces: usize,
    pub max_loot: My,
    pub scoop_type: My,
    pub size: f32,
    pub front_laser: usize,
    pub bounty: My,
    pub vanish_point: My,
    pub energy: My,
    pub velocity: My,
    pub missiles: My,
    pub laser_strength: My,
    pub points: Vec<ShipPoint>,
    pub lines: Vec<ShipLine>,
    pub normals: Vec<ShipFaceNormal>,
}
impl ShipData {
    pub fn get_name(&self) -> String {
        let mut s = "".to_string();
        for c in self.name {
            s += &c.to_string();
        }
        s.trim().to_string()
    }
}
impl Clone for ShipData {
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            num_points: self.num_points,
            num_lines: self.num_lines,
            num_faces: self.num_faces,
            max_loot: self.max_loot,
            scoop_type: self.scoop_type,
            size: self.size,
            front_laser: self.front_laser,
            bounty: self.bounty,
            vanish_point: self.vanish_point,
            energy: self.energy,
            velocity: self.velocity,
            missiles: self.missiles,
            laser_strength: self.laser_strength,
            points: self.points.clone(),
            lines: self.lines.clone(),
            normals: self.normals.clone(),
        }
    }
}
pub const SCR_INTRO_ONE: My = 1;
pub const SCR_INTRO_TWO: My = 2;
pub const SCR_GALACTIC_CHART: My = 3;
pub const SCR_SHORT_RANGE: My = 4;
pub const SCR_PLANET_DATA: My = 5;
pub const SCR_MARKET_PRICES: My = 6;
pub const SCR_CMDR_STATUS: My = 7;
pub const SCR_FRONT_VIEW: My = 8;
pub const SCR_REAR_VIEW: My = 9;
pub const SCR_LEFT_VIEW: My = 10;
pub const SCR_RIGHT_VIEW: My = 11;
pub const SCR_BREAK_PATTERN: My = 12;
pub const SCR_INVENTORY: My = 13;
pub const SCR_EQUIP_SHIP: My = 14;
pub const SCR_OPTIONS: My = 15;
pub const SCR_LOAD_CMDR: My = 16;
pub const SCR_SAVE_CMDR: My = 17;
pub const SCR_QUIT: My = 18;
pub const SCR_GAME_OVER: My = 19;
pub const SCR_SETTINGS: My = 20;
pub const SCR_ESCAPE_POD: My = 21;

pub const PULSE_LASER: My = 0x0F;
pub const BEAM_LASER: My = 0x8F;
pub const MILITARY_LASER: My = 0x97;
pub const MINING_LASER: My = 0x32;

pub const FLG_DEAD: My = 1;
pub const FLG_REMOVE: My = 2;
pub const FLG_EXPLOSION: My = 4;
pub const FLG_ANGRY: My = 8;
pub const FLG_FIRING: My = 16;
pub const FLG_HAS_ECM: My = 32;
pub const FLG_HOSTILE: My = 64;
pub const FLG_CLOAKED: My = 128;
pub const FLG_FLY_TO_PLANET: My = 256;
pub const FLG_FLY_TO_STATION: My = 512;
pub const FLG_INACTIVE: My = 1024;
pub const FLG_SLOW: My = 2048;
pub const FLG_BOLD: My = 4096;
pub const FLG_POLICE: My = 8192;

pub const MAX_UNIV_OBJECTS: usize = 20;

pub struct Commander {
    pub name: [char; 32],
    pub galaxy: GalaxySeed,
    pub mission: My,
    pub ship_x: My,
    pub ship_y: My,
    pub credits: My,
    pub fuel: My,
    pub unused1: My,
    pub galaxy_number: My,
    pub front_laser: My,
    pub rear_laser: My,
    pub left_laser: My,
    pub right_laser: My,
    pub unused2: My,
    pub unused3: My,
    pub cargo_capacity: My,
    pub ecm: My,
    pub fuel_scoop: My,
    pub energy_bomb: My,
    pub energy_unit: My,
    pub docking_computer: My,
    pub galactic_hyperdrive: My,
    pub escape_pod: My,
    pub unused4: My,
    pub unused5: My,
    pub unused6: My,
    pub unused7: My,
    pub missiles: My,
    pub legal_status: My,
    pub market_rnd: My,
    pub score: My,
    pub saved: My,
    pub station_stock: [My; NO_OF_STOCK_ITEMS],
    pub current_cargo: [My; NO_OF_STOCK_ITEMS],
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
            galaxy: GalaxySeed::new(),
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
            galaxy: GalaxySeed::new(),
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
        result.galaxy = GalaxySeed::new();
        result.galaxy.set(0x4a, 0x5a, 0x48, 0x02, 0x53, 0xb7); /* Galaxy Seed */
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
        result.docking_computer = 1; /* Docking Computer */
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
        mission: My,
        ship_x: My,
        ship_y: My,
        credits: My,
        fuel: My,
        unused1: My,
        galaxy_number: My,
        front_laser: My,
        rear_laser: My,
        left_laser: My,
        right_laser: My,
        unused2: My,
        unused3: My,
        cargo_capacity: My,
        ecm: My,
        fuel_scoop: My,
        energy_bomb: My,
        energy_unit: My,
        docking_computer: My,
        galactic_hyperdrive: My,
        escape_pod: My,
        unused4: My,
        unused5: My,
        unused6: My,
        unused7: My,
        missiles: My,
        legal_status: My,
        market_rnd: My,
        score: My,
        saved: My,
        station_stock: [My; NO_OF_STOCK_ITEMS],
        current_cargo: [My; NO_OF_STOCK_ITEMS],
    ) -> Self {
        Self {
            name,
            galaxy: galaxy_seed,
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
    pub max_speed: My,
    pub max_roll: My,
    pub max_climb: My,
    pub max_fuel: My,
    pub altitude: My,
    pub cabtemp: My,
    pub laser_temp: My,
    pub laser_counter: My,
    pub laser: My,
    pub laser_x: My,
    pub laser_y: My,
    pub ecm_active: bool,
    pub missile_target: DaType,
}

impl PlayerShip {
    pub fn new() -> Self {
        Self {
            max_speed: 0,
            max_roll: 0,
            max_climb: 0,
            max_fuel: 0,
            altitude: 0,
            cabtemp: 30,
            laser_temp: 0,
            laser_counter: 0,
            laser: 0,
            laser_x: 0,
            laser_y: 0,
            ecm_active: false,
            missile_target: MISSILE_UNARMED,
        }
    }
    fn _set(
        max_speed: My,
        max_roll: My,
        max_climb: My,
        max_fuel: My,
        altitude: My,
        cabtemp: My,
        laser_temp: My,
        laser_counter: My,
        laser: My,
        laser_x: My,
        laser_y: My,
        ecm_active: bool,
        missile_target: DaType,
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

pub fn restore_saved_commander(cmdr: &mut Commander, params: &mut GameParams) {
    *cmdr = Commander::get_saved();

    params.docked_planet = find_planet(
        cmdr.ship_x,
        cmdr.ship_y,
        &cmdr.galaxy,
        &mut params.carry_flag,
    );
    params.hyperspace_planet = params.docked_planet;

    params.current_planet_data = generate_planet_data(&params.docked_planet);
    generate_stock_market();
    set_stock_quantities(cmdr.station_stock);
}

fn set_stock_quantities(station_stock: [My; 17]) {
    println!("set_stock_quantities");
}

fn generate_stock_market() {
    println!("generate_stock_market");
}
