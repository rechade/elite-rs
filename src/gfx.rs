use macroquad::{
    color::{RED, YELLOW},
    shapes::{draw_circle_lines, draw_ellipse_lines, draw_line},
    text::draw_text,
};

use crate::{GameParams, My, THICKNESS};
pub const GFX_VIEW_TX: My = 1;
pub const GFX_VIEW_TY: My = 1;
pub const GFX_VIEW_BX: My = 1920;
pub const GFX_VIEW_BY: My = 1024;
pub const GFX_SCALE: My = 4;
pub const STAR_SIZE: f32 = 1.0 * GFX_SCALE as f32;
pub const GFX_X_OFFSET: My = 1;
pub const GFX_Y_OFFSET: My = 1;
pub fn gfx_draw_scanner(params: &GameParams, labels: &[&str]) {
    // top line
    // divides screen at 0.75
    let mut row_y_pos = params.row_y_pos; // need a copy to use to step through the rows
    draw_line(
        0.0,
        row_y_pos,
        params.screen_width,
        row_y_pos,
        THICKNESS,
        YELLOW,
    );
    // left side panel
    // side panels are 1/5 each
    // seven rows.

    // left vertical divider
    draw_line(
        params.row_width,
        row_y_pos,
        params.row_width,
        params.screen_height,
        THICKNESS,
        YELLOW,
    );
    // right vertical divider
    draw_line(
        params.screen_width - params.row_width,
        row_y_pos,
        params.screen_width - params.row_width,
        params.screen_height,
        THICKNESS,
        YELLOW,
    );
    // draw the rows and labels
    for i in 0..7 {
        row_y_pos += params.row_inc;
        // left rows
        draw_line(
            0.0,
            row_y_pos,
            params.row_width,
            row_y_pos,
            THICKNESS,
            YELLOW,
        );
        draw_text(
            labels[i * 2],
            params.screen_width * 0.01,
            row_y_pos,
            55.0,
            RED,
        );
        // right rows
        draw_line(
            params.screen_width - params.row_width,
            row_y_pos,
            params.screen_width,
            row_y_pos,
            THICKNESS,
            YELLOW,
        );
        draw_text(
            labels[i * 2 + 1],
            params.screen_width * 0.96,
            row_y_pos,
            55.0,
            RED,
        );
    }
    // scanner
    let ell_w_rad = params.screen_width * 0.2 * 0.25;
    let ell_h_rad = params.screen_height * 0.1 * 0.25;
    for i in 1..5 {
        draw_ellipse_lines(
            params.scanner_cx,
            params.scanner_cy,
            ell_w_rad * i as f32,
            ell_h_rad * i as f32,
            0.0,
            THICKNESS,
            RED,
        );
    }
    draw_line(
        params.scanner_cx,
        params.scanner_cy,
        params.screen_width * 0.5 - (0.1 * params.screen_width),
        params.screen_height - (params.screen_height * 0.211),
        THICKNESS,
        RED,
    );
    draw_line(
        params.scanner_cx,
        params.scanner_cy,
        params.screen_width * 0.5 + (0.1 * params.screen_width),
        params.screen_height - (params.screen_height * 0.211),
        THICKNESS,
        RED,
    );
    // compass
    draw_circle_lines(
        params.compass_x,
        params.compass_y,
        params.compass_r,
        THICKNESS,
        RED,
    );
    // params.screen_height * 0.875);

    // let size: Vec2 = Vec2 {
    //     x: screen_width(),
    //     y: screen_height() * 0.2,
    // };
    // let texture_params = DrawTextureParams {
    //     dest_size: Some(size),
    //     ..Default::default()
    // };
    // draw_texture_ex(
    //     overlay,
    //     0.0, //params.screen_width * 0.5,
    //     params.screen_height * 0.8,
    //     WHITE,
    //     texture_params,
    // );
}
