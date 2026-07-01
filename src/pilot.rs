/*
 * Elite - The New Kind.
 *
 * Reverse engineered from the BBC disk version of Elite.
 * Additional material by C.J.Pinder.
 *
 * The original Elite code is (C) I.Bell & D.Braben 1984.
 * This version re-engineered in C by C.J.Pinder 1999-2001.
 *
 * email: <christian@newkind.co.uk>
 *
 *
 */

/*
 * pilot.c
 *
 * The auto-pilot code.  Used for docking computers and for
 * flying other ships to and from the space station.
 */

/*
 * In the original Elite this code was mixed in with the tactics routines.
 * I have split it out to make it more understandable and easier to maintain.
 */

/*
 * Fly to a given point in space.
 */

use macroquad::{audio::Sound, prelude::rand};

use crate::{
    FLG_ANGRY, FLG_BOLD, FLG_DEAD, FLG_FIRING, FLG_FLY_TO_PLANET, FLG_FLY_TO_STATION, FLG_HAS_ECM,
    FLG_HOSTILE, FLG_INACTIVE, FLG_POLICE, FLG_SLOW, GameParams, My, auto_pilot_ship,
    elite::{Commander, ShipData},
    info_message,
    shipdata::{
        NO_OF_SHIPS, SHIP_ANACONDA, SHIP_CORIOLIS, SHIP_DODEC, SHIP_ESCAPE_CAPSULE, SHIP_HERMIT,
        SHIP_MISSILE, SHIP_PLANET, SHIP_SIDEWINDER, SHIP_SUN, SHIP_THARGLET, SHIP_THARGOID,
        SHIP_VIPER, SHIP_WORM,
    },
    sound::{SND_BLUE_DANUBE, SND_INCOMMING_FIRE_1, SND_INCOMMING_FIRE_2},
    space::{DaType, UnivObject, damage_ship},
    stars::rand255,
    swat::{launch_enemy, missile_tactics, snd_play_sample, track_object},
    vector::{START_VECTOR, Vector, unit_vector, vector_dot_product},
};
pub fn fly_to_vector(ship: &mut UnivObject, vec: &Vector) {
    let mut nvec: Vector = START_VECTOR;
    let mut rat = 3;
    let mut rat2 = 0.1666;
    let mut cnt2 = 0.8055;

    let nvec = unit_vector(&vec);
    let mut direction = vector_dot_product(&nvec, &ship.rotmat[2]);

    if (direction < -0.6666) {
        rat2 = 0.0;
    }

    let mut dir = vector_dot_product(&nvec, &ship.rotmat[1]);

    if (direction < -0.861) {
        ship.rotx = if (dir < 0.0) { 7 } else { -7 };
        ship.rotz = 0;
        return;
    }

    ship.rotx = 0;

    if (((dir).abs() * 2.0) >= rat2) {
        ship.rotx = if (dir < 0.0) { rat } else { -rat };
    }

    if ((ship.rotz).abs() < 16) {
        dir = vector_dot_product(&nvec, &ship.rotmat[0]);

        ship.rotz = 0;

        if (((dir).abs() * 2.0) >= rat2) {
            ship.rotz = if (dir < 0.0) { rat } else { -rat };

            if (ship.rotx < 0) {
                ship.rotz = -ship.rotz;
            }
        }
    }

    if (direction <= -0.167) {
        ship.acceleration = -1;
        return;
    }

    if (direction >= cnt2) {
        ship.acceleration = 3;
        return;
    }
}

/*
 * Fly towards the planet.
 */

pub fn fly_to_planet(ship: &mut UnivObject, universe: &mut [UnivObject]) {
    let mut vec: Vector = START_VECTOR;

    vec.x = universe[0].location.x - ship.location.x;
    vec.y = universe[0].location.y - ship.location.y;
    vec.z = universe[0].location.z - ship.location.z;

    fly_to_vector(ship, &vec);
}

/*
 * Fly to a point in front of the station docking bay.
 * Done prior to the final stage of docking.
 */

pub fn fly_to_station_front(ship: &mut UnivObject, universe: &mut [UnivObject]) {
    let mut vec: Vector = START_VECTOR;

    vec.x = universe[1].location.x - ship.location.x;
    vec.y = universe[1].location.y - ship.location.y;
    vec.z = universe[1].location.z - ship.location.z;

    vec.x += universe[1].rotmat[2].x * 768.0;
    vec.y += universe[1].rotmat[2].y * 768.0;
    vec.z += universe[1].rotmat[2].z * 768.0;

    fly_to_vector(ship, &vec);
}

/*
 * Fly towards the space station.
 */

pub fn fly_to_station(ship: &mut UnivObject, universe: &mut [UnivObject]) {
    let mut vec: Vector = START_VECTOR;

    vec.x = universe[1].location.x - ship.location.x;
    vec.y = universe[1].location.y - ship.location.y;
    vec.z = universe[1].location.z - ship.location.z;

    fly_to_vector(ship, &vec);
}

/*
 * Final stage of docking.
 * Fly into the docking bay.
 */

