use macroquad::{
    color::{GOLD, GREEN, RED, WHITE},
    input::{is_key_down, KeyCode},
    shapes::{draw_circle, draw_circle_lines, draw_line, draw_rectangle},
    text::{draw_text, draw_text_ex, measure_text, Font, TextParams},
};

use crate::{
    draw_cross,
    elite::{
        Commander, SCR_CMDR_STATUS, SCR_GALACTIC_CHART, SCR_MARKET_PRICES, SCR_PLANET_DATA,
        SCR_SHORT_RANGE,
    },
    gfx::{GFX_SCALE, GFX_X_CENTRE, GFX_Y_CENTRE},
    move_cross,
    planet::{
        capitalise_name, describe_planet, find_planet, generate_planet_data, name_planet,
        waggle_galaxy, GalaxySeed, PlanetData,
    },
    shipdata::{SHIP_DODEC, SHIP_MISSILE, SHIP_ROCK},
    space::{calc_distance_to_planet, DaType, UnivObject},
    trade::{total_cargo, StockItem},
    Config, GameParams, My, BEAM_LASER, FONT_BASE, MILITARY_LASER, MINING_LASER, PULSE_LASER,
    SCANNER_Y_PROPORTION, THICKNESS,
};

struct Rank {
    score: My,
    title: String,
}
pub const unit_name: [&str; 3] = ["t", "kg", "g"];

pub const TONNES: usize = 0;
pub const KILOGRAMS: usize = 1;
pub const GRAMS: usize = 2;
pub const SHORT_RANGE_X_FACTOR: f32 = 4.0;
pub const SHORT_RANGE_Y_FACTOR: f32 = 2.0;
pub const DESC_LIST: [[&str; 5]; 36] = [
    /*  0	*/ ["fabled", "notable", "well known", "famous", "noted"],
    /*  1	*/ ["very", "mildly", "most", "reasonably", ""],
    /*  2	*/ ["ancient", "<20>", "great", "vast", "pink"],
    /*  3	*/
    [
        "<29> <28> plantations",
        "mountains",
        "<27>",
        "<19> forests",
        "oceans",
    ],
    /*  4	*/
    [
        "shyness",
        "silliness",
        "mating traditions",
        "loathing of <5>",
        "love for <5>",
    ],
    /*  5	*/ ["food blenders", "tourists", "poetry", "discos", "<13>"],
    /*  6	*/ ["talking tree", "crab", "bat", "lobst", "%R"],
    /*  7	*/ ["beset", "plagued", "ravaged", "cursed", "scourged"],
    /*  8	*/
    [
        "<21> civil war",
        "<26> <23> <24>s",
        "a <26> disease",
        "<21> earthquakes",
        "<21> solar activity",
    ],
    /*  9	*/
    [
        "its <2> <3>",
        "the %I <23> <24>",
        "its inhabitants' <25> <4>",
        "<32>",
        "its <12> <13>",
    ],
    /* 10	*/ ["juice", "brandy", "water", "brew", "gargle blasters"],
    /* 11	*/ ["%R", "%I <24>", "%I %R", "%I <26>", "<26> %R"],
    /* 12	*/ ["fabulous", "exotic", "hoopy", "unusual", "exciting"],
    /* 13	*/ ["cuisine", "night life", "casinos", "sit coms", " <32> "],
    /* 14	*/
    [
        "%H",
        "The planet %H",
        "The world %H",
        "This planet",
        "This world",
    ],
    /* 15	*/
    [
        "n unremarkable",
        " boring",
        " dull",
        " tedious",
        " revolting",
    ],
    /* 16	*/ ["planet", "world", "place", "little planet", "dump"],
    /* 17	*/ ["wasp", "moth", "grub", "ant", "%R"],
    /* 18	*/ ["poet", "arts graduate", "yak", "snail", "slug"],
    /* 19	*/ ["tropical", "dense", "rain", "impenetrable", "exuberant"],
    /* 20	*/ ["funny", "wierd", "unusual", "strange", "peculiar"],
    /* 21	*/
    [
        "frequent",
        "occasional",
        "unpredictable",
        "dreadful",
        "deadly",
    ],
    /* 22	*/
    [
        "<1> <0> for <9>",
        "<1> <0> for <9> and <9>",
        "<7> by <8>",
        "<1> <0> for <9> but <7> by <8>",
        " a<15> <16>",
    ],
    /* 23	*/ ["<26>", "mountain", "edible", "tree", "spotted"],
    /* 24	*/ ["<30>", "<31>", "<6>oid", "<18>", "<17>"],
    /* 25	*/ ["ancient", "exceptional", "eccentric", "ingrained", "<20>"],
    /* 26	*/ ["killer", "deadly", "evil", "lethal", "vicious"],
    /* 27	*/
    [
        "parking meters",
        "dust clouds",
        "ice bergs",
        "rock formations",
        "volcanoes",
    ],
    /* 28	*/ ["plant", "tulip", "banana", "corn", "%Rweed"],
    /* 29	*/ ["%R", "%I %R", "%I <26>", "inhabitant", "%I %R"],
    /* 30	*/ ["shrew", "beast", "bison", "snake", "wolf"],
    /* 31	*/ ["leopard", "cat", "monkey", "goat", "fish"],
    /* 32	*/
    [
        "<11> <10>",
        "%I <30> <33>",
        "its <12> <31> <33>",
        "<34> <35>",
        "<11> <10>",
    ],
    /* 33	*/ ["meat", "cutlet", "steak", "burgers", "soup"],
    /* 34	*/ ["ice", "mud", "Zero-G", "vacuum", "%I ultra"],
    /* 35	*/ ["hockey", "cricket", "karate", "polo", "tennis"],
];
const INHABITANT_DESC1: [&str; 3] = ["Large ", "Fierce ", "Small "];

