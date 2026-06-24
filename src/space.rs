use macroquad::{
    color::{GOLD, GREEN, MAGENTA, PINK, RED, WHITE, YELLOW},
    shapes::{draw_circle, draw_line, draw_rectangle, draw_triangle},
};

use crate::{
    Config, FLG_CLOAKED, FLG_DEAD, FLG_FIRING, FLG_HOSTILE, FLG_REMOVE, GameParams, My,
    SCR_BREAK_PATTERN, THICKNESS,
    elite::{
        Commander, MAX_UNIV_OBJECTS, SCR_ESCAPE_POD, SCR_GAME_OVER, SCR_INTRO_ONE, SCR_INTRO_TWO,
        SCR_LEFT_VIEW, SCR_REAR_VIEW, SCR_RIGHT_VIEW, ShipData,
    },
    gfx::STAR_SIZE,
    info_message,
    pilot::disengage_auto_pilot,
    planet::GalaxySeed,
    shipdata::{
        NO_OF_SHIPS, SHIP_ALLOY, SHIP_ASTEROID, SHIP_BOULDER, SHIP_CARGO, SHIP_CONSTRICTOR,
        SHIP_CORIOLIS, SHIP_COUGAR, SHIP_DODEC, SHIP_ESCAPE_CAPSULE, SHIP_PLANET, SHIP_ROCK,
        SHIP_SUN, SHIP_VIPER,
    },
    sound::{SND_EXPLODE, SND_LAUNCH},
    stars::{Stars, create_new_stars},
    swat::{
        MISSILE_UNARMED, add_new_ship, add_new_station, clear_universe, remove_ship, reset_weapons,
        snd_play_sample,
    },
    threed::draw_ship,
    trade::carrying_contraband,
    vector::{Matrix, START_MATRIX, Vector, tidy_matrix, unit_vector},
};

#[derive(Clone, Copy)]
pub struct Point {
    pub x: My,
    pub y: My,
    pub z: My,
}
#[derive(Clone, Copy, Debug)]
pub struct UnivObject {
    pub location: Vector,
    pub rotmat: Matrix,
    pub da_type: My,
    pub rotx: My,
    pub rotz: My,
    pub flags: My,
    pub energy: My,
    pub velocity: My,
    pub acceleration: My,
    pub missiles: My,
    pub target: My,
    pub bravery: My,
    pub exp_delta: My,
    pub exp_seed: My,
    pub distance: My,
}

impl UnivObject {
    pub fn new(
        location: Vector,
        rotmat: Matrix,
        da_type: My,
        rotx: My,
        rotz: My,
        flags: My,
        energy: My,
        velocity: My,
        acceleration: My,
        missiles: My,
        target: My,
        bravery: My,
        exp_delta: My,
        exp_seed: My,
        distance: My,
    ) -> Self {
        Self {
            location,
            rotmat,
            da_type,
            rotx,
            rotz,
            flags,
            energy,
            velocity,
            acceleration,
            missiles,
            target,
            bravery,
            exp_delta,
            exp_seed,
            distance,
        }
    }
}

