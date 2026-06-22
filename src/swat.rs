use macroquad::{
    color::{RED, WHITE},
    shapes::{draw_line, draw_triangle},
};

use crate::{
    Config, GameParams, THICKNESS,
    elite::Commander,
    gfx::{GFX_SCALE, GFX_VIEW_BY},
    sound::SND_PULSE,
    stars::rand255,
};

pub const MISSILE_UNARMED: i16 = -2;
pub const MISSILE_ARMED: i16 = -1;
pub struct Swat {
    ecm_active: i16,
    missile_target: i16,
    in_battle: i16,
}

impl Swat {
    pub fn new() -> Self {
        Self {
            ecm_active: 0,
            missile_target: MISSILE_UNARMED,
            in_battle: 0,
        }
    }

    pub fn set(ecm_active: i16, missile_target: i16, in_battle: i16) -> Self {
        Self {
            ecm_active,
            missile_target,
            in_battle,
        }
    }
}
pub fn reset_weapons(params: &mut GameParams) {
    params.myship.laser_temp = 0;
    params.myship.laser_counter = 0;
    params.myship.laser = 0;
    params.myship.ecm_active = 0;
    params.myship.missile_target = MISSILE_UNARMED;
}
pub fn draw_laser_lines(params: &GameParams, config: &Config) {
    if config.wireframe != 0 {
        draw_line(
            32.0 * GFX_SCALE as f32,
            GFX_VIEW_BY as f32,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            THICKNESS,
            WHITE,
        );
        draw_line(
            48.0 * GFX_SCALE as f32,
            GFX_VIEW_BY as f32,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            THICKNESS,
            WHITE,
        );
        draw_line(
            208.0 * GFX_SCALE as f32,
            GFX_VIEW_BY as f32,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            THICKNESS,
            WHITE,
        );
        draw_line(
            224.0 * GFX_SCALE as f32,
            GFX_VIEW_BY as f32,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            THICKNESS,
            WHITE,
        );
    } else {
        /*
        draw_triangle(
            32 * GFX_SCALE,
            GFX_VIEW_BY,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            48 * GFX_SCALE,
            GFX_VIEW_BY,
            RED,
        );
        draw_triangle(
            208 * GFX_SCALE,
            GFX_VIEW_BY,
            params.myship.laser_x as f32,
            params.myship.laser_y as f32,
            224 * GFX_SCALE,
            GFX_VIEW_BY,
            RED,
        );
        */
    }
}
pub fn fire_laser(params: &mut GameParams, cmdr: &mut Commander) -> i16 {
    if (params.myship.laser_counter == 0) && (params.myship.laser_temp < 242) {
        match params.current_screen {
            SCR_FRONT_VIEW => {
                params.myship.laser = cmdr.front_laser;
            }

            SCR_REAR_VIEW => {
                params.myship.laser = cmdr.rear_laser;
            }

            SCR_RIGHT_VIEW => {
                params.myship.laser = cmdr.right_laser;
            }

            SCR_LEFT_VIEW => {
                params.myship.laser = cmdr.left_laser;
            }

            _ => {
                params.myship.laser = 0;
            }
        }

        if params.myship.laser != 0 {
            params.myship.laser_counter = if params.myship.laser > 127 {
                0
            } else {
                params.myship.laser & 0xFA
            };
            params.myship.laser &= 127;
            // params.myship.laser2 = params.myship.laser;

            snd_play_sample(SND_PULSE);
            params.myship.laser_temp += 8;
            if params.energy > 1 {
                params.energy -= 1;
            }

            params.myship.laser_x = ((rand255() & 3) + 128 - 2) * GFX_SCALE as u8;
            params.myship.laser_y = ((rand255() & 3) + 96 - 2) * GFX_SCALE as u8;

            return 2;
        }
    }

    return 0;
}

pub fn snd_play_sample(snd_pulse: usize) {
    println!("snd_play_sample()")
}
