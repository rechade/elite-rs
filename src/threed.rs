/*
 * Draws an object in the universe.
 * (Ship, Planet, Sun etc).
 */

use macroquad::{
    color::{BROWN, GREEN, ORANGE, RED, WHITE, YELLOW},
    shapes::{draw_circle_lines, draw_line, draw_rectangle},
};

use crate::{
    elite::{
        ShipData, ShipFaceNormal, SCR_ESCAPE_POD, SCR_FRONT_VIEW, SCR_GAME_OVER, SCR_INTRO_ONE,
        SCR_INTRO_TWO, SCR_LEFT_VIEW, SCR_REAR_VIEW, SCR_RIGHT_VIEW,
    },
    gfx::{
        GFX_SCALE, GFX_VIEW_BX, GFX_VIEW_BY, GFX_VIEW_TX, GFX_VIEW_TY, GFX_X_OFFSET, GFX_Y_OFFSET,
        STAR_SIZE,
    },
    shipdata::{NO_OF_SHIPS, SHIP_PLANET, SHIP_SUN},
    space::{Point, UnivObject},
    stars::{rand255, randint},
    vector::{
        mult_vector, unit_vector, vector_dot_product, Matrix, Vector, START_MATRIX, START_VECTOR,
    },
    Config, GameParams, My, FLG_DEAD, FLG_EXPLOSION, FLG_FIRING, SCANNER_Y_PROPORTION, THICKNESS,
};

