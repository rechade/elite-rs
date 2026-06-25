/*
 * Draws an object in the universe.
 * (Ship, Planet, Sun etc).
 */

use macroquad::{
    color::{BROWN, GREEN, ORANGE, RED, WHITE, YELLOW},
    shapes::{draw_circle, draw_circle_lines, draw_line, draw_rectangle},
};

use crate::{
    elite::{
        ShipData, ShipFaceNormal, ShipLine, SCR_ESCAPE_POD, SCR_FRONT_VIEW, SCR_GAME_OVER,
        SCR_INTRO_ONE, SCR_INTRO_TWO, SCR_LEFT_VIEW, SCR_REAR_VIEW, SCR_RIGHT_VIEW,
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
    Config, GameParams, My, FLG_DEAD, FLG_EXPLOSION, FLG_FIRING, THICKNESS,
};

pub fn draw_ship(
    ship: &mut UnivObject,
    params: &GameParams,
    config: &Config,
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
) {
    if ((params.current_screen != SCR_FRONT_VIEW)
        && (params.current_screen != SCR_REAR_VIEW)
        && (params.current_screen != SCR_LEFT_VIEW)
        && (params.current_screen != SCR_RIGHT_VIEW)
        && (params.current_screen != SCR_INTRO_ONE)
        && (params.current_screen != SCR_INTRO_TWO)
        && (params.current_screen != SCR_GAME_OVER)
        && (params.current_screen != SCR_ESCAPE_POD))
    {
        return;
    }

    if ((ship.flags & FLG_DEAD) != 0 && !(ship.flags & FLG_EXPLOSION) == 0) {
        ship.flags |= FLG_EXPLOSION;
        ship.exp_seed = randint();
        ship.exp_delta = 18;
    }

    if (ship.flags & FLG_EXPLOSION) != 0 {
        // draw_explosion(ship);
        return;
    }

    if (ship.location.z <= 0.0) {
        /* Only display ships in front of us. */
        return;
    }

    if (ship.da_type == SHIP_PLANET) {
        draw_planet(ship);
        return;
    }

    if (ship.da_type == SHIP_SUN) {
        draw_sun(ship);
        return;
    }

    if (((ship.location.x).abs() > ship.location.z) ||	/* Check for field of vision. */
        ((ship.location.y).abs() > ship.location.z))
    {
        return;
    }

    if (config.wireframe) != 0 {
        draw_wireframe_ship(ship, &ship_list);
    } else {
        // draw_solid_ship(ship);
    }
}
fn draw_wireframe_ship(univ: &UnivObject, ship_list: &[ShipData; NO_OF_SHIPS + 1]) {
    let mut trans_mat: Matrix = START_MATRIX;
    let mut sx: My;
    let mut sy: My;
    let mut ex: My;
    let mut ey: My;
    let mut rx: f32;
    let mut ry: f32;
    let mut rz: f32;
    let mut visible: [bool; 32] = [false; 32];
    let mut vec: Vector = START_VECTOR;
    let mut camera_vec: Vector = START_VECTOR;
    let mut cos_angle: f32;
    let mut tmp: f32;
    let mut ship_norm: Vec<ShipFaceNormal>;
    let mut num_faces: usize;
    let mut ship: ShipData;

    ship = ship_list[univ.da_type].clone();

    // dbg!(univ.da_type);
    for i in 0..3 {
        trans_mat[i] = univ.rotmat[i];
    }

    camera_vec = univ.location;
    mult_vector(&mut camera_vec, &trans_mat);
    camera_vec = unit_vector(camera_vec);

    num_faces = ship.num_faces;

    for i in 0..num_faces {
        ship_norm = ship.normals.clone();

        vec.x = ship_norm[i].x as f32;
        vec.y = ship_norm[i].y as f32;
        vec.z = ship_norm[i].z as f32;

        if ((vec.x == 0.0) && (vec.y == 0.0) && (vec.z == 0.0)) {
            visible[i] = true;
        } else {
            vec = unit_vector(vec);
            cos_angle = vector_dot_product(&vec, &camera_vec);
            visible[i] = (cos_angle < -0.2);
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

    let mut point_list: [Point; 60] = [Point { x: 0, y: 0, z: 0 }; 60];
    for i in 0..ship.num_points {
        vec.x = ship.points[i].x as f32;
        vec.y = ship.points[i].y as f32;
        vec.z = ship.points[i].z as f32;

        mult_vector(&mut vec, &trans_mat);

        rx = vec.x + univ.location.x;
        ry = vec.y + univ.location.y;
        rz = vec.z + univ.location.z;

        sx = ((rx * 256.0) / rz) as My;
        sy = ((ry * 256.0) / rz) as My;

        sy = -sy;

        sx += 128;
        sy += 96;

        sx *= GFX_SCALE;
        sy *= GFX_SCALE;

        point_list[i].x = sx;
        point_list[i].y = sy;
    }

    for i in 0..ship.num_lines {
        if (visible[ship.lines[i].face1] | visible[ship.lines[i].face2]) {
            sx = point_list[ship.lines[i].start_point].x;
            sy = point_list[ship.lines[i].start_point].y;

            ex = point_list[ship.lines[i].end_point].x;
            ey = point_list[ship.lines[i].end_point].y;

            draw_line(sx as f32, sy as f32, ex as f32, ey as f32, THICKNESS, WHITE);
        }
    }

    if (univ.flags & FLG_FIRING) != 0 {
        let lasv = ship_list[univ.da_type].front_laser;
        draw_line(
            point_list[lasv].x as f32,
            point_list[lasv].y as f32,
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

fn draw_planet(planet: &UnivObject) {
    let mut x = (planet.location.x * 256.0) / planet.location.z;
    let mut y = (planet.location.y * 256.0) / planet.location.z;

    y = -y;

    x += 128.0;
    y += 96.0;

    x *= GFX_SCALE as f32;
    y *= GFX_SCALE as f32;

    let mut radius = 6291456 / planet.distance;
    //	radius = 6291456 / ship_vec.z;   /* Planets are BIG! */
    radius *= GFX_SCALE;

    if (((x + radius as f32) < 0.0)
        || ((x - radius as f32) > 511.0)
        || ((y + radius as f32) < 0.0)
        || ((y - radius as f32) > 383.0))
    {
        return;
    }

    // match (planet_render_style) {
    match (1) {
        // 0 => draw_wireframe_planet(x, y, radius, planet.rotmat),
        1 => draw_circle_lines(x, y, radius as f32, THICKNESS, WHITE),
        // 2 => render_planet(x, y, radius as f32, planet.rotmat),
        // 3 => render_planet(x, y, radius as f32, planet.rotmat),
        _ => draw_circle_lines(x, y, radius as f32, THICKNESS, WHITE),
    }
}
fn render_sun_line(xo: My, yo: My, x: My, y: My, radius: My) {
    let sy = yo + y;
    // int sx,ex;
    let mut colour;
    // int dx,dy;
    let mut distance;
    // int inner,outer;
    // int inner2;
    let mut mix;

    // const GFX_Y_OFFSET: _ = $0;
    // const GFX_Y_OFFSET: My = 0;
    if ((sy < GFX_VIEW_TY + GFX_Y_OFFSET) || (sy > GFX_VIEW_BY + GFX_Y_OFFSET)) {
        return;
    }

    let mut sx = xo - x;
    let mut ex = xo + x;

    sx -= (radius * (2 + (randint() & 7))) >> 8;
    ex += (radius * (2 + (randint() & 7))) >> 8;

    if ((sx > GFX_VIEW_BX + GFX_X_OFFSET) || (ex < GFX_VIEW_TX + GFX_X_OFFSET)) {
        return;
    }

    if (sx < GFX_VIEW_TX + GFX_X_OFFSET) {
        sx = GFX_VIEW_TX + GFX_X_OFFSET;
    }

    if (ex > GFX_VIEW_BX + GFX_X_OFFSET) {
        ex = GFX_VIEW_BX + GFX_X_OFFSET;
    }
    let mut inner = (radius * (200 + (randint() & 7))) >> 8;
    inner *= inner;

    let mut inner2 = (radius * (220 + (randint() & 7))) >> 8;
    inner2 *= inner2;

    let mut outer = (radius * (239 + (randint() & 7))) >> 8;
    outer *= outer;

    let dy = y * y;
    let mut dx = sx - xo;

    while sx <= ex {
        mix = (sx ^ y) & 1;
        distance = dx * dx + dy;

        if (distance < inner) {
            colour = WHITE;
        } else if (distance < inner2) {
            colour = YELLOW;
        } else if (distance < outer) {
            colour = ORANGE;
        } else {
            colour = if mix != 0 { RED } else { BROWN };
        }

        draw_rectangle(sx as f32, sy as f32, STAR_SIZE, STAR_SIZE, colour);
        sx += 1;
        dx += 1;
    }
}

fn render_sun(xo: My, yo: My, radius: My) {
    let xxo = xo + GFX_X_OFFSET;
    let yyo = yo + GFX_Y_OFFSET;

    let mut s = -radius;
    let mut x = radius;
    let mut y = 0;

    // s -= x + x;
    while (y <= x) {
        render_sun_line(xxo, yyo, x, y, radius);
        render_sun_line(xxo, yyo, x, -y, radius);
        render_sun_line(xxo, yyo, y, x, radius);
        render_sun_line(xxo, yyo, y, -x, radius);

        s += y + y + 1;
        y += 1;
        if (s >= 0) {
            s -= x + x + 2;
            x -= 1;
        }
    }
}

fn draw_sun(planet: &UnivObject) {
    let mut x = (planet.location.x * 256.0) / planet.location.z;
    let mut y = (planet.location.y * 256.0) / planet.location.z;

    y = -y;

    x += 128.0;
    y += 96.0;

    x *= GFX_SCALE as f32;
    y *= GFX_SCALE as f32;

    let mut radius = 6291456 / planet.distance;

    radius *= GFX_SCALE;

    if (((x + radius as f32) < 0.0)
        || ((x - radius as f32) > 511.0)
        || ((y + radius as f32) < 0.0)
        || ((y - radius as f32) > 383.0))
    {
        return;
    }
    render_sun(x as My, y as My, radius);
}