pub struct Space {
    flight_climb: My,
    flight_roll: My,
    flight_speed: My,
    destination_planet: GalaxySeed,
    hyper_ready: bool,
    hyper_countdown: My,
    hyper_name: [char; 16],
    hyper_distance: My,
    hyper_galactic: bool,
}
pub fn dock_player(params: &mut GameParams) {
    disengage_auto_pilot(params);
    params.docked = true;
    params.flight_speed = 0;
    params.flight_roll = 0;
    params.flight_climb = 0;
    params.front_shield = 255;
    params.aft_shield = 255;
    params.energy = 255;
    params.myship.altitude = 255;
    params.myship.cabtemp = 30;
    reset_weapons(params);
}
pub fn jump_warp(universe: &mut [UnivObject], params: &mut GameParams) {
    let mut da_type;
    let mut jump;

    for i in 0..MAX_UNIV_OBJECTS {
        da_type = universe[i].da_type;

        if ((da_type > 0)
            && (da_type != SHIP_ASTEROID)
            && (da_type != SHIP_CARGO)
            && (da_type != SHIP_ALLOY)
            && (da_type != SHIP_ROCK)
            && (da_type != SHIP_BOULDER)
            && (da_type != SHIP_ESCAPE_CAPSULE))
        {
            info_message("Mass Locked".to_string(), params);
            return;
        }
    }

    if ((universe[0].distance < 75001) || (universe[1].distance < 75001)) {
        info_message("Mass Locked".to_string(), params);
        return;
    }

    if (universe[0].distance < universe[1].distance) {
        jump = universe[0].distance - 75000;
    } else {
        jump = universe[1].distance - 75000;
    }
    if (jump > 1024) {
        jump = 1024;
    }

    for i in 0..MAX_UNIV_OBJECTS {
        if (universe[i].da_type != 0) {
            universe[i].location.z -= jump as f32;
        }
    }

    params.warp_stars = true;
    params.mcount &= 63;
    params.in_battle = false;
}
pub fn launch_player(
    params: &mut GameParams,
    cmdr: &mut Commander,
    da_stars: &mut Stars,
    univ: &mut [UnivObject],
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
) {
    let mut rotmat: Matrix = START_MATRIX;

    params.docked = false;
    params.flight_speed = 12;
    params.flight_roll = -15;
    params.flight_climb = 0;
    cmdr.legal_status |= carrying_contraband(cmdr);
    create_new_stars(da_stars, params);
    clear_universe(univ, ship_count, &mut params.in_battle);
    // generate_landscape(docked_planet.a * 251 + docked_planet.b);
    add_new_ship(
        SHIP_PLANET,
        0.0,
        0.0,
        65536.0,
        &rotmat,
        0,
        0,
        univ,
        ship_list,
        ship_count,
    );

    rotmat[2].x = -rotmat[2].x;
    rotmat[2].y = -rotmat[2].y;
    rotmat[2].z = -rotmat[2].z;
    add_new_station(
        0.0,
        0.0,
        -256.0,
        &rotmat,
        univ,
        ship_list,
        ship_count,
        &params.current_planet_data,
    );

    params.current_screen = SCR_BREAK_PATTERN;
    snd_play_sample(SND_LAUNCH);
}
fn switch_to_view(flip: &mut UnivObject, params: &GameParams) {
    let mut tmp: f32;

    if ((params.current_screen == SCR_REAR_VIEW) || (params.current_screen == SCR_GAME_OVER)) {
        flip.location.x = -flip.location.x;
        flip.location.z = -flip.location.z;

        flip.rotmat[0].x = -flip.rotmat[0].x;
        flip.rotmat[0].z = -flip.rotmat[0].z;

        flip.rotmat[1].x = -flip.rotmat[1].x;
        flip.rotmat[1].z = -flip.rotmat[1].z;

        flip.rotmat[2].x = -flip.rotmat[2].x;
        flip.rotmat[2].z = -flip.rotmat[2].z;
        return;
    }

    if (params.current_screen == SCR_LEFT_VIEW) {
        tmp = flip.location.x;
        flip.location.x = flip.location.z;
        flip.location.z = -tmp;

        if (flip.da_type < 0) {
            return;
        }

        tmp = flip.rotmat[0].x;
        flip.rotmat[0].x = flip.rotmat[0].z;
        flip.rotmat[0].z = -tmp;

        tmp = flip.rotmat[1].x;
        flip.rotmat[1].x = flip.rotmat[1].z;
        flip.rotmat[1].z = -tmp;

        tmp = flip.rotmat[2].x;
        flip.rotmat[2].x = flip.rotmat[2].z;
        flip.rotmat[2].z = -tmp;
        return;
    }

    if (params.current_screen == SCR_RIGHT_VIEW) {
        tmp = flip.location.x;
        flip.location.x = -flip.location.z;
        flip.location.z = tmp;

        if (flip.da_type < 0) {
            return;
        }

        tmp = flip.rotmat[0].x;
        flip.rotmat[0].x = -flip.rotmat[0].z;
        flip.rotmat[0].z = tmp;

        tmp = flip.rotmat[1].x;
        flip.rotmat[1].x = -flip.rotmat[1].z;
        flip.rotmat[1].z = tmp;

        tmp = flip.rotmat[2].x;
        flip.rotmat[2].x = -flip.rotmat[2].z;
        flip.rotmat[2].z = tmp;
    }
}
/*
 * Update all the objects in the universe and render them.
 */