pub fn fly_to_docking_bay(ship: &mut UnivObject, universe: &mut [UnivObject]) {
    let mut diff: Vector = START_VECTOR;

    diff.x = ship.location.x - universe[1].location.x;
    diff.y = ship.location.y - universe[1].location.y;
    diff.z = ship.location.z - universe[1].location.z;

    let mut vec = unit_vector(&diff);

    ship.rotx = 0;

    if (ship.da_type < 0) {
        ship.rotz = 1;
        if (((vec.x >= 0.0) && (vec.y >= 0.0)) || ((vec.x < 0.0) && (vec.y < 0.0))) {
            ship.rotz = -ship.rotz;
        }

        if ((vec.x).abs() >= 0.0625) {
            ship.acceleration = 0;
            ship.velocity = 1;
            return;
        }

        if ((vec.y).abs() > 0.002436) {
            ship.rotx = if (vec.y < 0.0) { -1 } else { 1 };
        }

        if ((vec.y).abs() >= 0.0625) {
            ship.acceleration = 0;
            ship.velocity = 1;
            return;
        }
    }

    ship.rotz = 0;

    let dir = vector_dot_product(&ship.rotmat[0], &universe[1].rotmat[1]);

    if ((dir).abs() >= 0.9166) {
        ship.acceleration += 1;
        ship.rotz = 127;
        return;
    }

    ship.acceleration = 0;
    ship.rotz = 0;
}

pub fn engage_auto_pilot(params: &mut GameParams, danube: &Sound) {
    if params.auto_pilot || params.witchspace || params.hyper_ready {
        return;
    }

    params.auto_pilot = true;
    snd_play_sample(danube);
}

fn snd_play_midi(_da_midi: usize, _arg: i32) {
    println!("snd_play_midi")
}

pub fn disengage_auto_pilot(params: &mut GameParams) {
    if params.auto_pilot {
        params.auto_pilot = false;
        snd_stop_midi();
    }
}

