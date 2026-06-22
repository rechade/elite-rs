/*
 * Draws an object in the universe.
 * (Ship, Planet, Sun etc).
 */

use macroquad::{color::WHITE, shapes::draw_line};

use crate::{
    elite::{
        ShipData, ShipFaceNormal, ShipLine, SCR_ESCAPE_POD, SCR_FRONT_VIEW, SCR_GAME_OVER,
        SCR_INTRO_ONE, SCR_INTRO_TWO, SCR_LEFT_VIEW, SCR_REAR_VIEW, SCR_RIGHT_VIEW,
    },
    gfx::GFX_SCALE,
    shipdata::{NO_OF_SHIPS, SHIP_PLANET, SHIP_SUN},
    space::{Point, UnivObject},
    stars::{rand255, randint},
    vector::{vector_dot_product, Matrix, Vector, START_MATRIX, START_VECTOR},
    Config, GameParams, FLG_DEAD, FLG_EXPLOSION, FLG_FIRING, THICKNESS,
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
        // draw_planet(ship);
        return;
    }

    if (ship.da_type == SHIP_SUN) {
        // draw_sun(ship);
        return;
    }

    //   if ((fabs(ship.location.x) > ship.location.z) ||	/* Check for field of vision. */
    // (fabs(ship.location.y) > ship.location.z))
    //   {
    //       return;
    //   }

    if (config.wireframe) != 0 {
        draw_wireframe_ship(ship, &ship_list);
    } else {
        // draw_solid_ship(ship);
    }
}
fn draw_wireframe_ship(univ: &UnivObject, ship_list: &[ShipData; NO_OF_SHIPS + 1]) {
    let mut trans_mat: Matrix = START_MATRIX;
    let mut sx: i16;
    let mut sy: i16;
    let mut ex: i16;
    let mut ey: i16;
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
    let lasv: i16;

    ship = ship_list[univ.da_type as usize].clone();

    for i in 0..3 {
        trans_mat[i] = univ.rotmat[i];
    }

    camera_vec = univ.location;
    mult_vector(camera_vec, trans_mat);
    camera_vec = unit_vector(camera_vec);

    num_faces = ship.num_faces as usize;

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

    let mut point_list: [Point; 16] = [Point { x: 0, y: 0, z: 0 }; 16];
    for i in 0..ship.num_points as usize {
        vec.x = ship.points[i].x as f32;
        vec.y = ship.points[i].y as f32;
        vec.z = ship.points[i].z as f32;

        mult_vector(vec, trans_mat);

        rx = vec.x + univ.location.x;
        ry = vec.y + univ.location.y;
        rz = vec.z + univ.location.z;

        sx = ((rx * 256.0) / rz) as i16;
        sy = ((ry * 256.0) / rz) as i16;

        sy = -sy;

        sx += 128;
        sy += 96;

        sx *= GFX_SCALE;
        sy *= GFX_SCALE;

        point_list[i as usize].x = sx;
        point_list[i as usize].y = sy;
    }

    for i in 0..ship.num_lines {
        if (visible[ship.lines[i as usize].face1 as usize]
            | visible[ship.lines[i as usize].face2 as usize])
        {
            sx = point_list[ship.lines[i as usize].start_point as usize].x;
            sy = point_list[ship.lines[i as usize].start_point as usize].y;

            ex = point_list[ship.lines[i as usize].end_point as usize].x;
            ey = point_list[ship.lines[i as usize].end_point as usize].y;

            draw_line(sx as f32, sy as f32, ex as f32, ey as f32, THICKNESS, WHITE);
        }
    }

    if (univ.flags & FLG_FIRING) != 0 {
        lasv = ship_list[univ.da_type as usize].front_laser;
        draw_line(
            point_list[lasv as usize].x as f32,
            point_list[lasv as usize].y as f32,
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

fn unit_vector(camera_vec: Vector) -> Vector {
    todo!();
    START_VECTOR
}

fn mult_vector(camera_vec: Vector, trans_mat: [Vector; 3]) {
    todo!();
}