pub fn update_universe(
    universe: &mut [UnivObject],
    cmdr: &mut Commander,
    ship_list: &mut [ShipData; NO_OF_SHIPS + 1],
    params: &mut GameParams,
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    config: &Config,
) {
    let mut da_type;
    let mut bounty;
    let mut da_string: String;
    // char str[80];
    let mut flip: UnivObject;

    for i in 0..MAX_UNIV_OBJECTS {
        da_type = universe[i].da_type;

        if da_type != 0 {
            if (universe[i].flags & FLG_REMOVE) != 0 {
                if da_type == SHIP_VIPER {
                    cmdr.legal_status |= 64;
                }

                bounty = ship_list[da_type as usize].bounty;

                if ((bounty != 0) && (!params.witchspace)) {
                    cmdr.credits += bounty;
                    // sprintf (str, "%d.%d CR", cmdr.credits / 10, cmdr.credits % 10);
                    // info_message (str);
                }

                remove_ship(i, universe, ship_count, ship_list);
                continue;
            }

            let var_name = FLG_DEAD;
            if ((params.detonate_bomb != 0)
                && ((universe[i].flags & var_name) == 0)
                && (da_type != SHIP_PLANET)
                && (da_type != SHIP_SUN)
                && (da_type != SHIP_CONSTRICTOR)
                && (da_type != SHIP_COUGAR)
                && (da_type != SHIP_CORIOLIS)
                && (da_type != SHIP_DODEC))
            {
                snd_play_sample(SND_EXPLODE);
                universe[i].flags |= FLG_DEAD;
            }

            if ((params.current_screen != SCR_INTRO_ONE)
                && (params.current_screen != SCR_INTRO_TWO)
                && (params.current_screen != SCR_GAME_OVER)
                && (params.current_screen != SCR_ESCAPE_POD))
            {
                // tactics (i);
            }

            move_univ_object(&mut universe[i], &params, &ship_list);

            flip = universe[i];
            switch_to_view(&mut flip, &params);

            if (da_type == SHIP_PLANET) {
                if ((ship_count[SHIP_CORIOLIS as usize] == 0)
                    && (ship_count[SHIP_DODEC as usize] == 0)
                    && (universe[i].distance < 65792))
                // was 49152
                {
                    // make_station_appear();
                }

                draw_ship(&mut flip, params, config, ship_list);
                continue;
            }

            if (da_type == SHIP_SUN) {
                draw_ship(&mut flip, params, config, ship_list);
                continue;
            }

            if (universe[i].distance < 170) {
                if ((da_type == SHIP_CORIOLIS) || (da_type == SHIP_DODEC)) {
                    // check_docking (i);
                } else {
                    // scoop_item(i);
                }

                continue;
            }

            if (universe[i].distance > 57344) {
                remove_ship(i, universe, ship_count, ship_list);
                continue;
            }

            draw_ship(&mut flip, params, config, ship_list);

            universe[i].flags = flip.flags;
            universe[i].exp_seed = flip.exp_seed;
            universe[i].exp_delta = flip.exp_delta;

            universe[i].flags &= !FLG_FIRING;

            if (universe[i].flags & FLG_DEAD) != 0 {
                continue;
            }

            // check_target(i, &flip);
        }
    }

    params.detonate_bomb = 0;
}
/*
 * Update an objects location in the universe.
 */