fn snd_stop_midi() {
    println!("snd_stop_midi")
}
pub fn tactics(
    un: DaType,
    universe: &mut [UnivObject],
    params: &mut GameParams,
    ship_count: &mut [My; NO_OF_SHIPS + 1],
    cmdr: &Commander,
    ship_list: &[ShipData; NO_OF_SHIPS + 1],
    incoming_1_sfx: &Sound,
    incoming_2_sfx: &Sound,
    explode_sfx: &Sound,
) {
    let cnt2 = 0.223;

    let mut ship = universe[un as usize];
    let da_type = ship.da_type;
    let mut flags = ship.flags;

    if ((da_type as DaType == SHIP_PLANET) || (da_type as DaType == SHIP_SUN)) {
        return;
    }
    if (flags & FLG_DEAD) != 0 {
        return;
    }
    if (flags & FLG_INACTIVE) != 0 {
        return;
    }

    if (da_type == SHIP_MISSILE) {
        if (flags & FLG_ANGRY) != 0 {
            missile_tactics(un, universe, params, explode_sfx);
        }
        return;
    }

    if (((un as u8 ^ params.mcount) & 7) != 0) {
        return;
    }

    if ((da_type == SHIP_CORIOLIS) || (da_type == SHIP_DODEC)) {
        if (flags & FLG_ANGRY) != 0 {
            if ((rand::gen_range(0, i32::MAX) & 255) < 240) {
                return;
            }

            if (ship_count[SHIP_VIPER as usize] >= 4) {
                return;
            }

            launch_enemy(
                un,
                SHIP_VIPER,
                FLG_ANGRY | FLG_HAS_ECM,
                113,
                universe,
                ship_list,
                ship_count,
            );
            return;
        }

        // launch_shuttle ();
        return;
    }

    if (da_type == SHIP_HERMIT) {
        if (rand255() > 200) {
            launch_enemy(
                un,
                SHIP_SIDEWINDER + (rand255() as DaType & 3),
                FLG_ANGRY | FLG_HAS_ECM,
                113,
                universe,
                ship_list,
                ship_count,
            );
            ship.flags |= FLG_INACTIVE;
        }

        return;
    }

    if (ship.energy < ship_list[da_type as usize].energy) {
        ship.energy += 1;
    }

    if ((da_type == SHIP_THARGLET) && (ship_count[SHIP_THARGOID as usize] == 0)) {
        ship.flags = 0;
        ship.velocity /= 2;
        return;
    }

    if (flags & FLG_SLOW) != 0 {
        if (rand255() > 50) {
            return;
        }
    }

    if (flags & FLG_POLICE) != 0 {
        if (cmdr.legal_status >= 64) {
            flags |= FLG_ANGRY;
            ship.flags = flags;
        }
    }

    if ((flags & FLG_ANGRY) == 0) {
        if ((flags & FLG_FLY_TO_PLANET) != 0 || (flags & FLG_FLY_TO_STATION) != 0) {
            let mut ship_clone = universe[un as usize];
            auto_pilot_ship(&mut ship_clone, universe, ship_count);
            universe[un as usize] = ship_clone;
        }

        return;
    }

    /* If we get to here then the ship is angry so start attacking... */

    if (ship_count[SHIP_CORIOLIS as usize] != 0 || ship_count[SHIP_DODEC as usize] != 0) {
        if ((flags & FLG_BOLD) == 0) {
            ship.bravery = 0;
        }
    }

    if (da_type == SHIP_ANACONDA) {
        if (rand255() > 200) {
            launch_enemy(
                un,
                if rand255() > 100 {
                    SHIP_WORM
                } else {
                    SHIP_SIDEWINDER
                },
                FLG_ANGRY | FLG_HAS_ECM,
                113,
                universe,
                ship_list,
                ship_count,
            );
            return;
        }
    }

    if (rand255() >= 250) {
        ship.rotz = rand255() | 0x68;
        if (ship.rotz > 127) {
            ship.rotz = -(ship.rotz & 127);
        }
    }

    let maxeng = ship_list[da_type as usize].energy;
    let energy = ship.energy;

    if (energy < (maxeng / 2)) {
        if ((energy < (maxeng / 8)) && (rand255() > 230) && (da_type != SHIP_THARGOID)) {
            ship.flags &= !FLG_ANGRY;
            ship.flags = ship.flags & !FLG_ANGRY;
            ship.flags |= FLG_INACTIVE;
            launch_enemy(
                un,
                SHIP_ESCAPE_CAPSULE,
                0,
                126,
                universe,
                ship_list,
                ship_count,
            );
            return;
        }

        if ((ship.missiles != 0)
            && (!params.myship.ecm_active)
            && (ship.missiles >= (rand255() & 31)))
        {
            ship.missiles -= 1;
            if (da_type == SHIP_THARGOID) {
                launch_enemy(
                    un,
                    SHIP_THARGLET,
                    FLG_ANGRY,
                    ship.bravery,
                    universe,
                    ship_list,
                    ship_count,
                );
            } else {
                launch_enemy(
                    un,
                    SHIP_MISSILE,
                    FLG_ANGRY,
                    126,
                    universe,
                    ship_list,
                    ship_count,
                );
                info_message("INCOMING MISSILE".to_string(), params);
            }
            return;
        }
    }

    let mut nvec = unit_vector(&universe[un as usize].location);
    let mut direction = vector_dot_product(&nvec, &ship.rotmat[2]);

    if ((ship.distance < 8192)
        && (direction <= -0.833)
        && (ship_list[da_type as usize].laser_strength != 0))
    {
        if (direction <= -0.917) {
            ship.flags |= FLG_FIRING | FLG_HOSTILE;
        }

        if (direction <= -0.972) {
            damage_ship(
                ship_list[da_type as usize].laser_strength,
                {
                    if (ship.location.z >= 0.0) {
                        true
                    } else {
                        false
                    }
                },
                params,
            );
            ship.acceleration -= 1;
            if (((ship.location.z >= 0.0) && (params.front_shield == 0))
                || ((ship.location.z < 0.0) && (params.aft_shield == 0)))
            {
                snd_play_sample(incoming_2_sfx);
            } else {
                snd_play_sample(incoming_1_sfx);
            }
        } else {
            nvec.x = -nvec.x;
            nvec.y = -nvec.y;
            nvec.z = -nvec.z;
            direction = -direction;
            track_object(&mut universe[un as usize], direction, nvec);
        }

        //		if ((fabs(ship.location.z) < 768) && (ship.bravery <= ((rand255() & 127) | 64)))
        if ((ship.location.z).abs() < 768.0) {
            ship.rotx = rand255() & 0x87;
            if (ship.rotx > 127) {
                ship.rotx = -(ship.rotx & 127);
            }
            ship.acceleration = 3;
            return;
        }

        if (ship.distance < 8192) {
            ship.acceleration = -1;
        } else {
            ship.acceleration = 3;
        }
        return;
    }

    let mut attacking = 0;

    if (((ship.location.z).abs() >= 768.0)
        || ((ship.location.x).abs() >= 512.0)
        || ((ship.location.y).abs() >= 512.0))
    {
        if (ship.bravery > (rand255() & 127)) {
            attacking = 1;
            nvec.x = -nvec.x;
            nvec.y = -nvec.y;
            nvec.z = -nvec.z;
            direction = -direction;
        }
    }

    track_object(&mut universe[un as usize], direction, nvec);

    if ((attacking == 1) && (ship.distance < 2048)) {
        if (direction >= cnt2) {
            ship.acceleration = -1;
            return;
        }

        if (ship.velocity < 6) {
            ship.acceleration = 3;
        } else {
            if (rand255() >= 200) {
                ship.acceleration = -1;
            }
            return;
        }
    }

    if (direction <= -0.167) {
        ship.acceleration = -1;
        return;
    }

    if (direction >= cnt2) {
        ship.acceleration = 3;
        return;
    }

    if (ship.velocity < 6) {
        ship.acceleration = 3;
    } else {
        if (rand255() >= 200) {
            ship.acceleration = -1;
        }
    }
    universe[un as usize] = ship;
}
