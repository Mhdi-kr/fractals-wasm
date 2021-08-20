extern crate image;

use image::{RgbImage, Rgb};
const EXPORT_IMAGE_WIDTH: u32 = 40000;
const EXPORT_IMAGE_HEIGHT: u32 = 20000;
const MAX_ITERATIONS: i32 = 300;

fn main() {
    let mut image = RgbImage::new(EXPORT_IMAGE_WIDTH, EXPORT_IMAGE_HEIGHT);
        for y in 0..EXPORT_IMAGE_HEIGHT {
            for x in 0..EXPORT_IMAGE_WIDTH {
                let scaled_x_coordinate_to_range = scale_value(x as f32, 0 as f32, EXPORT_IMAGE_WIDTH as f32, -2 as f32,  0 as f32);
                let scaled_y_coordinate_to_range = scale_value(y as f32, 0 as f32,  EXPORT_IMAGE_HEIGHT as f32, -1 as f32, 0 as f32);
                let mut stored_scaled_x = 0 as f32;
                let mut stored_scaled_y = 0 as f32;
                let mut iteration_number = 0;
                while iteration_number <= MAX_ITERATIONS && ((stored_scaled_x * stored_scaled_x) + (scaled_y_coordinate_to_range * scaled_y_coordinate_to_range) <= 16 as f32)  {
                    let temporary_x_value = (stored_scaled_x * stored_scaled_x) - (stored_scaled_y * stored_scaled_y) + scaled_x_coordinate_to_range;
                    stored_scaled_y = (2 as f32 * stored_scaled_x * stored_scaled_y) + scaled_y_coordinate_to_range;
                    stored_scaled_x = temporary_x_value;
                    iteration_number += 1;
                }
                let color = scale_value(iteration_number as f32,0 as f32,MAX_ITERATIONS as f32, 0 as f32, 255 as f32) as u8;
                image.put_pixel(x, y, Rgb([color, color, color]));
            }
        }
    image.save("output.png").unwrap();
}

fn scale_value(variable_in_first_range: f32,first_range_x: f32, first_range_y: f32,second_range_x: f32, second_range_y: f32) -> f32 {
    let scaled_variable = (((variable_in_first_range - first_range_x) / (first_range_y - first_range_x)) * (second_range_y - second_range_x)) + second_range_x;
    return scaled_variable;
}