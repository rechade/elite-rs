use macroquad::prelude::rand;

use crate::{GameParams, My, elite::Commander};
#[derive(Copy, Clone)]
pub struct GalaxySeed {
    pub a: u8, /* 6c */
    pub b: u8, /* 6d */
    pub c: u8, /* 6e */
    pub d: u8, /* 6f */
    pub e: u8, /* 70 */
    pub f: u8, /* 71 */
}

impl GalaxySeed {
    pub fn new() -> Self {
        Self {
            a: rand::gen_range(0, u8::MAX),
            b: rand::gen_range(0, u8::MAX),
            c: rand::gen_range(0, u8::MAX),
            d: rand::gen_range(0, u8::MAX),
            e: rand::gen_range(0, u8::MAX),
            f: rand::gen_range(0, u8::MAX),
        }
    }

    pub fn set(&mut self, a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) {
        *self = GalaxySeed { a, b, c, d, e, f };
    }
}

pub struct PlanetData {
    government: u8,
    economy: u8,
    pub techlevel: u8,
    population: u8,
    productivity: u8,
    radius: u8,
}

impl PlanetData {
    pub fn new(
        government: u8,
        economy: u8,
        techlevel: u8,
        population: u8,
        productivity: u8,
        radius: u8,
    ) -> Self {
        Self {
            government,
            economy,
            techlevel,
            population,
            productivity,
            radius,
        }
    }
}
pub fn generate_planet_data(planet_seed: &GalaxySeed) -> PlanetData {
    let mut pl = PlanetData::new(0, 0, 0, 0, 0, 0);
    pl.government = (planet_seed.c / 8) & 7;

    pl.economy = planet_seed.b & 7;

    if pl.government < 2 {
        pl.economy |= 2;
    }

    pl.techlevel = pl.economy ^ 7;
    pl.techlevel += planet_seed.d & 3;
    pl.techlevel += (pl.government / 2) + (pl.government & 1);

    pl.population = pl.techlevel * 4;
    pl.population += pl.government;
    pl.population += pl.economy;
    pl.population += 1;

    pl.productivity = (pl.economy ^ 7) + 3;
    pl.productivity *= pl.government + 4;
    pl.productivity *= pl.population;
    pl.productivity *= 8;

    pl.radius = (((planet_seed.f & 15) + 11) * 255) + planet_seed.d;
    pl
}
pub fn find_planet(cx: My, cy: My, cmdr: &Commander, params: &mut GameParams) -> GalaxySeed {
    let mut min_dist = 10000;
    let mut glx: GalaxySeed;
    let mut planet: GalaxySeed;
    let mut distance;
    let mut dx;
    let mut dy;

    glx = cmdr.galaxy_seed;
    planet = glx;

    for _ in 0..256 {
        dx = (cx - glx.d as My).abs();
        dy = (cy - glx.b as My).abs();

        if dx > dy {
            distance = (dx + dx + dy) / 2;
        } else {
            distance = (dx + dy + dy) / 2;
        }

        if distance < min_dist {
            min_dist = distance;
            planet = glx;
        }

        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
    }

    planet
}
fn waggle_galaxy(glx_ptr: &mut GalaxySeed, carry_flag: &mut My) {
    let mut x: u16;
    let mut y: u16;

    x = glx_ptr.a as u16 + glx_ptr.c as u16;
    y = glx_ptr.b as u16 + glx_ptr.d as u16;

    if x > 0xFF {
        y += 1;
    }

    x &= 0xFF;
    y &= 0xFF;

    glx_ptr.a = glx_ptr.c;
    glx_ptr.b = glx_ptr.d;
    glx_ptr.c = glx_ptr.e;
    glx_ptr.d = glx_ptr.f;

    x += glx_ptr.c as u16;
    y += glx_ptr.d as u16;

    if x > 0xFF {
        y += 1;
    }

    if y > 0xFF {
        *carry_flag = 1;
    } else {
        *carry_flag = 0;
    }

    x &= 0xFF;
    y &= 0xFF;

    glx_ptr.e = x as u8;
    glx_ptr.f = y as u8;
}
const DIGRAMS: &str =
    "ABOUSEITILETSTONLONUTHNOALLEXEGEZACEBISOUSESARMAINDIREA?ERATENBERALAVETIEDORQUANTEISRION";
pub fn name_planet(gname: &mut String, glx: &mut GalaxySeed, carry_flag: &mut My) {
    let mut x: u8;

    *gname = "".to_string();

    let size = if (glx.a & 0x40) == 0 { 3 } else { 4 };

    for _ in 0..size {
        x = glx.f & 0x1F;
        if x != 0 {
            x += 12;
            x *= 2;
            *gname += &DIGRAMS[x as usize..(x + 1) as usize];
            let contains = DIGRAMS[(x + 1) as usize..(x + 2) as usize].contains('?');
            if contains {
                *gname += &DIGRAMS[(x + 1) as usize..(x + 2) as usize];
            }
        }
        waggle_galaxy(glx, carry_flag);
    }
}

pub fn capitalise_name(name: &mut String) {
    let mut initial = "".to_string();
    let mut remaining = "".to_string();
    if !name.is_empty() {
        initial = name[0..1].to_uppercase();
    }
    if name.len() > 1 {
        remaining = name[1..name.len()].to_lowercase();
    }
    *name = initial + &remaining;
}
