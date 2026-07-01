use macroquad::{
    color::{GOLD, GREEN, WHITE},
    shapes::draw_line,
    text::{Font, TextParams, draw_text, draw_text_ex, measure_text},
};

use crate::{
    BEAM_LASER, GameParams, MILITARY_LASER, MINING_LASER, My, PULSE_LASER, THICKNESS,
    elite::{Commander, SCR_CMDR_STATUS},
    gfx::GFX_SCALE,
    planet::{capitalise_name, name_planet},
    shipdata::{SHIP_DODEC, SHIP_MISSILE, SHIP_ROCK},
    space::{DaType, UnivObject},
};

struct Rank {
    score: My,
    title: String,
}

const NO_OF_RANKS: usize = 9;

const EQUIP_START_Y: f32 = 202.0;
const EQUIP_START_X: f32 = 50.0;
const EQUIP_MAX_Y: f32 = 290.0;
const EQUIP_WIDTH: f32 = 200.0;
const Y_INC: f32 = 16.0;

const CONDITION_TXT: [&str; 4] = ["Docked", "Green", "Yellow", "Red"];
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
            &mut params.docked_planet,
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
            &mut params.hyperspace_planet,
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