const INHABITANT_DESC2: [&str; 6] = ["Green ", "Red ", "Yellow ", "Blue ", "Black ", "Harmless "];

const INHABITANT_DESC3: [&str; 6] = ["Slimy ", "Bug-Eyed ", "Horned ", "Bony ", "Fat ", "Furry "];

const INHABITANT_DESC4: [&str; 8] = [
    "Rodent", "Frog", "Lizard", "Lobster", "Bird", "Humanoid", "Feline", "Insect",
];
const ECONOMY_TYPE: [&str; 8] = [
    "Rich Industrial",
    "Average Industrial",
    "Poor Industrial",
    "Mainly Industrial",
    "Mainly Agricultural",
    "Rich Agricultural",
    "Average Agricultural",
    "Poor Agricultural",
];

const GOVERNMENT_TYPE: [&str; 8] = [
    "Anarchy",
    "Feudal",
    "Multi-Government",
    "Dictatorship",
    "Communist",
    "Confederacy",
    "Democracy",
    "Corporate State",
];
const NO_OF_RANKS: usize = 9;

const EQUIP_START_Y: f32 = 202.0;
const EQUIP_START_X: f32 = 50.0;
const EQUIP_MAX_Y: f32 = 290.0;
const EQUIP_WIDTH: f32 = 200.0;
const Y_INC: f32 = 16.0;

