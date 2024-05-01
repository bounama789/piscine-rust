mod areas_volumes;

use areas_volumes::{
    circle_area, cone_volume, cube_volume, parallelepiped_volume, rectangle_area, sphere_volume, square_area, triangle_area, triangular_pyramid_volume
};
pub use areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit(
    x: usize,
    y: usize,
    objects: areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    match objects {
        areas_volumes::GeometricalShapes::Square => square_area(a) * times <= rectangle_area(x, y),
        areas_volumes::GeometricalShapes::Triangle => {
            triangle_area(a, b) * times as f64 <= rectangle_area(x, y) as f64
        }
        areas_volumes::GeometricalShapes::Circle => circle_area(a) * times as f64 <= rectangle_area(x, y) as f64,
        areas_volumes::GeometricalShapes::Rectangle => {
            rectangle_area(a, b) * times <= rectangle_area(x, y)
        }
    }
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    match objects {
        areas_volumes::GeometricalVolumes::Cube => {
            cube_volume(a) * times <= (y*x*z)
        }
        areas_volumes::GeometricalVolumes::Sphere => {
            sphere_volume(a) * times as f64 <= (y*x*z) as f64
        }
        areas_volumes::GeometricalVolumes::Pyramid => {
            triangular_pyramid_volume(a as f64, b) * times as f64
                <= (y*x*z) as f64
        }
        areas_volumes::GeometricalVolumes::Parallelepiped => {
            parallelepiped_volume(a, b, c) * times <= (y*x*z)
        }
        areas_volumes::GeometricalVolumes::Cone => {
            cone_volume(a, b) * times as f64 <= (y*x*z) as f64
        }
    }
}
