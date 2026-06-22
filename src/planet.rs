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

    pub fn set(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        Self { a, b, c, d, e, f }
    }
}

struct PlanetData {
    government: u8,
    economy: u8,
    techlevel: u8,
    population: u8,
    productivity: u8,
    radius: u8,
}
