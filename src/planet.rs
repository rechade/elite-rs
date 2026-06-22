use std::u8;

use macroquad::prelude::rand;

pub struct GalaxySeed {
    a: u8, /* 6c */
    b: u8, /* 6d */
    c: u8, /* 6e */
    d: u8, /* 6f */
    e: u8, /* 70 */
    f: u8, /* 71 */
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
pub fn generate_planet_data(pl: &mut PlanetData, planet_seed: &GalaxySeed) {
    pl.government = (planet_seed.c / 8) & 7;

    pl.economy = planet_seed.b & 7;

    if (pl.government < 2) {
        pl.economy = pl.economy | 2;
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
}