pub fn draw_ship(
    ship: &mut UnivObject,
    params: &GameParams,
    config: &Config,
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    point_list: &mut [Point; 60],
) {
    if (params.current_screen != SCR_FRONT_VIEW)
        && (params.current_screen != SCR_REAR_VIEW)
        && (params.current_screen != SCR_LEFT_VIEW)
        && (params.current_screen != SCR_RIGHT_VIEW)
        && (params.current_screen != SCR_INTRO_ONE)
        && (params.current_screen != SCR_INTRO_TWO)
        && (params.current_screen != SCR_GAME_OVER)
        && (params.current_screen != SCR_ESCAPE_POD)
    {
        return;
    }

    if (ship.flags & FLG_DEAD) != 0 && !(ship.flags & FLG_EXPLOSION) == 0 {
        ship.flags |= FLG_EXPLOSION;
        ship.exp_seed = randint();
        ship.exp_delta = 18;
    }

    if (ship.flags & FLG_EXPLOSION) != 0 {
        // crst
        // draw_explosion(ship);
        return;
    }

    if ship.location.z <= 0.0 {
        /* Only display ships in front of us. */
        return;
    }

    if ship.da_type == SHIP_PLANET {
        draw_planet(ship, params);
        return;
    }

    if ship.da_type == SHIP_SUN {
        draw_sun(ship);
        return;
    }

    if ((ship.location.x).abs() > ship.location.z * 4.0) ||	/* Check for field of vision. */
        ((ship.location.y).abs() > ship.location.z * 2.0)
    {
        return;
    }

    if (config.wireframe) != 0 {
        draw_wireframe_ship(ship, ship_list, params, point_list);
    } else {
        // crst
        // draw_solid_ship(ship);
    }
}
fn draw_wireframe_ship(
    univ: &UnivObject,
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    params: &GameParams,
    point_list: &mut [Point; 60],
) {
    let mut trans_mat: Matrix = START_MATRIX;
    let mut sx: f32;
    let mut sy: f32;
    let mut ex: f32;
    let mut ey: f32;
    let mut rx: f32;
    let mut ry: f32;
    let mut rz: f32;
    let mut visible: [bool; 32] = [false; 32];
    let mut vec: Vector = START_VECTOR;
    let mut cos_angle: f32;
    let mut tmp: f32;
    // let mut ship_norm: Vec<ShipFaceNormal>;
    let ship = ship_list[univ.da_type as usize].clone();

    for i in 0..3 {
        trans_mat[i] = univ.rotmat[i];
    }

    let mut camera_vec = univ.location;
    mult_vector(&mut camera_vec, &trans_mat);
    camera_vec = unit_vector(&camera_vec);

    let num_faces = ship.num_faces;

    // ship_norm = ship.normals.clone();
    for i in 0..num_faces {
        vec.x = ship.normals[i].x;
        vec.y = ship.normals[i].y;
        vec.z = ship.normals[i].z;

        if (vec.x == 0.0) && (vec.y == 0.0) && (vec.z == 0.0) {
            visible[i] = true;
        } else {
            vec = unit_vector(&vec);
            cos_angle = vector_dot_product(&vec, &camera_vec);
            visible[i] = cos_angle < -0.15;
        }
    }

    tmp = trans_mat[0].y;
    trans_mat[0].y = trans_mat[1].x;
    trans_mat[1].x = tmp;

    tmp = trans_mat[0].z;
    trans_mat[0].z = trans_mat[2].x;
    trans_mat[2].x = tmp;

    tmp = trans_mat[1].z;
    trans_mat[1].z = trans_mat[2].y;
    trans_mat[2].y = tmp;

    // let mut point_list: [Point; 60] = [Point {
    //     x: 0.0,
    //     y: 0.0,
    //     z: 0.0,
    // }; 60];
    for i in 0..ship.num_points {
        vec.x = ship.points[i].x;
        vec.y = ship.points[i].y;
        vec.z = ship.points[i].z;

        mult_vector(&mut vec, &trans_mat);

        rx = vec.x + univ.location.x;
        ry = vec.y + univ.location.y;
        rz = vec.z + univ.location.z;

        sx = ((rx * 256.0) / rz);
        sy = ((ry * 256.0) / rz);

        sy = -sy;

        // sx += 128.0; // params.screen_width * 0.5;
        // sy += 96.0; // params.screen_height * 0.5 * (1.0 - SCANNER_Y_PROPORTION);

        // sx *= params.screen_scale;
        // sy *= params.screen_scale;
        point_list[i].x = sx;
        point_list[i].y = sy;
    }

    for i in 0..ship.num_lines {
        if visible[ship.lines[i].face1] | visible[ship.lines[i].face2] {
            sx = point_list[ship.lines[i].start_point].x;
            sy = point_list[ship.lines[i].start_point].y;

            ex = point_list[ship.lines[i].end_point].x;
            ey = point_list[ship.lines[i].end_point].y;

            sx += params.mid_screen_x;
            sy += params.mid_screen_y * (1.0 - SCANNER_Y_PROPORTION);
            ex += params.mid_screen_x;
            ey += params.mid_screen_y * (1.0 - SCANNER_Y_PROPORTION);
            if sx > 0.0
                && sx < params.screen_width
                && ex > 0.0
                && ex < params.screen_width
                && sy > 0.0
                && sy < params.screen_height * (1.0 - SCANNER_Y_PROPORTION)
                && ey > 0.0
                && ey < params.screen_height * (1.0 - SCANNER_Y_PROPORTION)
            {
                draw_line(sx, sy, ex, ey, THICKNESS, WHITE);
            }
        }
    }

    if (univ.flags & FLG_FIRING) != 0 {
        let lasv = ship_list[univ.da_type as usize].front_laser;
        draw_line(
            point_list[lasv].x,
            point_list[lasv].y,
            {
                if univ.location.x > 0.0 {
                    0.0
                } else {
                    511.0
                }
            },
            (rand255() * 2) as f32,
            THICKNESS,
            WHITE,
        );
    }
}
/*
 * Draw a planet.
 * We can currently do three different types of planet...
 * - Wireframe.
 * - Fractal landscape.
 * - SNES Elite style.
 */