fn move_univ_object(
    obj: &mut UnivObject,
    params: &GameParams,
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
) {
    let mut x: f32;
    let mut y: f32;
    let mut z: f32;
    let mut k2: f32;
    let mut alpha: f32;
    let mut beta: f32;
    let mut rotx: My;
    let mut rotz: My;
    let mut speed: f32;

    alpha = params.flight_roll as f32 / 256.0;
    beta = params.flight_climb as f32 / 256.0;

    x = obj.location.x;
    y = obj.location.y;
    z = obj.location.z;

    if (!(obj.flags & FLG_DEAD) != 0) {
        if (obj.velocity != 0) {
            speed = obj.velocity as f32;
            speed *= 1.5;
            x += obj.rotmat[2].x * speed;
            y += obj.rotmat[2].y * speed;
            z += obj.rotmat[2].z * speed;
        }

        if (obj.acceleration != 0) {
            obj.velocity += obj.acceleration;
            obj.acceleration = 0;
            if (obj.velocity > ship_list[obj.da_type as usize].velocity) {
                obj.velocity = ship_list[obj.da_type as usize].velocity;
            }

            if (obj.velocity <= 0) {
                obj.velocity = 1;
            }
        }
    }

    k2 = y - alpha * x;
    z = z + beta * k2;
    y = k2 - z * beta;
    x = x + alpha * y;

    z = z - params.flight_speed as f32;

    obj.location.x = x;
    obj.location.y = y;
    obj.location.z = z;

    obj.distance = (x * x + y * y + z * z).sqrt() as My;

    if (obj.da_type == SHIP_PLANET) {
        beta = 0.0;
    }

    rotate_vec(&mut obj.rotmat[2], alpha, beta);
    rotate_vec(&mut obj.rotmat[1], alpha, beta);
    rotate_vec(&mut obj.rotmat[0], alpha, beta);

    if (obj.flags & FLG_DEAD) != 0 {
        return;
    }

    rotx = obj.rotx;
    rotz = obj.rotz;

    /* If necessary rotate the object around the X axis... */

    if (rotx != 0) {
        (obj.rotmat[2].x, obj.rotmat[1].x) = rotate_x_first(obj.rotmat[2].x, obj.rotmat[1].x, rotx);
        (obj.rotmat[2].y, obj.rotmat[1].y) = rotate_x_first(obj.rotmat[2].y, obj.rotmat[1].y, rotx);
        (obj.rotmat[2].z, obj.rotmat[1].z) = rotate_x_first(obj.rotmat[2].z, obj.rotmat[1].z, rotx);

        if ((rotx != 127) && (rotx != -127)) {
            obj.rotx -= if rotx < 0 { -1 } else { 1 };
        }
    }

    /* If necessary rotate the object around the Z axis... */

    if (rotz != 0) {
        (obj.rotmat[0].x, obj.rotmat[1].x) = rotate_x_first(obj.rotmat[0].x, obj.rotmat[1].x, rotz);
        (obj.rotmat[0].y, obj.rotmat[1].y) = rotate_x_first(obj.rotmat[0].y, obj.rotmat[1].y, rotz);
        (obj.rotmat[0].z, obj.rotmat[1].z) = rotate_x_first(obj.rotmat[0].z, obj.rotmat[1].z, rotz);

        if ((rotz != 127) && (rotz != -127)) {
            obj.rotz -= if (rotz < 0) { -1 } else { 1 };
        }
    }

    /* Orthonormalize the rotation matrix... */

    tidy_matrix(&mut obj.rotmat);
}
fn rotate_x_first(a: f32, b: f32, direction: My) -> (f32, f32) {
    let mut fx: f32;
    let mut ux: f32;
    let mut aa;
    let mut bb;

    fx = a;
    ux = b;

    if (direction < 0) {
        aa = fx - (fx / 512.0) + (ux / 19.0);
        bb = ux - (ux / 512.0) - (fx / 19.0);
    } else {
        aa = fx - (fx / 512.0) - (ux / 19.0);
        bb = ux - (ux / 512.0) + (fx / 19.0);
    }
    (aa, bb)
}

fn rotate_vec(vec: &mut Vector, alpha: f32, beta: f32) {
    let mut x: f32;
    let mut y: f32;
    let mut z: f32;

    x = vec.x;
    y = vec.y;
    z = vec.z;

    y = y - alpha * x;
    x = x + alpha * y;
    y = y - beta * z;
    z = z + beta * y;

    vec.x = x;
    vec.y = y;
    vec.z = z;
}
pub fn update_scanner(universe: &[UnivObject], params: &GameParams) {
    let mut x;
    let mut y;
    let mut z;
    let mut x1;
    let mut y1;
    // let mut z1;
    let mut y2;
    let mut colour;

    for i in 0..MAX_UNIV_OBJECTS {
        if ((universe[i].da_type <= 0)
            || (universe[i].flags & FLG_DEAD) != 0
            || (universe[i].flags & FLG_CLOAKED != 0))
        {
            continue;
        }

        x = universe[i].location.x / 256.0;
        y = universe[i].location.y / 256.0;
        z = universe[i].location.z / 256.0;

        x1 = x;
        y1 = -z / 4.0;
        y2 = y1 - y / 2.0;

        if ((y2 < -28.0) || (y2 > 28.0) || (x1 < -50.0) || (x1 > 50.0)) {
            continue;
        }

        x1 += params.screen_width * 0.5; //scanner_cx
        y1 += params.screen_height * 0.75; //scanner_cy;
        y2 += params.screen_height * 0.75; //scanner_cy;

        colour = if (universe[i].flags & FLG_HOSTILE) != 0 {
            YELLOW
        } else {
            WHITE
        };

        match (universe[i].da_type) {
            SHIP_MISSILE => colour = PINK,

            SHIP_DODEC => colour = GREEN,
            SHIP_CORIOLIS => colour = GREEN,

            SHIP_VIPER => colour = MAGENTA,
        }

        draw_line(x1 + 2.0, y2, x1 - 3.0, y2, THICKNESS, colour);
        draw_line(x1 + 2.0, y2 + 1.0, x1 - 3.0, y2 + 1.0, THICKNESS, colour);
        draw_line(x1 + 2.0, y2 + 2.0, x1 - 3.0, y2 + 2.0, THICKNESS, colour);
        draw_line(x1 + 2.0, y2 + 3.0, x1 - 3.0, y2 + 3.0, THICKNESS, colour);

        draw_line(x1, y1, x1, y2, THICKNESS, colour);
        draw_line(x1 + 1.0, y1, x1 + 1.0, y2, THICKNESS, colour);
        draw_line(x1 + 2.0, y1, x1 + 2.0, y2, THICKNESS, colour);
    }
}

