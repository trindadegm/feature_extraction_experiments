use std::f32::consts::PI as PI_F32;

pub trait Gabor {
    fn sample_2d(
        x: Self,
        y: Self,
        orientation: Self,
        frequency: Self,
        phase_shift: Self,
        standard_deviation: Self,
        spatial_aspect_ratio: Self,
    ) -> Self;
}

impl Gabor for f32 {
    fn sample_2d(
        x: f32,
        y: f32,
        orientation: f32,
        frequency: f32,
        phase_shift: f32,
        standard_deviation: f32,
        spatial_aspect_ratio: f32,
    ) -> f32 {
        let x_rot = f32::cos(orientation)*x - f32::sin(orientation)*y;
        let y_rot = f32::sin(orientation)*x + f32::cos(orientation)*y;
        let g = f32::exp(-(x_rot*x_rot + spatial_aspect_ratio*(y_rot*y_rot))/(2.0*standard_deviation));
        let w = f32::cos(2.0*PI_F32*frequency*x_rot + phase_shift);
        g * w
    }
}
