use macroquad::{
    color::{GOLD, GREEN, WHITE},
    shapes::draw_line,
    text::{draw_text, draw_text_ex},
};

use crate::{
    elite::{Commander, MAX_UNIV_OBJECTS, SCR_CMDR_STATUS},
    gfx::GFX_SCALE,
    planet::{capitalise_name, name_planet},
    shipdata::{SHIP_DODEC, SHIP_MISSILE, SHIP_ROCK},
    space::UnivObject,
    GameParams, My, THICKNESS,
};

struct Rank {
    score: My,
    title: String,
}

const NO_OF_RANKS: usize = 9;

const EQUIP_START_Y: My = 202;
const EQUIP_START_X: My = 50;
const EQUIP_MAX_Y: My = 290;
const EQUIP_WIDTH: My = 200;
const Y_INC: My = 16;

const CONDITION_TXT: [&str; 4] = ["Docked", "Green", "Yellow", "Red"];
pub fn display_commander_status(
    cmdr: &Commander,
    params: &mut GameParams,
    universe: &[UnivObject],
) {
    fn laser_type(strength: My) -> String {
        let laser_name: [String; 5] = [
            "Pulse".to_string(),
            "Beam".to_string(),
            "Military".to_string(),
            "Mining".to_string(),
            "Custom".to_string(),
        ];
        match (strength) {
            PULSE_LASER => laser_name[0].clone(),

            BEAM_LASER => laser_name[1].clone(),

            MILITARY_LASER => laser_name[2].clone(),

            MINING_LASER => laser_name[3].clone(),
            _ => laser_name[4].clone(),
        }
    }
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
    let mut da_str: String = "".to_string(); //[100];
    let mut x: My;
    let mut y: My;

    let mut condition: usize;
    let mut da_type: usize;

    params.current_screen = SCR_CMDR_STATUS;

    da_str = "COMMANDER {}".to_string();
    for c in cmdr.name {
        da_str += &c.to_string();
    }
    da_str = da_str.trim_end().to_string();

    draw_text(
        da_str,
        100.0 * GFX_SCALE as f32,
        10.0 * GFX_SCALE as f32,
        12.0 * GFX_SCALE as f32,
        GOLD,
    ); // should be centred

    draw_line(0.0, 36.0, 511.0, 36.0, THICKNESS, WHITE);

    draw_text(
        "Present System:",
        16.0 * GFX_SCALE as f32,
        58.0 * GFX_SCALE as f32,
        10.0 * GFX_SCALE as f32,
        GREEN,
    );

    if (!params.witchspace) {
        name_planet(
            &mut planet_name,
            &mut params.docked_planet,
            &mut params.carry_flag,
        );
        capitalise_name(&mut planet_name);
        draw_text(
            planet_name,
            190.0 * GFX_SCALE as f32,
            58.0 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
            WHITE,
        );
    } else {
        draw_text(
            "Hyperspace System:",
            16.0 * GFX_SCALE as f32,
            74.0 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
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
            190.0 * GFX_SCALE as f32,
            58.0 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
            WHITE,
        );
    }

    if (params.docked) {
        condition = 0;
    } else {
        condition = 1;

        for i in 0..MAX_UNIV_OBJECTS {
            da_type = universe[i].da_type;

            if ((da_type == SHIP_MISSILE) || ((da_type > SHIP_ROCK) && (da_type < SHIP_DODEC))) {
                condition = 2;
                break;
            }
        }

        if ((condition == 2) && (params.energy < 128)) {
            condition = 3;
        }
    }

    draw_text(
        "Condition",
        16.0 * GFX_SCALE as f32,
        90.0 * GFX_SCALE as f32,
        10.0 * GFX_SCALE as f32,
        GREEN,
    );
    draw_text(
        CONDITION_TXT[condition],
        190.0 * GFX_SCALE as f32,
        90.0 * GFX_SCALE as f32,
        10.0 * GFX_SCALE as f32,
        WHITE,
    );

    da_str = format!("{},{} Light Years", cmdr.fuel / 10, cmdr.fuel % 10);
    draw_text(
        "Fuel:",
        16.0 * GFX_SCALE as f32,
        106.0 * GFX_SCALE as f32,
        10.0 * GFX_SCALE as f32,
        GREEN,
    );
    draw_text(
        da_str,
        70.0 * GFX_SCALE as f32,
        106.0 * GFX_SCALE as f32,
        10.0 * GFX_SCALE as f32,
        WHITE,
    );

    da_str = format!("{}.{} Cr", cmdr.credits / 10, cmdr.credits % 10);
    draw_text(
        "Cash:",
        16.0 * GFX_SCALE as f32,
        122.0 * GFX_SCALE as f32,
        10.0 * GFX_SCALE as f32,
        GREEN,
    );
    draw_text(
        da_str,
        70.0 * GFX_SCALE as f32,
        122.0 * GFX_SCALE as f32,
        10.0 * GFX_SCALE as f32,
        WHITE,
    );

    if (cmdr.legal_status == 0) {
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
        16.0 * GFX_SCALE as f32,
        138.0 * GFX_SCALE as f32,
        10.0 * GFX_SCALE as f32,
        GREEN,
    );
    draw_text(
        da_str,
        128.0 * GFX_SCALE as f32,
        138.0 * GFX_SCALE as f32,
        10.0 * GFX_SCALE as f32,
        WHITE,
    );

    da_str = rating[0].title.clone();
    for i in 0..NO_OF_RANKS {
        if (cmdr.score >= rating[i].score) {
            da_str = rating[i].title.clone();
        }
    }

    draw_text(
        "Rating:",
        16.0 * GFX_SCALE as f32,
        154.0 * GFX_SCALE as f32,
        10.0 * GFX_SCALE as f32,
        GREEN,
    );
    draw_text(
        da_str.clone(),
        80.0 * GFX_SCALE as f32,
        154.0 * GFX_SCALE as f32,
        10.0 * GFX_SCALE as f32,
        WHITE,
    );

    draw_text(
        "EQUIPMENT:",
        16.0 * GFX_SCALE as f32,
        186.0 * GFX_SCALE as f32,
        10.0 * GFX_SCALE as f32,
        GREEN,
    );

    x = EQUIP_START_X;
    y = EQUIP_START_Y;

    if (cmdr.cargo_capacity > 20) {
        draw_text(
            "Large Cargo Bay",
            x as f32 * GFX_SCALE as f32,
            y as f32 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
            WHITE,
        );
        y += Y_INC * GFX_SCALE;
    }

    if (cmdr.escape_pod != 0) {
        draw_text(
            "Escape Pod",
            x as f32 * GFX_SCALE as f32,
            y as f32 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
            WHITE,
        );
        y += Y_INC * GFX_SCALE;
    }

    if (cmdr.fuel_scoop != 0) {
        draw_text(
            "Fuel Scoops",
            x as f32 * GFX_SCALE as f32,
            y as f32 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
            WHITE,
        );
        y += Y_INC * GFX_SCALE;
    }

    if (cmdr.ecm != 0) {
        draw_text(
            "E.C.M. System",
            x as f32 * GFX_SCALE as f32,
            y as f32 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
            WHITE,
        );
        y += Y_INC * GFX_SCALE;
    }

    if (cmdr.energy_bomb != 0) {
        draw_text(
            "Energy Bomb",
            x as f32 * GFX_SCALE as f32,
            y as f32 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
            WHITE,
        );
        y += Y_INC * GFX_SCALE;
    }

    if (cmdr.energy_unit != 0) {
        draw_text(
            if cmdr.energy_unit == 1 {
                "Extra Energy Unit"
            } else {
                "Naval Energy Unit"
            },
            x as f32 * GFX_SCALE as f32,
            y as f32 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
            WHITE,
        );
        y += Y_INC * GFX_SCALE;
        if (y > EQUIP_MAX_Y) {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if (cmdr.docking_computer != 0) {
        draw_text(
            "Docking Computers",
            x as f32 * GFX_SCALE as f32,
            y as f32 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
            WHITE,
        );
        y += Y_INC * GFX_SCALE;
        if (y > EQUIP_MAX_Y) {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if (cmdr.galactic_hyperdrive != 0) {
        draw_text(
            "Galactic Hyperspace",
            x as f32 * GFX_SCALE as f32,
            y as f32 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
            WHITE,
        );
        y += Y_INC * GFX_SCALE;
        if (y > EQUIP_MAX_Y) {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if (cmdr.front_laser != 0) {
        da_str = format!("Front {} Laser", laser_type(cmdr.front_laser));
        draw_text(
            da_str,
            x as f32 * GFX_SCALE as f32,
            y as f32 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
            WHITE,
        );
        y += Y_INC * GFX_SCALE;
        if (y > EQUIP_MAX_Y) {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if (cmdr.rear_laser != 0) {
        da_str = format!("Rear {} Laser", laser_type(cmdr.rear_laser));
        draw_text(
            da_str,
            x as f32 * GFX_SCALE as f32,
            y as f32 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
            WHITE,
        );
        y += Y_INC * GFX_SCALE;
        if (y > EQUIP_MAX_Y) {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if (cmdr.left_laser != 0) {
        da_str = format!("Left {} Laser", laser_type(cmdr.left_laser));
        draw_text(
            da_str,
            x as f32 * GFX_SCALE as f32,
            y as f32 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
            WHITE,
        );
        y += Y_INC * GFX_SCALE;
        if (y > EQUIP_MAX_Y) {
            y = EQUIP_START_Y;
            x += EQUIP_WIDTH;
        }
    }

    if (cmdr.right_laser != 0) {
        da_str = format!("Right {} Laser", laser_type(cmdr.right_laser));
        draw_text(
            da_str,
            x as f32 * GFX_SCALE as f32,
            y as f32 * GFX_SCALE as f32,
            10.0 * GFX_SCALE as f32,
            WHITE,
        );
    }
}

/***********************************************************************************/
