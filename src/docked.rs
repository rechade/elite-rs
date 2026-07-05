use macroquad::{
    color::{GOLD, GREEN, WHITE},
    shapes::{draw_circle, draw_circle_lines, draw_line, draw_rectangle},
    text::{Font, TextParams, draw_text, draw_text_ex, measure_text},
};

use crate::{
    BEAM_LASER, Config, GameParams, MILITARY_LASER, MINING_LASER, My, PULSE_LASER, THICKNESS,
    elite::{Commander, SCR_CMDR_STATUS, SCR_GALACTIC_CHART, SCR_PLANET_DATA, SCR_SHORT_RANGE},
    gfx::{GFX_SCALE, GFX_X_CENTRE, GFX_Y_CENTRE},
    planet::{
        GalaxySeed, PlanetData, capitalise_name, describe_planet, generate_planet_data,
        name_planet, waggle_galaxy,
    },
    shipdata::{SHIP_DODEC, SHIP_MISSILE, SHIP_ROCK},
    space::{DaType, UnivObject},
};

struct Rank {
    score: My,
    title: String,
}

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
    // struct galaxy_seed glx;
    let mut dx;
    let mut dy;
    let mut px;
    let mut py;
    let mut planet_name: String = "".to_string();
    let mut row_used: [i16; 64] = [0; 64];
    let mut row;
    let mut blob_size;

    params.current_screen = SCR_SHORT_RANGE;

    let da_str = format!("SHORT RANGE CHART");

    let mut text_params_clone = text_params.clone();
    text_params_clone.color = GOLD;
    text_params_clone.font_size = 12;
    let pos_x =
        (params.screen_width - measure_text(&da_str, Some(&font), 12, GFX_SCALE).width) * 0.5;
    draw_text_ex(&da_str, pos_x, 140.0, text_params_clone.clone());
    draw_text_ex(&"SHORT RANGE CHART", pos_x, 140.0, text_params_clone);

    draw_line(0.0, 36.0, 511.0, 36.0, THICKNESS, WHITE);

    draw_fuel_limit_circle(GFX_X_CENTRE, GFX_Y_CENTRE, params, cmdr);

    for i in 0..64 {
        row_used[i] = 0;
    }

    let mut glx = cmdr.galaxy_seed;

    for i in 0..256 {
        dx = (glx.d as My - params.docked_planet.d as My).abs();
        dy = (glx.b as My - params.docked_planet.b as My).abs();

        if ((dx >= 20) || (dy >= 38)) {
            waggle_galaxy(&mut glx, &mut params.carry_flag);
            waggle_galaxy(&mut glx, &mut params.carry_flag);
            waggle_galaxy(&mut glx, &mut params.carry_flag);
            waggle_galaxy(&mut glx, &mut params.carry_flag);
            continue;
        }

        px = (glx.d as f32 - params.docked_planet.d as f32);
        px = px * 4.0 * GFX_SCALE + GFX_X_CENTRE; /* Convert to screen co-ords */

        py = (glx.b as f32 - params.docked_planet.b as f32);
        py = py * 2.0 * GFX_SCALE + GFX_Y_CENTRE; /* Convert to screen co-ords */

        row = (py / (8.0 * GFX_SCALE)) as usize;

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

        if (row_used[row] == 0) {
            row_used[row] = 1;

            name_planet(&mut planet_name, &mut glx.clone(), &mut params.carry_flag);
            capitalise_name(&mut planet_name);

            draw_text_ex(
                &planet_name,
                px + (4.0 * GFX_SCALE),
                (row * 8 - 5) as f32 * GFX_SCALE,
                text_params.clone(),
            );
        }

        /* The next bit calculates the size of the circle used to represent */
        /* a planet.  The carry_flag is left over from the name generation. */
        /* Yes this was how it was done... don't ask :-( */

        blob_size = (glx.f & 1) as f32 + 2.0 + params.carry_flag as f32;
        blob_size *= GFX_SCALE;
        draw_circle(px, py, blob_size, GOLD);

        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
    }

    // xyz
    params.cross_x =
        (((params.hyperspace_planet.d as f32 - params.docked_planet.d as f32) * 4.0 * GFX_SCALE)
            + GFX_X_CENTRE) as My;
    params.cross_y =
        (((params.hyperspace_planet.b as f32 - params.docked_planet.b as f32) * 2.0 * GFX_SCALE)
            + GFX_Y_CENTRE) as My;
}
fn draw_fuel_limit_circle(cx: f32, cy: f32, params: &mut GameParams, cmdr: &mut Commander) {
    let radius;
    let cross_size;

    if (params.current_screen == SCR_GALACTIC_CHART) {
        radius = cmdr.fuel as f32 / 4.0 * GFX_SCALE;
        cross_size = 7.0 * GFX_SCALE;
    } else {
        radius = cmdr.fuel as f32 * GFX_SCALE;
        cross_size = 16.0 * GFX_SCALE;
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
    text_params: &TextParams,
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

    let mut text_params_clone = text_params.clone();
    text_params_clone.color = GOLD;
    text_params_clone.font_size = 12;
    let mut pos_x =
        (params.screen_width - measure_text(&da_str, Some(&font), 12, GFX_SCALE).width) * 0.5;
    draw_text_ex(&da_str, pos_x, 140.0, text_params.clone());

    draw_line(0.0, 36.0, 511.0, 36.0, THICKNESS, WHITE);

    hyper_planet_data = generate_planet_data(&params.hyperspace_planet);

    // crst
    // show_distance(42, params.docked_planet, params.hyperspace_planet);

    da_str = format!(
        "Economy:{}",
        ECONOMY_TYPE[hyper_planet_data.economy as usize]
    );
    pos_x = (params.screen_width - measure_text(&da_str, Some(&font), 12, GFX_SCALE).width) * 0.5;
    draw_text_ex(&da_str, pos_x, 74.0, text_params_clone.clone());

    da_str = format!(
        "Government:{}",
        GOVERNMENT_TYPE[hyper_planet_data.government as usize],
    );
    draw_text_ex(&da_str, 16.0, 106.0, text_params_clone.clone());

    da_str = format!("Tech.Level:{}", hyper_planet_data.techlevel + 1);
    pos_x = (params.screen_width - measure_text(&da_str, Some(&font), 12, GFX_SCALE).width) * 0.5;
    draw_text_ex(&da_str, pos_x, 138.0, text_params_clone.clone());

    da_str = format!(
        "Population:{}.{} Billion",
        hyper_planet_data.population / 10,
        hyper_planet_data.population % 10,
    );
    pos_x = (params.screen_width - measure_text(&da_str, Some(&font), 12, GFX_SCALE).width) * 0.5;
    draw_text_ex(&da_str, pos_x, 170.0, text_params_clone.clone());

    describe_inhabitants(&mut da_str, &params.hyperspace_planet);
    pos_x = (params.screen_width - measure_text(&da_str, Some(&font), 12, GFX_SCALE).width) * 0.5;
    draw_text_ex(&da_str, pos_x, 202.0, text_params_clone.clone());

    da_str = format!("Gross Productivity:{} M CR", hyper_planet_data.productivity,);
    pos_x = (params.screen_width - measure_text(&da_str, Some(&font), 12, GFX_SCALE).width) * 0.5;
    draw_text_ex(&da_str, pos_x, 234.0, text_params_clone.clone());

    da_str = format!("Average Radius:{} km", hyper_planet_data.radius);
    draw_text_ex(&da_str, pos_x, 266.0, text_params_clone.clone());

    da_str = describe_planet(
        &mut params.hyperspace_planet.clone(),
        cmdr,
        config,
        &mut params.carry_flag,
    );
    draw_text_ex(&da_str, pos_x, 298.0, text_params_clone.clone());
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
    text_params: &TextParams,
    font: &Font,
    cmdr: &mut Commander,
) {
    let mut px;
    let mut py;

    params.current_screen = SCR_GALACTIC_CHART;

    let da_str = format!("GALACTIC CHART {}", cmdr.galaxy_number + 1);

    let mut text_params_clone = text_params.clone();
    text_params_clone.color = GOLD;
    text_params_clone.font_size = 12;
    let pos_x =
        (params.screen_width - measure_text(&da_str, Some(&font), 12, GFX_SCALE).width) * 0.5;
    draw_text_ex(&da_str, pos_x, 140.0, text_params_clone);

    draw_line(0.0, 36.0, 511.0, 36.0, THICKNESS, WHITE);
    draw_line(0.0, 36.0 + 258.0, 511.0, 36.0 + 258.0, THICKNESS, WHITE);

    draw_fuel_limit_circle(
        params.docked_planet.d as f32 * GFX_SCALE,
        (params.docked_planet.b as f32 / (2.0 / GFX_SCALE)) + (18.0 * GFX_SCALE) + 1.0,
        params,
        cmdr,
    );

    let mut glx = cmdr.galaxy_seed;

    for i in 0..256 {
        px = glx.d as f32 * GFX_SCALE;
        py = (glx.b as f32 / (2.0 / GFX_SCALE)) + (18.0 * GFX_SCALE) + 1.0;
        draw_rectangle(px, py, 1.0, 1.0, WHITE);
        if ((glx.e | 0x50) < 0x90) {
            draw_rectangle(px + 1.0, py, 1.0, 1.0, WHITE);
        }
        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
        waggle_galaxy(&mut glx, &mut params.carry_flag);
    }
    // xyz
    params.cross_x = (params.hyperspace_planet.d as f32 * GFX_SCALE) as My;
    params.cross_y =
        ((params.hyperspace_planet.b as f32 / (2.0 / GFX_SCALE)) + (18.0 * GFX_SCALE) + 1.0) as My;
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
    let msg_width = measure_text(&da_str, Some(font), 12, GFX_SCALE).width;
    let x_pos = (params.screen_width - msg_width) * 0.5;
    let width_factor = params.screen_width / 324.0;
    let height_factor = params.row_y_pos / 300.0;
    let pointsize_factor = height_factor;
    let mut clone_params = text_params.clone();
    clone_params.font_size = (clone_params.font_size as f32 * pointsize_factor) as u16;
    clone_params.color = GOLD;

    // text_params for measuring centred
    draw_text_ex(da_str, x_pos, 10.0 * height_factor, clone_params);

    draw_line(
        0.0,
        36.0 * height_factor,
        params.screen_width,
        36.0 * height_factor,
        THICKNESS,
        WHITE,
    );

    draw_text(
        "Present System:",
        16.0 * width_factor,
        58.0 * height_factor,
        10.0 * pointsize_factor,
        GREEN,
    );

    if !params.witchspace {
        name_planet(
            &mut planet_name,
            &mut params.docked_planet.clone(),
            &mut params.carry_flag,
        );
        capitalise_name(&mut planet_name);
        draw_text(
            planet_name,
            128.0 * width_factor,
            58.0 * height_factor,
            10.0 * pointsize_factor,
            WHITE,
        );
    } else {
        draw_text(
            "Hyperspace System:",
            16.0 * width_factor,
            74.0 * height_factor,
            10.0 * pointsize_factor,
            WHITE,
        );
        name_planet(
            &mut planet_name,
            &mut params.hyperspace_planet.clone(),
            &mut params.carry_flag,
        );
        capitalise_name(&mut planet_name);
        draw_text(
            planet_name,
            128.0 * width_factor,
            58.0 * height_factor,
            10.0 * pointsize_factor,
            WHITE,
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

    draw_text(
        "Condition:",
        16.0 * width_factor,
        90.0 * height_factor,
        10.0 * pointsize_factor,
        GREEN,
    );
    draw_text(
        CONDITION_TXT[condition],
        128.0 * width_factor,
        90.0 * height_factor,
        10.0 * pointsize_factor,
        WHITE,
    );

    da_str = format!("{},{} Light Years", cmdr.fuel / 10, cmdr.fuel % 10);
    draw_text(
        "Fuel:",
        16.0 * width_factor,
        106.0 * height_factor,
        10.0 * pointsize_factor,
        GREEN,
    );
    draw_text(
        da_str,
        128.0 * width_factor,
        106.0 * height_factor,
        10.0 * pointsize_factor,
        WHITE,
    );

    da_str = format!("{}.{} Cr", cmdr.credits / 10, cmdr.credits % 10);
    draw_text(
        "Cash:",
        16.0 * width_factor,
        122.0 * height_factor,
        10.0 * pointsize_factor,
        GREEN,
    );
    draw_text(
        da_str,
        128.0 * width_factor,
        122.0 * height_factor,
        10.0 * pointsize_factor,
        WHITE,
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

    draw_text(
        "Legal Status:",
        16.0 * width_factor,
        138.0 * height_factor,
        10.0 * pointsize_factor,
        GREEN,
    );
    draw_text(
        da_str,
        128.0 * width_factor,
        138.0 * height_factor,
        10.0 * pointsize_factor,
        WHITE,
    );

    da_str = rating[0].title.clone();
    for da_rating in rating {
        if cmdr.score >= da_rating.score {
            da_str = da_rating.title.clone();
        }
    }

    draw_text(
        "Rating:",
        16.0 * width_factor,
        154.0 * height_factor,
        10.0 * pointsize_factor,
        GREEN,
    );
    draw_text(
        da_str.clone(),
        128.0 * width_factor,
        154.0 * height_factor,
        10.0 * pointsize_factor,
        WHITE,
    );

    draw_text(
        "EQUIPMENT:",
        16.0 * width_factor,
        186.0 * height_factor,
        10.0 * pointsize_factor,
        GREEN,
    );

    x = EQUIP_START_X;
    y = EQUIP_START_Y;

    if cmdr.cargo_capacity > 20 {
        draw_text(
            "Large Cargo Bay",
            x * width_factor,
            y * height_factor,
            10.0 * pointsize_factor,
            WHITE,
        );
        y += Y_INC * height_factor;
    }

    if cmdr.escape_pod != 0 {
        draw_text(
            "Escape Pod",
            x * width_factor,
            y * height_factor,
            10.0 * pointsize_factor,
            WHITE,
        );
        y += Y_INC * height_factor;
    }

    if cmdr.fuel_scoop != 0 {
        draw_text(
            "Fuel Scoops",
            x * width_factor,
            y * height_factor,
            10.0 * pointsize_factor,
            WHITE,
        );
        y += Y_INC * height_factor;
    }

    if cmdr.ecm != 0 {
        draw_text(
            "E.C.M. System",
            x * width_factor,
            y * height_factor,
            10.0 * pointsize_factor,
            WHITE,
        );
        y += Y_INC * height_factor;
    }

    if cmdr.energy_bomb != 0 {
        draw_text(
            "Energy Bomb",
            x * width_factor,
            y * height_factor,
            10.0 * pointsize_factor,
            WHITE,
        );
        y += Y_INC * height_factor;
    }

    if cmdr.energy_unit != 0 {
        draw_text(
            if cmdr.energy_unit == 1 {
                "Extra Energy Unit"
            } else {
                "Naval Energy Unit"
            },
            x * width_factor,
            y * height_factor,
            10.0 * pointsize_factor,
            WHITE,
        );
        y += Y_INC * height_factor;
        if y > EQUIP_MAX_Y {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if cmdr.docking_computer != 0 {
        draw_text(
            "Docking Computers",
            x * width_factor,
            y * height_factor,
            10.0 * pointsize_factor,
            WHITE,
        );
        y += Y_INC * height_factor;
        if y > EQUIP_MAX_Y {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if cmdr.galactic_hyperdrive != 0 {
        draw_text(
            "Galactic Hyperspace",
            x * width_factor,
            y * height_factor,
            10.0 * pointsize_factor,
            WHITE,
        );
        y += Y_INC * height_factor;
        if y > EQUIP_MAX_Y {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if cmdr.front_laser != 0 {
        da_str = format!("Front {} Laser", laser_type(cmdr.front_laser));
        draw_text(
            da_str,
            x * width_factor,
            y * height_factor,
            10.0 * pointsize_factor,
            WHITE,
        );
        y += Y_INC * height_factor;
        if y > EQUIP_MAX_Y {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if cmdr.rear_laser != 0 {
        da_str = format!("Rear {} Laser", laser_type(cmdr.rear_laser));
        draw_text(
            da_str,
            x * width_factor,
            y * height_factor,
            10.0 * pointsize_factor,
            WHITE,
        );
        y += Y_INC * height_factor;
        if y > EQUIP_MAX_Y {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if cmdr.left_laser != 0 {
        da_str = format!("Left {} Laser", laser_type(cmdr.left_laser));
        draw_text(
            da_str,
            x * width_factor,
            y * height_factor,
            10.0 * pointsize_factor,
            WHITE,
        );
        y += Y_INC * height_factor;
        if y > EQUIP_MAX_Y {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if cmdr.right_laser != 0 {
        da_str = format!("Right {} Laser", laser_type(cmdr.right_laser));
        draw_text(
            da_str,
            x * width_factor,
            y * height_factor,
            10.0 * pointsize_factor,
            WHITE,
        );
    }
}

/***********************************************************************************/