fn draw_planet(planet: &UnivObject, params: &GameParams) {
    let mut trans_mat: Matrix = START_MATRIX;
    let mut normal_vec: Vector = START_VECTOR;
    let visible;
    for i in 0..3 {
        trans_mat[i] = planet.rotmat[i];
    }
    let mut camera_vec = planet.location;
    let cos_angle;
    camera_vec = unit_vector(&camera_vec);

    normal_vec.x = -planet.location.x;
    normal_vec.y = -planet.location.y;
    normal_vec.z = -planet.location.z;

    if (normal_vec.x == 0.0) && (normal_vec.y == 0.0) && (normal_vec.z == 0.0) {
        visible = true;
    } else {
        normal_vec = unit_vector(&normal_vec);
        cos_angle = vector_dot_product(&normal_vec, &camera_vec);
        visible = cos_angle < -0.15;
    }
    if visible {
        let mut x = (planet.location.x * 512.0) / planet.location.z;
        let mut y = (planet.location.y * 512.0) / planet.location.z;

        y = -y;

        x += params.screen_width * 0.5;
        y += params.row_y_pos * 0.5;

        /* Planets are BIG! */
        let mut radius = (6291456 / planet.distance) as f32;
        if ((x + radius) < 0.0)
            || ((x - radius) > params.screen_width)
            || ((y + radius) < 0.0)
            || ((y - radius) > params.row_y_pos)
        {
            return;
        }

        // crst
        // match (planet_render_style) {
        match 1 {
            // 0 => draw_wireframe_planet(x, y, radius, planet.rotmat),
            1 => draw_circle_lines(x, y, radius, THICKNESS, GREEN),
            // 2 => render_planet(x, y, radius , planet.rotmat),
            // 3 => render_planet(x, y, radius , planet.rotmat),
            _ => draw_circle_lines(x, y, radius, THICKNESS, GREEN),
        }
    }
}
fn render_sun_line(xo: f32, yo: f32, x: f32, y: f32, radius: f32) {
    let sy = yo + y;
    let mut colour;
    let mut distance;
    let mut mix;

    if !(GFX_VIEW_TY + GFX_Y_OFFSET..=GFX_VIEW_BY + GFX_Y_OFFSET).contains(&sy) {
        return;
    }

    let mut sx = xo - x;
    let mut ex = xo + x;

    sx -= ((radius as My * (2 + (randint() & 7))) >> 8) as f32;
    ex += ((radius as My * (2 + (randint() & 7))) >> 8) as f32;

    if (sx > GFX_VIEW_BX + GFX_X_OFFSET) || (ex < GFX_VIEW_TX + GFX_X_OFFSET) {
        return;
    }

    if sx < GFX_VIEW_TX + GFX_X_OFFSET {
        sx = GFX_VIEW_TX + GFX_X_OFFSET;
    }

    if ex > GFX_VIEW_BX + GFX_X_OFFSET {
        ex = GFX_VIEW_BX + GFX_X_OFFSET;
    }
    let mut inner = (radius as My * (200 + (randint() & 7))) >> 8;
    inner *= inner;

    let mut inner2 = (radius as My * (220 + (randint() & 7))) >> 8;
    inner2 *= inner2;

    let mut outer = (radius as My * (239 + (randint() & 7))) >> 8;
    outer *= outer;

    let dy = y * y;
    let mut dx = sx - xo;

    while sx <= ex {
        mix = (sx as My ^ y as My) & 1;
        distance = dx * dx + dy;

        if distance < inner as f32 {
            colour = WHITE;
        } else if distance < inner2 as f32 {
            colour = YELLOW;
        } else if distance < outer as f32 {
            colour = ORANGE;
        } else {
            colour = if mix != 0 { RED } else { BROWN };
        }

        draw_rectangle(sx, sy, STAR_SIZE, STAR_SIZE, colour);
        sx += 1.0;
        dx += 1.0;
    }
}

fn render_sun(xo: f32, yo: f32, radius: f32) {
    let xxo = xo + GFX_X_OFFSET;
    let yyo = yo + GFX_Y_OFFSET;

    let mut s = -radius;
    let mut x = radius;
    let mut y = 0.0;

    // crst
    // s -= x + x;
    while y <= x {
        render_sun_line(xxo, yyo, x, y, radius);
        render_sun_line(xxo, yyo, x, -y, radius);
        render_sun_line(xxo, yyo, y, x, radius);
        render_sun_line(xxo, yyo, y, -x, radius);

        s += y + y + 1.0;
        y += 1.0;
        if s >= 0.0 {
            s -= x + x + 2.0;
            x -= 1.0;
        }
    }
}

fn draw_sun(planet: &UnivObject) {
    let mut x = (planet.location.x * 256.0) / planet.location.z;
    let mut y = (planet.location.y * 256.0) / planet.location.z;

    y = -y;

    x += 128.0;
    y += 96.0;

    x *= GFX_SCALE;
    y *= GFX_SCALE;

    let mut radius = (6291456 / planet.distance) as f32;

    radius = radius * GFX_SCALE;

    if ((x + radius) < 0.0)
        || ((x - radius) > 511.0)
        || ((y + radius) < 0.0)
        || ((y - radius) > 383.0)
    {
        return;
    }
    render_sun(x, y, radius);
}
