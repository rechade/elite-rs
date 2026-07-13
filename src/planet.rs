use macroquad::prelude::rand;

use crate::{
    docked::DESC_LIST,
    elite::Commander,
    stars::{gen_rnd_number, rand255},
    Config, GameParams, My,
};
#[derive(Copy, Clone, Debug)]
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

fn expand_description(
    source: &str,
    hyperspace_planet: &mut GalaxySeed,
    carry_flag: &mut My,
    rnd_seed: &mut GalaxySeed,
) -> String {
    let mut processing = true;
    let mut num: usize;
    let mut start = 0;
    let mut end = 0;
    let mut rnd;
    let mut option = 0;
    let mut result = source.to_string();
    let mut da_str = "".to_string();
    while result.contains("<") && result.contains(">") {
        for i in 0..result.len() {
            if &result[i..i + 1] == "<" {
                start = i;
                break;
            }
        }
        for j in 0..result.len() {
            if &result[j..j + 1] == ">" {
                end = j;
                break;
            }
        }
        num = result[start + 1..end].parse().unwrap();
        // if (hoopy_casinos)
        // crst
        if (false) {
            // option = gen_msx_rnd_number();
        } else {
            rnd = gen_rnd_number(rnd_seed);
            option = 0;
            if (rnd >= 0x33) {
                option += 1
            };
            if (rnd >= 0x66) {
                option += 1
            };
            if (rnd >= 0x99) {
                option += 1
            };
            if (rnd >= 0xCC) {
                option += 1
            };
        }
        for n in start..=end {
            result.remove(start);
        }
        result =
            result[0..start].to_string() + DESC_LIST[num][option] + &result[start..].to_string();
    }
    while result.contains("%") {
        for i in 0..result.len() {
            if &result[i..i + 1] == "%" {
                start = i;
                break;
            }
        }
        result.remove(start);
        match &result[start..start + 1] {
            "H" => {
                result.remove(start);
                name_planet(&mut da_str, &mut hyperspace_planet.clone(), carry_flag);
                capitalise_name(&mut da_str);
                result = result[0..start].to_string() + &da_str + &result[start..];
            }
            "I" => {
                result.remove(start);
                name_planet(&mut da_str, &mut hyperspace_planet.clone(), carry_flag);
                capitalise_name(&mut da_str);
                result = result[0..start].to_string() + &da_str + "ian" + &result[start..];
            }
            "R" => {
                result.remove(start);
                let len = gen_rnd_number(rnd_seed) & 3;
                for i in 0..len {
                    let x = gen_rnd_number(rnd_seed) & 0x3e;
                    if (i == 0) {
                        result = result + &DIGRAMS[x as usize..x as usize];
                    } else {
                        result = result + &DIGRAMS[x as usize..x as usize].to_ascii_lowercase();
                    }
                    result = result + &DIGRAMS[x as usize..x as usize].to_ascii_lowercase();
                }
            }
            _ => (),
        }
    }
    result = result.replace("  ", " ");
    result
}
pub fn describe_planet(
    planet: &mut GalaxySeed,
    cmdr: &Commander,
    config: &Config,
    carry_flag: &mut My,
) -> String {
    let mut mission_text = "".to_string();
    let mut rnd_seed: GalaxySeed = *planet;

    if (cmdr.mission == 1) {
        //crst
        // mission_text = mission_planet_desc (planet);
        // if (mission_text != NULL){
        // 	return mission_text;
        // }
    }

    let mut planet_description = "".to_string();
    rnd_seed.a = planet.c;
    rnd_seed.b = planet.d;
    rnd_seed.c = planet.e;
    rnd_seed.d = planet.f;

    // crst
    // if (config.hoopy_casinos != 0) {
    //     rnd_seed.a ^= planet.a;
    //     rnd_seed.b ^= planet.b;
    //     rnd_seed.c ^= rnd_seed.a;
    //     rnd_seed.d ^= rnd_seed.b;
    // }

    expand_description("<14> is <22>.", planet, carry_flag, &mut rnd_seed)
}
pub struct PlanetData {
    pub government: u16,
    pub economy: u16,
    pub techlevel: u16,
    pub population: u16,
    pub productivity: u16,
    pub radius: u16,
}

impl PlanetData {
    pub fn new(
        government: u16,
        economy: u16,
        techlevel: u16,
        population: u16,
        productivity: u16,
        radius: u16,
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
    pl.government = ((planet_seed.c / 8) & 7) as u16;

    pl.economy = (planet_seed.b & 7) as u16;

    if pl.government < 2 {
        pl.economy |= 2;
    }

    pl.techlevel = pl.economy ^ 7;
    pl.techlevel += (planet_seed.d & 3) as u16;
    pl.techlevel += ((pl.government / 2) + (pl.government & 1)) as u16;

    pl.population = pl.techlevel * 4;
    pl.population += pl.government;
    pl.population += pl.economy;
    pl.population += 1;

    pl.productivity = (pl.economy ^ 7) + 3;
    pl.productivity *= pl.government + 4;
    pl.productivity *= pl.population;
    pl.productivity *= 8;

    pl.radius = ((((planet_seed.f & 15) as u16 + 11) as u16 * 256) + planet_seed.d as u16);
    pl
}
pub fn find_planet(cx: My, cy: My, base_location: &GalaxySeed, carry_flag: &mut My) -> GalaxySeed {
    let mut min_dist = 127;
    let mut glx: GalaxySeed;
    let mut planet: GalaxySeed;
    let mut distance;
    let mut dx;
    let mut dy;

    glx = *base_location;
    planet = glx;

    for _ in 0..256 {
        dx = (cx - glx.d as My).abs();
        dy = (cy - glx.b as My).abs();
        if (dx > dy) {
            distance = (dx + dx + dy) / 2;
        } else {
            distance = (dx + dy + dy) / 2;
        }

        if distance <= min_dist {
            min_dist = distance;
            planet = glx;
        }
        waggle_galaxy(&mut glx, carry_flag);
        waggle_galaxy(&mut glx, carry_flag);
        waggle_galaxy(&mut glx, carry_flag);
        waggle_galaxy(&mut glx, carry_flag);
    }
    planet
}
pub fn waggle_galaxy(glx_ptr: &mut GalaxySeed, carry_flag: &mut My) {
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
            if !DIGRAMS[x as usize + 1..x as usize + 2].contains("?") {
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
// crst
pub fn find_planet_number(planet: &mut GalaxySeed, carry_flag: &mut My, cmdr: &Commander) -> My {
    let mut glx = cmdr.galaxy;
    for i in 0..256 {
        if ((planet.a == glx.a)
            && (planet.b == glx.b)
            && (planet.c == glx.c)
            && (planet.d == glx.d)
            && (planet.e == glx.e)
            && (planet.f == glx.f))
        {
            return i;
        }
        waggle_galaxy(&mut glx, carry_flag);
        waggle_galaxy(&mut glx, carry_flag);
        waggle_galaxy(&mut glx, carry_flag);
        waggle_galaxy(&mut glx, carry_flag);
    }
    return -1;
}
