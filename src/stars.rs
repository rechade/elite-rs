use std::i64;

use macroquad::{
    color::WHITE,
    prelude::rand,
    shapes::{draw_line, draw_rectangle},
};

use crate::{
    elite::SCR_FRONT_VIEW,
    gfx::{GFX_SCALE, GFX_VIEW_BX, GFX_VIEW_BY, GFX_VIEW_TX, GFX_VIEW_TY, STAR_SIZE},
    GameParams, SCR_LEFT_VIEW, THICKNESS, *,
};

pub struct Stars {
    warp_stars: bool,
    stars: [Star; 20],
}

impl Stars {
    pub fn new() -> Self {
        Self {
            warp_stars: false,
            stars: [Star::new(); 20],
        }
    }
}
#[derive(Copy, Clone)]
struct Star {
    x: f32,
    y: f32,
    z: f32,
}

impl Star {
    fn set(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

pub fn create_new_stars(da_stars: &mut Stars, params: &GameParams) {
    let nstars = {
        if params.witchspace {
            3
        } else {
            12
        }
    };

    for i in 0..nstars {
        da_stars.stars[i].x = ((rand255() - 128) | 8) as f32;
        da_stars.stars[i].y = ((rand255() - 128) | 4) as f32;
        da_stars.stars[i].z = (rand255() | 0x90) as f32;
    }
    da_stars.warp_stars = false;
}

fn front_starfield(da_stars: &mut Stars, params: &GameParams) {
    let mut q: f32;
    let mut xx: f32;
    let mut yy: f32;
    let mut zz: f32;
    let mut sx: My;
    let mut sy: My;

    let nstars = {
        if params.witchspace {
            3
        } else {
            12
        }
    };

    let mut delta = if da_stars.warp_stars {
        50.0
    } else {
        params.flight_speed as f32
    };
    let mut alpha = params.flight_roll as f32;
    let beta = params.flight_climb as f32;

    alpha /= 256.0;
    delta /= 2.0;

    for i in 0..nstars {
        /* Plot the stars in their current locations... */

        sy = (da_stars.stars[i].y) as My;
        sx = (da_stars.stars[i].x) as My;
        zz = da_stars.stars[i].z;

        sx += 128;
        sy += 96;

        sx *= GFX_SCALE;
        sy *= GFX_SCALE;

        if (!da_stars.warp_stars)
            && (sx >= GFX_VIEW_TX)
            && (sx <= GFX_VIEW_BX)
            && (sy >= GFX_VIEW_TY)
            && (sy <= GFX_VIEW_BY)
        {
            draw_rectangle(sx as f32, sy as f32, STAR_SIZE, STAR_SIZE, WHITE);

            if zz < 0xC0 as f32 {
                draw_rectangle(sx as f32 + 1.0, sy as f32, STAR_SIZE, STAR_SIZE, WHITE);
            }

            if zz < 0x90 as f32 {
                draw_rectangle(sx as f32, sy as f32 + 1.0, STAR_SIZE, STAR_SIZE, WHITE);
                draw_rectangle(
                    sx as f32 + 1.0,
                    sy as f32 + 1.0,
                    STAR_SIZE,
                    STAR_SIZE,
                    WHITE,
                );
            }
        }

        /* Move the stars to their new locations...*/

        q = delta / da_stars.stars[i].z;

        da_stars.stars[i].z -= delta;
        yy = da_stars.stars[i].y + (da_stars.stars[i].y * q);
        xx = da_stars.stars[i].x + (da_stars.stars[i].x * q);
        zz = da_stars.stars[i].z;

        yy = yy + (xx * alpha);
        xx = xx - (yy * alpha);

        /*
                tx = yy * beta;
                xx = xx + (tx * tx * 2);
        */
        yy = yy + beta;

        da_stars.stars[i].y = yy;
        da_stars.stars[i].x = xx;

        if da_stars.warp_stars {
            draw_line(
                sx as f32,
                sy as f32,
                (xx + 128.0) * GFX_SCALE as f32,
                (yy + 96.0) * GFX_SCALE as f32,
                THICKNESS,
                WHITE,
            );
        }

        sx = xx as My;
        sy = yy as My;

        if (sx > 120) || (sx < -120) || (sy > 120) || (sy < -120) || (zz < 16.0) {
            da_stars.stars[i].x = ((rand255() - 128) | 8) as f32;
            da_stars.stars[i].y = ((rand255() - 128) | 4) as f32;
            da_stars.stars[i].z = (rand255() | 0x90) as f32;
            continue;
        }
    }

    da_stars.warp_stars = false;
}

fn rear_starfield(da_stars: &mut Stars, params: &GameParams) {
    let mut q: f32;
    let mut xx: f32;
    let mut yy: f32;
    let mut zz: f32;
    let mut sx: My;
    let mut sy: My;
    let mut ex: My;
    let mut ey: My;

    let nstars = {
        if params.witchspace {
            3
        } else {
            12
        }
    };

    let mut delta = if da_stars.warp_stars {
        50.0
    } else {
        params.flight_speed as f32
    };
    let mut alpha = -params.flight_roll as f32;
    let beta = -params.flight_climb as f32;

    alpha /= 256.0;
    delta /= 2.0;

    for i in 0..nstars {
        /* Plot the stars in their current locations... */

        sy = da_stars.stars[i].y as My;
        sx = da_stars.stars[i].x as My;
        zz = da_stars.stars[i].z;

        sx += 128;
        sy += 96;

        sx *= GFX_SCALE;
        sy *= GFX_SCALE;

        if (!da_stars.warp_stars)
            && (sx >= GFX_VIEW_TX)
            && (sx <= GFX_VIEW_BX)
            && (sy >= GFX_VIEW_TY)
            && (sy <= GFX_VIEW_BY)
        {
            draw_rectangle(sx as f32, sy as f32, STAR_SIZE, STAR_SIZE, WHITE);

            if zz < 0xC0 as f32 {
                draw_rectangle(sx as f32 + 1.0, sy as f32, STAR_SIZE, STAR_SIZE, WHITE);
            }

            if zz < 0x90 as f32 {
                draw_rectangle(sx as f32, sy as f32 + 1.0, STAR_SIZE, STAR_SIZE, WHITE);
                draw_rectangle(
                    sx as f32 + 1.0,
                    sy as f32 + 1.0,
                    STAR_SIZE,
                    STAR_SIZE,
                    WHITE,
                );
            }
        }

        /* Move the stars to their new locations...*/

        q = delta / da_stars.stars[i].z;

        da_stars.stars[i].z += delta;
        yy = da_stars.stars[i].y - (da_stars.stars[i].y * q);
        xx = da_stars.stars[i].x - (da_stars.stars[i].x * q);
        zz = da_stars.stars[i].z;

        yy = yy + (xx * alpha);
        xx = xx - (yy * alpha);

        /*
                tx = yy * beta;
                xx = xx + (tx * tx * 2);
        */
        yy = yy + beta;

        if da_stars.warp_stars {
            ey = yy as My;
            ex = xx as My;
            ex = (ex + 128) * GFX_SCALE;
            ey = (ey + 96) * GFX_SCALE;

            if (sx >= GFX_VIEW_TX)
                && (sx <= GFX_VIEW_BX)
                && (sy >= GFX_VIEW_TY)
                && (sy <= GFX_VIEW_BY)
                && (ex >= GFX_VIEW_TX)
                && (ex <= GFX_VIEW_BX)
                && (ey >= GFX_VIEW_TY)
                && (ey <= GFX_VIEW_BY)
            {
                draw_line(
                    sx as f32,
                    sy as f32,
                    (xx + 128.0) * GFX_SCALE as f32,
                    (yy + 96.0) * GFX_SCALE as f32,
                    THICKNESS,
                    WHITE,
                );
            }
        }

        da_stars.stars[i].y = yy;
        da_stars.stars[i].x = xx;

        if (zz >= 300.0) || (yy.abs() >= 110.0) {
            da_stars.stars[i].z = ((rand255() & 127) + 51) as f32;

            if (rand255() & 1) != 0 {
                da_stars.stars[i].x = (rand255() - 128) as f32;
                da_stars.stars[i].y = if (rand255() & 1) != 0 { -115.0 } else { 115.0 };
            } else {
                da_stars.stars[i].x = if (rand255() & 1) != 0 { -126.0 } else { 126.0 };
                da_stars.stars[i].y = (rand255() - 128) as f32;
            }
        }
    }

    da_stars.warp_stars = false;
}

fn side_starfield(da_stars: &mut Stars, params: &GameParams) {
    let mut delta;
    let mut alpha;
    let mut beta;
    let mut xx;
    let mut yy;
    let mut zz;
    let mut sx;
    let mut sy;
    let mut delt8;
    let nstars;

    nstars = if params.witchspace { 3 } else { 12 };

    delta = if da_stars.warp_stars {
        50
    } else {
        params.flight_speed
    };
    alpha = params.flight_roll;
    beta = params.flight_climb;

    if params.current_screen == SCR_LEFT_VIEW {
        delta = -delta;
        alpha = -alpha;
        beta = -beta;
    }

    for i in 0..nstars {
        sy = da_stars.stars[i].y;
        sx = da_stars.stars[i].x;
        zz = da_stars.stars[i].z;

        sx += 128.0;
        sy += 96.0;

        sx *= GFX_SCALE as f32;
        sy *= GFX_SCALE as f32;

        if (!da_stars.warp_stars)
            && (sx >= GFX_VIEW_TX as f32)
            && (sx <= GFX_VIEW_BX as f32)
            && (sy >= GFX_VIEW_TY as f32)
            && (sy <= GFX_VIEW_BY as f32)
        {
            draw_rectangle(sx as f32, sy as f32, STAR_SIZE, STAR_SIZE, WHITE);

            if zz < 0xC0 as f32 {
                draw_rectangle(sx as f32 + 1.0, sy as f32, STAR_SIZE, STAR_SIZE, WHITE);
            }

            if zz < 0x90 as f32 {
                draw_rectangle(sx as f32, sy as f32 + 1.0, STAR_SIZE, STAR_SIZE, WHITE);
                draw_rectangle(
                    sx as f32 + 1.0,
                    sy as f32 + 1.0,
                    STAR_SIZE,
                    STAR_SIZE,
                    WHITE,
                );
            }
        }

        yy = da_stars.stars[i].y;
        xx = da_stars.stars[i].x;
        zz = da_stars.stars[i].z;

        delt8 = delta as f32 / (zz / 32.0);
        xx = xx + delt8;

        xx += yy * (beta / 256) as f32;
        yy -= xx * (beta / 256) as f32;

        xx += ((yy / 256.0) * (alpha / 256) as f32) * (-xx);
        yy += ((yy / 256.0) * (alpha / 256) as f32) * (yy);

        yy += alpha as f32;

        da_stars.stars[i].y = yy;
        da_stars.stars[i].x = xx;

        if da_stars.warp_stars {
            draw_line(
                sx,
                sy,
                (xx + 128.0) * GFX_SCALE as f32,
                (yy + 96.0) * GFX_SCALE as f32,
                THICKNESS,
                WHITE,
            );
        }

        if (da_stars.stars[i].x).abs() >= 116.0 {
            da_stars.stars[i].y = (rand255() - 128) as f32;
            da_stars.stars[i].x = if params.current_screen == SCR_LEFT_VIEW {
                115.0
            } else {
                -115.0
            };
            da_stars.stars[i].z = (rand255() | 8) as f32;
        } else if (da_stars.stars[i].y).abs() >= 116.0 {
            da_stars.stars[i].x = (rand255() - 128) as f32;
            da_stars.stars[i].y = if alpha > 0 { -110.0 } else { 110.0 };
            da_stars.stars[i].z = (rand255() | 8) as f32;
        }
    }

    da_stars.warp_stars = false;
}

/*
 * When we change view, flip the stars over so they look like other stars.
 */

pub fn flip_stars(da_stars: &mut Stars, params: &GameParams) {
    let mut sx;
    let mut sy;

    let nstars = if params.witchspace { 3 } else { 12 };
    for i in 0..nstars {
        sy = da_stars.stars[i].y;
        sx = da_stars.stars[i].x;
        da_stars.stars[i].x = sy;
        da_stars.stars[i].y = sx;
    }
}

pub fn update_starfield(da_stars: &mut Stars, params: &GameParams) {
    if params.current_screen == SCR_FRONT_VIEW {
        front_starfield(da_stars, params);
    } else if params.current_screen == SCR_INTRO_ONE {
        front_starfield(da_stars, params);
    } else if params.current_screen == SCR_INTRO_TWO {
        front_starfield(da_stars, params);
    } else if params.current_screen == SCR_ESCAPE_POD {
        front_starfield(da_stars, params);
    } else if params.current_screen == SCR_REAR_VIEW {
        rear_starfield(da_stars, params);
    } else if params.current_screen == SCR_GAME_OVER {
        rear_starfield(da_stars, params);
    } else if params.current_screen == SCR_LEFT_VIEW {
        side_starfield(da_stars, params);
    } else if params.current_screen == SCR_RIGHT_VIEW {
        side_starfield(da_stars, params);
    }
}
pub fn rand255() -> My {
    rand::gen_range(0, 255)
}
pub fn randint() -> My {
    rand::gen_range(0, My::MAX)
}