const CONDITION_TXT: [&str; 4] = ["Docked", "Green", "Yellow", "Red"];
pub fn display_short_range_chart(
    params: &mut GameParams,
    cmdr: &mut Commander,
    text_params: &mut TextParams,
    font: &Font,
) {
    // jjj
    const NUM_ROWS: usize = 128;
    let mut dx;
    let mut dy;
    let mut px;
    let mut py;
    let mut planet_name: String = "".to_string();
    let mut row_used: [i16; NUM_ROWS] = [0; NUM_ROWS];
    let mut row;
    let mut blob_size;
    params.current_screen = SCR_SHORT_RANGE;
    let da_str = format!("SHORT RANGE CHART");
    text_params.color = GOLD;
    let pos_x = (params.screen_width
        - measure_text(&da_str, Some(&font), FONT_BASE, params.screen_scale).width)
        * 0.5;
    draw_text_ex(
        &da_str,
        pos_x,
        15.0 * params.screen_scale,
        text_params.clone(),
    );
    draw_line(
        0.0,
        25.0 * params.screen_scale,
        params.screen_width,
        25.0 * params.screen_scale,
        THICKNESS,
        WHITE,
    );
    draw_fuel_limit_circle(params.mid_screen_x, params.mid_screen_y, params, cmdr);
    for i in 0..NUM_ROWS {
        row_used[i] = 0;
    }
    let mut glx = cmdr.galaxy;
    for i in 0..256 {
        // nothing waggled yet. original glx
        // it's just for some test values to see
        // if it gives good values before starting, or needs waggling
        dx = (glx.d as My - params.docked_planet.d as My).abs();
        dy = (glx.b as My - params.docked_planet.b as My).abs();
        if ((dx >= 20) || (dy >= 38)) {
            // only if needed
            waggle_galaxy(&mut glx, &mut params.carry_flag);
            waggle_galaxy(&mut glx, &mut params.carry_flag);
            waggle_galaxy(&mut glx, &mut params.carry_flag);
            waggle_galaxy(&mut glx, &mut params.carry_flag);
            continue;
        }
        /* Convert to screen co-ords */
        // glx has now been waggled, generates different coords
        px = short_range_world_to_screen(
            glx.d,
            params.docked_planet.d,
            SHORT_RANGE_X_FACTOR,
            params.mid_screen_x,
            params.screen_scale,
        ) as f32;
        py = short_range_world_to_screen(
            glx.b,
            params.docked_planet.b,
            SHORT_RANGE_Y_FACTOR,
            params.mid_screen_y,
            params.screen_scale,
        ) as f32;

        row = (py / (8.0 * params.screen_scale)) as usize;
        if (row_used[row] == 1) {
            row += 1;
        }
        if (row_used[row] == 1) {
            row -= 2;
        }
        if (row <= 3) {
            waggle_galaxy(&mut glx, &mut params.carry_flag);
            waggle_galaxy(&mut glx, &mut params.carry_flag);
            waggle_galaxy(&mut glx, &mut params.carry_flag);
            waggle_galaxy(&mut glx, &mut params.carry_flag);
            continue;
        }
        text_params.color = WHITE;
        // now use the glx that met above to get the planet
        if (row_used[row] == 0) {
            row_used[row] = 1;
            name_planet(&mut planet_name, &mut glx.clone(), &mut params.carry_flag);
            capitalise_name(&mut planet_name);
            draw_text_ex(
                &planet_name,
                px + (4.0 * params.screen_scale),
                (row * 8 - 5) as f32 * params.screen_scale,
                text_params.clone(),
            );
        }
        /* The next bit calculates the size of the circle used to represent */
        /* a planet.  The carry_flag is left over from the name generation. */
        /* Yes this was how it was done... don't ask :-( */
        blob_size = (glx.f & 1) as f32 + 2.0 + params.carry_flag as f32;
        blob_size *= params.screen_scale;
        draw_circle(px, py, blob_size, GOLD);
        // waggle ready for start of next loop
        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::S) {
        move_cross(params, 0, -1);
    }

    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::X) {
        move_cross(params, 0, 1);
    }

    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::Comma) {
        move_cross(params, -1, 0);
    }

    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::Period) {
        move_cross(params, 1, 0);
    }
    draw_cross(params, 0, 0);
    show_distance_to_planet(params, text_params, font, cmdr);
}
pub fn show_distance_to_planet(
    params: &mut GameParams,
    text_params: &TextParams,
    font: &Font,
    cmdr: &mut Commander,
) {
    // jjj
    let mut px;
    let mut py;
    let mut planet_name = "".to_string();

    if (params.current_screen == SCR_GALACTIC_CHART) {
        px = params.cross_x as f32 / params.screen_scale;
        py = (params.cross_y as f32 - ((18.0 * params.screen_scale) + 1.0))
            * (2.0 / params.screen_scale);
    } else {
        // use cross screen coords and docked planet world coords
        // to get the world-coords of the targetted spot
        px = short_range_screen_to_world(
            params.cross_x as f32,
            params.mid_screen_x,
            SHORT_RANGE_X_FACTOR,
            params.docked_planet.d as f32,
            params.screen_scale,
        );
        py = short_range_screen_to_world(
            params.cross_y as f32,
            params.mid_screen_y,
            SHORT_RANGE_Y_FACTOR,
            params.docked_planet.b as f32,
            params.screen_scale,
        );
    }

    // find the closest planet to the targetted spot
    params.hyperspace_planet =
        find_planet(px as My, py as My, &cmdr.galaxy, &mut params.carry_flag);
    params.hack_planet = params.hyperspace_planet;
    name_planet(
        &mut planet_name,
        &mut params.hyperspace_planet.clone(),
        &mut params.carry_flag,
    );
    params.dest_planet_string = planet_name;
    capitalise_name(&mut params.dest_planet_string);
    params.dest_planet_string = params.dest_planet_string.clone() + " - ";

    show_distance(
        356,
        &params.docked_planet,
        &params.hyperspace_planet,
        &mut params.dest_planet_string,
    );
    // snap the cross to the location returned from find_planet()
    if (params.current_screen == SCR_GALACTIC_CHART) {
        params.cross_x = (params.hyperspace_planet.d as f32 * params.screen_scale) as My;
        params.cross_y = (params.hyperspace_planet.b as f32 / (2.0 / params.screen_scale)
            + (18.0 * params.screen_scale)
            + 1.0) as My;
    }
    let msg_width = measure_text(
        &params.dest_planet_string,
        Some(font),
        FONT_BASE,
        params.screen_scale,
    )
    .width;
    let x_pos = (params.screen_width - msg_width) * 0.5;
    draw_text_ex(
        &params.dest_planet_string,
        x_pos,
        params.screen_height * 0.95 * (1.0 - SCANNER_Y_PROPORTION),
        text_params.clone(),
    );
}
fn draw_fuel_limit_circle(cx: f32, cy: f32, params: &mut GameParams, cmdr: &mut Commander) {
    let radius;
    let cross_size;
    if (params.current_screen == SCR_GALACTIC_CHART) {
        radius = cmdr.fuel as f32 / 4.0 * params.screen_scale;
        cross_size = 7.0 * params.screen_scale;
    } else {
        radius = cmdr.fuel as f32 * params.screen_scale;
        cross_size = 16.0 * params.screen_scale;
    }
    draw_circle_lines(cx, cy, radius, THICKNESS, GREEN);
    draw_line(cx, cy - cross_size, cx, cy + cross_size, THICKNESS, WHITE);
    draw_line(cx - cross_size, cy, cx + cross_size, cy, THICKNESS, WHITE);
}
/*
 * Displays data on the currently selected Hyperspace Planet.
 */
