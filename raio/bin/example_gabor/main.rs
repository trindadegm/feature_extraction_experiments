use raio::{
    gabor::Gabor,
    nalgebra::DMatrix,
};

fn main() {
    let output = std::env::args().nth(1)
        .expect("Missing output file argument");

    let mut kernel = DMatrix::<f32>::zeros(400, 400);

    use std::f32::consts::*;

    let frequency = 4.0;

    for i in 0..400 {
        for j in 0..400 {
            kernel[(i, j)] = 4.0 * Gabor::sample_2d(
                i as f32 / 200.0 - 1.0,
                j as f32 / 200.0 - 1.0,
                0.0,
                frequency,
                0.0,
                0.4,
                1.0,
            ) * Gabor::sample_2d(
                i as f32 / 200.0 - 1.0,
                j as f32 / 200.0 - 1.0,
                PI/4.0,
                frequency,
                0.0,
                0.4,
                1.0,
            ) * Gabor::sample_2d(
                i as f32 / 200.0 - 1.0,
                j as f32 / 200.0 - 1.0,
                PI/2.0,
                frequency,
                0.0,
                0.4,
                1.0,
            ) * Gabor::sample_2d(
                i as f32 / 200.0 - 1.0,
                j as f32 / 200.0 - 1.0,
                3.0*PI/4.0,
                frequency,
                0.0,
                0.4,
                1.0,
            );
        }
    }

    let img_kernel: Vec<u8> =
        kernel.data.as_slice().iter().map(|fvalue| ((fvalue + 1.0) * 127.5) as u8).collect();

    image::save_buffer(output, &img_kernel, 400, 400, image::ColorType::L8)
        .expect("Failed to save image");
}