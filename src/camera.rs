use nalgebra::{Matrix3, Matrix4, Vector3};

use crate::vec_ops::*;

fn ComputeCameraMatrix(verticalFOVDegrees: f32, imageWidth: f32, imageHeight: f32) -> Matrix3<f32> {
    // Convert vertical FOV from degrees to radians
    let verticalFOVRadians = verticalFOVDegrees.to_radians();
    let aspectRatio = imageWidth / imageHeight;
    let horizontalFOVRadians = aspectRatio * verticalFOVRadians;


    let fx = imageWidth / (2.0 * (horizontalFOVRadians / 2.0).tan());
    let fy = imageHeight / (2.0 * (verticalFOVRadians / 2.0).tan());

    // Compute the image center
    let cx = (imageWidth - 1.0) / 2.0;
    let cy = (imageHeight - 1.0) / 2.0;

    // Intrinsic camera matrix
    let cameraMatrix: Matrix3<f32> = Matrix3::new(
        fx, 0.0, cx,
        0.0, fy, cy,
        0.0, 0.0, 1.0,
    );

    cameraMatrix
}

fn GenerateHomogenousPixelCoordinates(imageWidth: u32, imageHeight: u32) -> Vec<Vector3<f32>> {
    let mut pixels = Vec::new();

    for y in 0..imageHeight {
        for x in 0..imageWidth {
            pixels.push(Vector3::new(x as f32, y as f32, 1.0));
        }
    }

    pixels
}

pub struct Camera {
    transform: Matrix4<f32>,
    cameraMatrix: Matrix3<f32>,
    cameraMatrixInverse: Matrix3<f32>,
    pub imageWidth: f32,
    pub imageHeight: f32,
}

impl Camera {
    pub fn new(transform: Matrix4<f32>, verticalFov: f32, imageWidth: f32, imageHeight: f32) -> Self {
        let _cameraMatrix = ComputeCameraMatrix(verticalFov, imageWidth, imageHeight);
        Self {
            transform,
            cameraMatrix: _cameraMatrix,
            cameraMatrixInverse: _cameraMatrix.try_inverse().unwrap(),
            imageWidth,
            imageHeight,
        }
    }

    pub fn getRays(&mut self) -> Vec<(f32, f32, f32, f32, f32, f32)> {
        // Get pixel coordinates
        let pixelCoordsHomo = GenerateHomogenousPixelCoordinates(self.imageWidth as u32, self.imageHeight as u32);

        // Get rays for all pixels
        // let rayOrigins =
        let rayDirections = pixelCoordsHomo.transform(&self.cameraMatrixInverse).normalize().mul_scalar(-1.0);
        let rayOrigins = rayDirections.zeros().add_vector(Vector3::<f32>::new(0.0, 0.0, 10.0));

        let mut vec: Vec<(f32, f32, f32, f32, f32, f32)> = Vec::with_capacity(rayDirections.len());

        for (origin, direction) in rayOrigins.iter().zip(rayDirections.iter()) {
            vec.push((
                origin[0], origin[1], origin[2],
                direction[0], direction[1], direction[2]
            ));
        }

        vec
    }
}