pub fn display_data_on_planet(
    params: &mut GameParams,
    text_params: &mut TextParams,
    font: &Font,
    cmdr: &mut Commander,
    config: &Config,
) {
    let mut planet_name: String = "".to_string();
    let mut da_str: String = "".to_string();
    let mut description: String = "".to_string();
    let mut hyper_planet_data: PlanetData;
    params.current_screen = SCR_PLANET_DATA;

    name_planet(
        &mut planet_name,
        &mut params.hyperspace_planet.clone(),
        &mut params.carry_flag,
    );
    da_str = format!("DATA ON {}", planet_name);
    let mut pos_x = (params.screen_width
        - measure_text(&da_str, Some(&font), FONT_BASE, params.screen_scale).width)
        * 0.5;
    text_params.color = GOLD;
    draw_text_ex(
        &da_str,
        pos_x,
        15.0 * params.screen_scale,
        text_params.clone(),
    );
    draw_line(
        0.0,
        25.0 * params.screen_scale,
        params.screen_width,
        25.0 * params.screen_scale,
        THICKNESS,
        WHITE,
    );
    hyper_planet_data = generate_planet_data(&params.hyperspace_planet);
    text_params.color = WHITE;
    params.dest_planet_string = "".to_string();
    show_distance(
        42,
        &params.docked_planet,
        &params.hyperspace_planet,
        &mut params.dest_planet_string,
    );
    pos_x = (params.screen_width
        - measure_text(
            &params.dest_planet_string,
            Some(&font),
            FONT_BASE,
            params.screen_scale,
        )
        .width)
        * 0.5;
    draw_text_ex(
        &params.dest_planet_string,
        pos_x,
        42.0 * params.screen_scale,
        text_params.clone(),
    );
    da_str = format!(
        "Economy:{}",
        ECONOMY_TYPE[hyper_planet_data.economy as usize]
    );
    pos_x = (params.screen_width
        - measure_text(&da_str, Some(&font), FONT_BASE, params.screen_scale).width)
        * 0.5;
    draw_text_ex(
        &da_str,
        pos_x,
        74.0 * params.screen_scale,
        text_params.clone(),
    );
    pos_x = (params.screen_width
        - measure_text(&da_str, Some(&font), FONT_BASE, params.screen_scale).width)
        * 0.5;
    da_str = format!(
        "Government:{}",
        GOVERNMENT_TYPE[hyper_planet_data.government as usize],
    );
    draw_text_ex(
        &da_str,
        pos_x,
        106.0 * params.screen_scale,
        text_params.clone(),
    );
    da_str = format!("Tech.Level:{}", hyper_planet_data.techlevel + 1);
    pos_x = (params.screen_width
        - measure_text(&da_str, Some(&font), FONT_BASE, params.screen_scale).width)
        * 0.5;
    draw_text_ex(
        &da_str,
        pos_x,
        138.0 * params.screen_scale,
        text_params.clone(),
    );
    da_str = format!(
        "Population:{}.{} Billion",
        hyper_planet_data.population / 10,
        hyper_planet_data.population % 10,
    );
    pos_x = (params.screen_width
        - measure_text(&da_str, Some(&font), 12, params.screen_scale).width)
        * 0.5;
    draw_text_ex(
        &da_str,
        pos_x,
        170.0 * params.screen_scale,
        text_params.clone(),
    );
    da_str = "".to_string();
    describe_inhabitants(&mut da_str, &params.hyperspace_planet);
    pos_x = (params.screen_width
        - measure_text(&da_str, Some(&font), 12, params.screen_scale).width)
        * 0.5;
    draw_text_ex(
        &da_str,
        pos_x,
        202.0 * params.screen_scale,
        text_params.clone(),
    );
    da_str = format!("Gross Productivity:{} M CR", hyper_planet_data.productivity,);
    pos_x = (params.screen_width
        - measure_text(&da_str, Some(&font), 12, params.screen_scale).width)
        * 0.5;
    draw_text_ex(
        &da_str,
        pos_x,
        234.0 * params.screen_scale,
        text_params.clone(),
    );
    da_str = format!("Average Radius:{} km", hyper_planet_data.radius);
    draw_text_ex(
        &da_str,
        pos_x,
        266.0 * params.screen_scale,
        text_params.clone(),
    );
    da_str = describe_planet(
        &mut params.hyperspace_planet.clone(),
        cmdr,
        config,
        &mut params.carry_flag,
    );
    let words = da_str.split(" ");
    let mut line = "".to_string();
    let mut y_pos = 298.0;
    let mut count = 0;
    for w in words {
        line += w;
        line += &" ".to_string();
        count += 1;
        if count == 9 {
            pos_x = (params.screen_width
                - measure_text(&line, Some(&font), 12, params.screen_scale).width)
                * 0.5;
            draw_text_ex(
                &line,
                pos_x,
                y_pos * params.screen_scale,
                text_params.clone(),
            );
            count = 0;
            line = "".to_string();
            y_pos += 32.0;
        }
    }
    pos_x = (params.screen_width - measure_text(&line, Some(&font), 12, params.screen_scale).width)
        * 0.5;
    draw_text_ex(
        &line,
        pos_x,
        y_pos * params.screen_scale,
        text_params.clone(),
    );
}
fn describe_inhabitants(da_string: &mut String, planet: &GalaxySeed) {
    let mut inhab;
    *da_string += "(";
    if (planet.e < 128) {
        *da_string += &"Human Colonial";
    } else {
        inhab = (planet.f / 4) & 7;
        if (inhab < 3) {
            *da_string += INHABITANT_DESC1[inhab as usize];
        }
        inhab = planet.f / 32;
        if (inhab < 6) {
            *da_string += INHABITANT_DESC2[inhab as usize];
        }
        inhab = (planet.d ^ planet.b) & 7;
        if (inhab < 6) {
            *da_string += INHABITANT_DESC3[inhab as usize];
        }
        inhab = (inhab + (planet.f & 3)) & 7;
        *da_string += INHABITANT_DESC4[inhab as usize];
    }
    *da_string += "s)";
}
pub fn display_galactic_chart(
    params: &mut GameParams,
    text_params: &mut TextParams,
    font: &Font,
    cmdr: &mut Commander,
) {
    let mut px;
    let mut py;
    params.current_screen = SCR_GALACTIC_CHART;
    let da_str = format!("GALACTIC CHART {}", cmdr.galaxy_number + 1);
    text_params.color = GOLD;
    let pos_x = (params.screen_width
        - measure_text(&da_str, Some(&font), FONT_BASE, params.screen_scale).width)
        * 0.5;
    draw_text_ex(
        &da_str,
        pos_x,
        15.0 * params.screen_scale,
        text_params.clone(),
    );
    draw_line(0.0, 36.0, 511.0, 36.0, THICKNESS, WHITE);
    draw_line(0.0, 36.0 + 258.0, 511.0, 36.0 + 258.0, THICKNESS, WHITE);
    draw_fuel_limit_circle(
        params.docked_planet.d as f32 * params.screen_scale,
        (params.docked_planet.b as f32 / (2.0 / params.screen_scale))
            + (18.0 * params.screen_scale)
            + 1.0,
        params,
        cmdr,
    );
    let mut glx = cmdr.galaxy;
    for i in 0..256 {
        px = glx.d as f32 * params.screen_scale;
        py = (glx.b as f32 / (2.0 / params.screen_scale)) + (18.0 * params.screen_scale) + 1.0;
        draw_rectangle(px, py, 1.0, 1.0, WHITE);
        if ((glx.e | 0x50) < 0x90) {
            draw_rectangle(px + 1.0, py, 1.0, 1.0, WHITE);
        }
        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
    }
    // uof display_galactic_chart
    if is_key_down(KeyCode::Up)
        || is_key_down(KeyCode::S)
        || is_key_down(KeyCode::Down)
        || is_key_down(KeyCode::X)
        || is_key_down(KeyCode::Left)
        || is_key_down(KeyCode::Comma)
        || is_key_down(KeyCode::Right)
        || is_key_down(KeyCode::Period)
    {
    } else {
        params.cross_x = (params.hyperspace_planet.d as f32 * params.screen_scale) as My;
        params.cross_y = ((params.hyperspace_planet.b as f32 / (2.0 / params.screen_scale))
            + (18.0 * params.screen_scale)
            + 1.0) as My;
    }
}