/*
 * Update the compass which tracks the space station / planet.
 */

pub fn update_compass(
    params: &GameParams,
    ship_count: &[My; NO_OF_SHIPS + 1],
    universe: &[UnivObject],
) {
    let mut dest: Vector;
    let mut un = 0;

    if (params.witchspace) {
        return;
    }

    if (ship_count[SHIP_CORIOLIS as usize] != 0 || ship_count[SHIP_DODEC as usize] != 0) {
        un = 1;
    }

    dest = unit_vector(universe[un].location);

    // let compass_x = compass_centre_x + (dest.x * 16.0);
    // let compass_y = compass_centre_y + (dest.y * -16.0);
    let compass_x = (params.screen_width * 0.66) + (dest.x * 16.0);
    let compass_y = (params.screen_height * 0.66) + (dest.y * -16.0);

    if (dest.z < 0.0) {
        draw_circle(compass_x, compass_y, STAR_SIZE * 2.0, RED);
        // gfx_draw_sprite (IMG_RED_DOT, compass_x, compass_y);
    } else {
        draw_circle(compass_x, compass_y, STAR_SIZE * 2.0, GREEN);
        // gfx_draw_sprite (IMG_GREEN_DOT, compass_x, compass_y);
    }
}

/*
 * Display the speed bar.
 */

pub fn display_speed(params: &GameParams) {
    let sx = 417;
    let sy = 384 + 9;

    let len = ((params.flight_speed * 64) / params.myship.max_speed) - 1;

    let colour = if (params.flight_speed > (params.myship.max_speed * 2 / 3)) {
        RED
    } else {
        GOLD
    };

    for i in 0..6 {
        draw_line(
            sx as f32,
            sy as f32 + i as f32,
            sx as f32 + len as f32,
            sy as f32 + i as f32,
            THICKNESS,
            colour,
        );
    }
}

/*
 * Draw an indicator bar.
 * Used for shields and energy banks.
 */

pub fn display_dial_bar(len: My, x: My, y: My) {
    let mut i = 0;

    draw_line(
        x as f32,
        y as f32 + 384 as f32,
        x as f32 + len as f32,
        y as f32 + 384 as f32,
        THICKNESS * 2.0,
        GOLD,
    );
    i += 1;
    draw_line(
        x as f32,
        y as f32 + i as f32 + 384 as f32,
        x as f32 + len as f32,
        y as f32 + i as f32 + 384 as f32,
        THICKNESS * 2.0,
        GOLD,
    );
    i = 2;
    while i < 7 {
        draw_line(
            x as f32,
            y as f32 + i as f32 + 384 as f32,
            x as f32 + len as f32,
            y as f32 + i as f32 + 384 as f32,
            THICKNESS * 2.0,
            YELLOW,
        );
        i += 1;
    }

    draw_line(
        x as f32,
        y as f32 + i as f32 + 384 as f32,
        x as f32 + len as f32,
        y as f32 + i as f32 + 384 as f32,
        THICKNESS * 2.0,
        RED,
    );
}

/*
 * Display the current shield strengths.
 */

pub fn display_shields(params: &GameParams) {
    if (params.front_shield > 3) {
        display_dial_bar(params.front_shield / 4, 31, 7);
    }

    if (params.aft_shield > 3) {
        display_dial_bar(params.aft_shield / 4, 31, 23);
    }
}

pub fn display_altitude(params: &GameParams) {
    if (params.myship.altitude > 3) {
        display_dial_bar(params.myship.altitude / 4, 31, 92);
    }
}