fn laser_type(strength: My) -> String {
    let laser_name: [String; 5] = [
        "Pulse".to_string(),
        "Beam".to_string(),
        "Military".to_string(),
        "Mining".to_string(),
        "Custom".to_string(),
    ];
    if strength == PULSE_LASER {
        laser_name[0].clone()
    } else if strength == BEAM_LASER {
        laser_name[1].clone()
    } else if strength == MILITARY_LASER {
        laser_name[2].clone()
    } else if strength == MINING_LASER {
        laser_name[3].clone()
    } else {
        laser_name[4].clone()
    }
}
pub fn display_commander_status(
    cmdr: &Commander,
    params: &mut GameParams,
    universe: &[UnivObject],
    font: &Font,
    text_params: &mut TextParams,
) {
    let rating: [Rank; NO_OF_RANKS] = [
        Rank {
            score: 0x0000,
            title: "Harmless".to_string(),
        },
        Rank {
            score: 0x0008,
            title: "Mostly Harmless".to_string(),
        },
        Rank {
            score: 0x0010,
            title: "Poor".to_string(),
        },
        Rank {
            score: 0x0020,
            title: "Average".to_string(),
        },
        Rank {
            score: 0x0040,
            title: "Above Average".to_string(),
        },
        Rank {
            score: 0x0080,
            title: "Competent".to_string(),
        },
        Rank {
            score: 0x0200,
            title: "Dangerous".to_string(),
        },
        Rank {
            score: 0x0A00,
            title: "Deadly".to_string(),
        },
        Rank {
            score: 0x1900,
            title: "---- E L I T E ---".to_string(),
        },
    ];
    let mut planet_name: String = "".to_string(); //[16];
    let mut x: f32;
    let mut y: f32;

    let mut condition: usize;
    let mut da_type: DaType;

    params.current_screen = SCR_CMDR_STATUS;

    let mut da_str = "COMMANDER ".to_string();
    for c in cmdr.name {
        da_str += &c.to_string();
    }
    da_str = da_str.trim_end().to_string();
    let msg_width = measure_text(&da_str, Some(font), FONT_BASE, params.screen_scale).width;
    let x_pos = (params.screen_width - msg_width) * 0.5;
    let width_factor = params.screen_width / 324.0;
    let height_factor = params.row_y_pos / 300.0;
    text_params.color = GOLD;

    // text_params for measuring centred
    draw_text_ex(
        da_str,
        x_pos,
        15.0 * params.screen_scale,
        text_params.clone(),
    );
    draw_line(
        0.0,
        25.0 * params.screen_scale,
        params.screen_width,
        25.0 * params.screen_scale,
        THICKNESS,
        WHITE,
    );

    text_params.color = GREEN;
    draw_text_ex(
        "Present System:",
        16.0 * width_factor,
        58.0 * height_factor,
        text_params.clone(),
    );

    text_params.color = WHITE;
    if !params.witchspace {
        name_planet(
            &mut planet_name,
            &mut params.docked_planet.clone(),
            &mut params.carry_flag,
        );
        capitalise_name(&mut planet_name);
        draw_text_ex(
            planet_name,
            128.0 * width_factor,
            58.0 * height_factor,
            text_params.clone(),
        );
    } else {
        draw_text_ex(
            "Hyperspace System:",
            16.0 * width_factor,
            74.0 * height_factor,
            text_params.clone(),
        );
        name_planet(
            &mut planet_name,
            &mut params.hyperspace_planet.clone(),
            &mut params.carry_flag,
        );
        capitalise_name(&mut planet_name);
        draw_text_ex(
            planet_name,
            128.0 * width_factor,
            58.0 * height_factor,
            text_params.clone(),
        );
    }

    if params.docked {
        condition = 0;
    } else {
        condition = 1;

        for uni_object in universe {
            da_type = uni_object.da_type;

            if (da_type == SHIP_MISSILE) || ((da_type > SHIP_ROCK) && (da_type < SHIP_DODEC)) {
                condition = 2;
                break;
            }
        }

        if (condition == 2) && (params.energy < 128) {
            condition = 3;
        }
    }

    text_params.color = GREEN;
    draw_text_ex(
        "Condition:",
        16.0 * width_factor,
        90.0 * height_factor,
        text_params.clone(),
    );
    text_params.color = WHITE;
    draw_text_ex(
        CONDITION_TXT[condition],
        128.0 * width_factor,
        90.0 * height_factor,
        text_params.clone(),
    );

    da_str = format!("{}.{} Light Years", cmdr.fuel / 10, cmdr.fuel % 10);
    text_params.color = GREEN;
    draw_text_ex(
        "Fuel:",
        16.0 * width_factor,
        106.0 * height_factor,
        text_params.clone(),
    );
    text_params.color = WHITE;
    draw_text_ex(
        da_str,
        128.0 * width_factor,
        106.0 * height_factor,
        text_params.clone(),
    );

    da_str = format!("{}.{} Cr", cmdr.credits / 10, cmdr.credits % 10);
    text_params.color = GREEN;
    draw_text_ex(
        "Cash:",
        16.0 * width_factor,
        122.0 * height_factor,
        text_params.clone(),
    );
    text_params.color = WHITE;
    draw_text_ex(
        da_str,
        128.0 * width_factor,
        122.0 * height_factor,
        text_params.clone(),
    );

    if cmdr.legal_status == 0 {
        da_str = "Clean".to_string();
    } else {
        if cmdr.legal_status > 50 {
            da_str = "Fugitive".to_string();
        } else {
            da_str = "Offender".to_string();
        }
    }

    text_params.color = GREEN;
    draw_text_ex(
        "Legal Status:",
        16.0 * width_factor,
        138.0 * height_factor,
        text_params.clone(),
    );
    text_params.color = WHITE;
    draw_text_ex(
        da_str,
        128.0 * width_factor,
        138.0 * height_factor,
        text_params.clone(),
    );

    da_str = rating[0].title.clone();
    for da_rating in rating {
        if cmdr.score >= da_rating.score {
            da_str = da_rating.title.clone();
        }
    }

    text_params.color = GREEN;
    draw_text_ex(
        "Rating:",
        16.0 * width_factor,
        154.0 * height_factor,
        text_params.clone(),
    );
    text_params.color = WHITE;
    draw_text_ex(
        da_str.clone(),
        128.0 * width_factor,
        154.0 * height_factor,
        text_params.clone(),
    );

    text_params.color = GREEN;
    draw_text_ex(
        "EQUIPMENT:",
        16.0 * width_factor,
        186.0 * height_factor,
        text_params.clone(),
    );

    x = EQUIP_START_X;
    y = EQUIP_START_Y;

    text_params.color = WHITE;
    if cmdr.cargo_capacity > 20 {
        draw_text_ex(
            "Large Cargo Bay",
            x * width_factor,
            y * height_factor,
            text_params.clone(),
        );
        y += Y_INC * height_factor;
    }

    if cmdr.escape_pod != 0 {
        draw_text_ex(
            "Escape Pod",
            x * width_factor,
            y * height_factor,
            text_params.clone(),
        );
        y += Y_INC * height_factor;
    }

    if cmdr.fuel_scoop != 0 {
        draw_text_ex(
            "Fuel Scoops",
            x * width_factor,
            y * height_factor,
            text_params.clone(),
        );
        y += Y_INC * height_factor;
    }

    if cmdr.ecm != 0 {
        draw_text_ex(
            "E.C.M. System",
            x * width_factor,
            y * height_factor,
            text_params.clone(),
        );
        y += Y_INC * height_factor;
    }

    if cmdr.energy_bomb != 0 {
        draw_text_ex(
            "Energy Bomb",
            x * width_factor,
            y * height_factor,
            text_params.clone(),
        );
        y += Y_INC * height_factor;
    }

    if cmdr.energy_unit != 0 {
        draw_text_ex(
            if cmdr.energy_unit == 1 {
                "Extra Energy Unit"
            } else {
                "Naval Energy Unit"
            },
            x * width_factor,
            y * height_factor,
            text_params.clone(),
        );
        y += Y_INC * height_factor;
        if y > EQUIP_MAX_Y {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if cmdr.docking_computer != 0 {
        draw_text_ex(
            "Docking Computers",
            x * width_factor,
            y * height_factor,
            text_params.clone(),
        );
        y += Y_INC * height_factor;
        if y > EQUIP_MAX_Y {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if cmdr.galactic_hyperdrive != 0 {
        draw_text_ex(
            "Galactic Hyperspace",
            x * width_factor,
            y * height_factor,
            text_params.clone(),
        );
        y += Y_INC * height_factor;
        if y > EQUIP_MAX_Y {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if cmdr.front_laser != 0 {
        da_str = format!("Front {} Laser", laser_type(cmdr.front_laser));
        draw_text_ex(
            da_str,
            x * width_factor,
            y * height_factor,
            text_params.clone(),
        );
        y += Y_INC * height_factor;
        if y > EQUIP_MAX_Y {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if cmdr.rear_laser != 0 {
        da_str = format!("Rear {} Laser", laser_type(cmdr.rear_laser));
        draw_text_ex(
            da_str,
            x * width_factor,
            y * height_factor,
            text_params.clone(),
        );
        y += Y_INC * height_factor;
        if y > EQUIP_MAX_Y {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if cmdr.left_laser != 0 {
        da_str = format!("Left {} Laser", laser_type(cmdr.left_laser));
        draw_text_ex(
            da_str,
            x * width_factor,
            y * height_factor,
            text_params.clone(),
        );
        y += Y_INC * height_factor;
        if y > EQUIP_MAX_Y {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if cmdr.right_laser != 0 {
        da_str = format!("Right {} Laser", laser_type(cmdr.right_laser));
        draw_text_ex(
            da_str,
            x * width_factor,
            y * height_factor,
            text_params.clone(),
        );
    }
}

fn short_range_screen_to_world(
    p1: f32,
    mid_screen: f32,
    p_factor: f32,
    p2: f32,
    screen_scale: f32,
) -> f32 {
    (p1 - mid_screen) / (p_factor * screen_scale) + p2
}

fn short_range_world_to_screen(
    p1: u8,
    p2: u8,
    p_factor: f32,
    mid_screen: f32,
    screen_scale: f32,
) -> My {
    ((p1 as f32 - p2 as f32) * (p_factor * screen_scale) + mid_screen) as My
}

pub fn show_distance(
    ypos: u16,
    from_planet: &GalaxySeed,
    to_planet: &GalaxySeed,
    dest_planet_string: &mut String,
) {
    let light_years = calc_distance_to_planet(from_planet, to_planet);
    let mut dist_string = "".to_string();
    if (light_years > 0) {
        dist_string = format!(
            "Distance: {}.{} Light Years ",
            light_years / 10,
            light_years % 10,
        );
    } else {
    }
    *dest_planet_string += &dist_string;
}
pub fn move_cursor_to_origin(params: &mut GameParams) {
    if (params.current_screen == SCR_GALACTIC_CHART) {
        params.cross_x = (params.docked_planet.d as f32 * params.screen_scale) as My;
        params.cross_y = (params.docked_planet.b as f32 / (2.0 / params.screen_scale)
            + (18.0 * params.screen_scale)
            + 1.0) as My;
    } else {
        params.cross_x = params.mid_screen_x as My;
        params.cross_y = params.mid_screen_y as My;
    }
    // show_distance_to_planet();
}
pub fn display_market_prices(
    params: &mut GameParams,
    text_params: &mut TextParams,
    font: &Font,
    cmdr: &Commander,
) {
    let mut planet_name: String = "".to_string();
    let mut msg: String = "".to_string();

    params.current_screen = SCR_MARKET_PRICES;

    // gfx_clear_display();
    name_planet(
        &mut planet_name,
        &mut params.docked_planet.clone(),
        &mut params.carry_flag,
    );
    msg = format!("{} MARKET PRICES", planet_name);
    text_params.color = GOLD;
    let msg_width = measure_text(&msg, Some(font), FONT_BASE, params.screen_scale).width;
    let msg_x_pos = (params.screen_width - msg_width) * 0.5;
    draw_text_ex(
        &msg,
        msg_x_pos,
        15.0 * params.screen_scale,
        text_params.clone(),
    );

    draw_line(
        0.0,
        25.0 * params.screen_scale,
        params.screen_width,
        25.0 * params.screen_scale,
        THICKNESS,
        WHITE,
    );

    text_params.color = GREEN;
    draw_text_ex(
        &"PRODUCT",
        16.0,
        55.0 * params.screen_scale,
        text_params.clone(),
    );
    draw_text_ex(
        &"UNIT",
        166.0,
        55.0 * params.screen_scale,
        text_params.clone(),
    );
    draw_text_ex(
        &"PRICE",
        246.0,
        55.0 * params.screen_scale,
        text_params.clone(),
    );
    draw_text_ex(
        &"FOR SALE",
        314.0,
        55.0 * params.screen_scale,
        text_params.clone(),
    );
    draw_text_ex(
        &"IN HOLD",
        420.0,
        55.0 * params.screen_scale,
        text_params.clone(),
    );

    for i in 0..17 {
        display_stock_price(i, cmdr, text_params, params);
    }

    if (params.docked) {
        highlight_stock(cmdr, text_params, params);
    }
}
fn display_stock_price(
    i: usize,
    cmdr: &Commander,
    text_params: &mut TextParams,
    params: &GameParams,
) {
    let mut msg = "".to_string();
    let y = i as f32 * 15.0 * params.screen_scale + 75.0 * params.screen_scale;
    text_params.color = WHITE;
    draw_text_ex(
        &cmdr.stock_market.stock_market[i].name,
        16.0,
        y,
        text_params.clone(),
    );

    draw_text_ex(
        unit_name[cmdr.stock_market.stock_market[i].units],
        180.0,
        y,
        text_params.clone(),
    );
    msg = format!(
        "{}.{}",
        cmdr.stock_market.stock_market[i].current_price / 10,
        cmdr.stock_market.stock_market[i].current_price % 10,
    );
    draw_text_ex(msg, 256.0, y, text_params.clone());

    if (cmdr.stock_market.stock_market[i].current_quantity > 0) {
        msg = format!(
            "{}{}",
            cmdr.stock_market.stock_market[i].current_quantity,
            unit_name[cmdr.stock_market.stock_market[i].units],
        );
    } else {
        msg = "-".to_string();
    }

    draw_text_ex(msg, 338.0, y, text_params.clone());

    if (cmdr.current_cargo[i] > 0) {
        msg = format!(
            "{}{}",
            cmdr.current_cargo[i], unit_name[cmdr.stock_market.stock_market[i].units],
        );
    } else {
        msg = "-".to_string();
    }

    draw_text_ex(msg, 444.0, y, text_params.clone());
}

fn highlight_stock(cmdr: &Commander, text_params: &mut TextParams, params: &GameParams) {
    let mut y: f32;
    let mut msg: String;

    y = params.hilite_item as f32 * 15.0 * params.screen_scale + 75.0 * params.screen_scale;
    y += 15.0 * params.screen_scale * 0.25;
    draw_rectangle(
        2.0 * params.screen_scale,
        y - 15.0 * params.screen_scale,
        510.0 * params.screen_scale,
        15.0 * params.screen_scale,
        RED,
    );
    display_stock_price(params.hilite_item, cmdr, text_params, params);

    msg = format!("Cash: {}.{}", cmdr.credits / 10, cmdr.credits % 10);
    draw_text_ex(msg, 16.0, 340.0 * params.screen_scale, text_params.clone());
}
pub fn select_previous_stock(
    params: &mut GameParams,
    cmdr: &Commander,
    text_params: &mut TextParams,
) {
    if ((!params.docked) || (params.hilite_item == 0)) {
        return;
    }
    params.hilite_item -= 1;
    highlight_stock(cmdr, text_params, params);
}

pub fn select_next_stock(params: &mut GameParams, cmdr: &Commander, text_params: &mut TextParams) {
    if ((!params.docked) || (params.hilite_item == 16)) {
        return;
    }
    params.hilite_item += 1;
    highlight_stock(cmdr, text_params, params);
}

pub fn buy_stock(cmdr: &mut Commander, text_params: &mut TextParams, params: &GameParams) {
    let mut cargo_held: My;

    if (!params.docked) {
        return;
    }

    if ((cmdr.stock_market.stock_market[params.hilite_item].current_quantity == 0)
        || (cmdr.credits < cmdr.stock_market.stock_market[params.hilite_item].current_price))
    {
        return;
    }

    cargo_held = total_cargo(cmdr);

    if ((cmdr.stock_market.stock_market[params.hilite_item].units == TONNES)
        && (cargo_held == cmdr.cargo_capacity))
    {
        return;
    }

    cmdr.current_cargo[params.hilite_item] += 1;
    cmdr.stock_market.stock_market[params.hilite_item].current_quantity -= 1;
    cmdr.credits -= cmdr.stock_market.stock_market[params.hilite_item].current_price;

    highlight_stock(cmdr, text_params, params);
}

pub fn sell_stock(cmdr: &mut Commander, text_params: &mut TextParams, params: &GameParams) {
    let mut item: StockItem;

    if ((!params.docked) || (cmdr.current_cargo[params.hilite_item] == 0)) {
        return;
    }

    cmdr.current_cargo[params.hilite_item] -= 1;
    cmdr.stock_market.stock_market[params.hilite_item].current_quantity += 1;
    cmdr.credits += cmdr.stock_market.stock_market[params.hilite_item].current_price;

    highlight_stock(cmdr, text_params, params);
}
/***********************************************************************************/