pub fn display_cabin_temp(params: &GameParams) {
    if (params.myship.cabtemp > 3) {
        display_dial_bar(params.myship.cabtemp / 4, 31, 60);
    }
}

pub fn display_laser_temp(params: &GameParams) {
    if (params.myship.laser_temp > 0) {
        display_dial_bar(params.myship.laser_temp / 4, 31, 76);
    }
}

/*
 * Display the energy banks.
 */

pub fn display_energy(params: &GameParams) {
    let e1 = if params.energy > 64 {
        64
    } else {
        params.energy
    };
    let e2 = if params.energy > 128 {
        64
    } else {
        params.energy - 64
    };
    let e3 = if params.energy > 192 {
        64
    } else {
        params.energy - 128
    };
    let e4 = params.energy - 192;

    if (e4 > 0) {
        display_dial_bar(e4, 416, 61);
    }

    if (e3 > 0) {
        display_dial_bar(e3, 416, 79);
    }

    if (e2 > 0) {
        display_dial_bar(e2, 416, 97);
    }

    if (e1 > 0) {
        display_dial_bar(e1, 416, 115);
    }
}

pub fn display_flight_roll(params: &GameParams) {
    let sx = 416;
    let sy = 384 + 9 + 14;

    let mut pos = sx - ((params.flight_roll * 28) / params.myship.max_roll);
    pos += 32;

    for i in 0..4 {
        draw_line(
            pos as f32 + i as f32,
            sy as f32,
            pos as f32 + i as f32,
            sy as f32 + 7 as f32,
            THICKNESS * 2.0,
            GOLD,
        );
    }
}

pub fn display_flight_climb(params: &GameParams) {
    let sx = 416.0;
    let sy = 384.0 + 9.0 + 14.0 + 16.0;

    let mut pos = sx + ((params.flight_climb * 28) / params.myship.max_climb) as f32;
    pos += 32.0;

    for i in 0..4 {
        draw_line(
            pos + i as f32,
            sy,
            pos + i as f32,
            sy + 7.0,
            THICKNESS * 2.0,
            GOLD,
        );
    }
}

pub fn display_fuel(cmdr: &Commander, params: &GameParams) {
    if (cmdr.fuel > 0) {
        display_dial_bar((cmdr.fuel * 64) / params.myship.max_fuel, 31, 44);
    }
}

pub fn display_missiles(params: &GameParams, cmdr: &Commander) {
    if (cmdr.missiles == 0) {
        return;
    }

    let mut nomiss = if cmdr.missiles > 4 { 4 } else { cmdr.missiles };

    let mut x = (4 - nomiss) * 16 + 35;
    let y = 113 + 385;

    if (params.myship.missile_target != MISSILE_UNARMED) {
        if params.myship.missile_target < 0 {
            draw_rectangle(
                x as f32,
                y as f32,
                params.screen_width * 0.05,
                params.screen_height * 0.5,
                YELLOW,
            );
        } else {
            draw_rectangle(
                x as f32,
                y as f32,
                params.screen_width * 0.05,
                params.screen_height * 0.5,
                RED,
            );
        }
        x += 16;
        nomiss -= 1;
    }

    while nomiss > 0 {
        draw_rectangle(
            x as f32,
            y as f32,
            params.screen_width * 0.05,
            params.screen_height * 0.5,
            GREEN,
        );
        // gfx_draw_sprite(IMG_MISSILE_GREEN, x, y);
        x += 16;
        nomiss -= 1;
    }
}
pub fn update_console(
    params: &GameParams,
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    ship_count: &[My; NO_OF_SHIPS + 1],
    universe: &[UnivObject],
    cmdr: &Commander,
) {
    // gfx_draw_scanner();

    display_speed(params);
    display_flight_climb(params);
    display_flight_roll(params);
    display_shields(params);
    display_altitude(params);
    display_energy(params);
    display_cabin_temp(params);
    display_laser_temp(params);
    display_fuel(cmdr, params);
    display_missiles(params, cmdr);

    if (params.docked) {
        return;
    }

    update_scanner(universe, params);
    update_compass(params, ship_count, universe);

    if (ship_count[SHIP_CORIOLIS as usize] != 0 || ship_count[SHIP_DODEC as usize] != 0) {
        // gfx_draw_sprite(IMG_BIG_S, 387, 490);
    }

    if (params.myship.ecm_active) {
        // gfx_draw_sprite(IMG_BIG_E, 115, 490);
    }
}
