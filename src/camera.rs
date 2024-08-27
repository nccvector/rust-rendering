use nalgebra::{Matrix3, Matrix4, Rotation3, Vector3};

use itertools::Itertools;

fn ComputeCameraMatrix(verticalFOVDegrees: f32, imageWidth: f32, imageHeight: f32) -> Matrix3<f32> {
    // Convert vertical FOV from degrees to radians
    let verticalFOVRadians = verticalFOVDegrees.to_radians();

    // Compute image plane distance and horizontal FOV
    let baseDistance = (imageHeight / 2.0) / (verticalFOVRadians / 2.0).tan();    // distance of image plane from camera origin
    let horizontalFOVRadians = 2.0 * ((imageWidth / 2.0) / baseDistance).atan();

    // Compute camera matrix parameters
    let fy = (imageHeight - 1.0) / (2.0 * (verticalFOVRadians / 2.0).tan());
    let fx = (imageWidth - 1.0) / (2.0 * (horizontalFOVRadians / 2.0).tan());

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

fn GenerateHomogenousPixelCoordinates(imageWidth: u32, imageHeight: u32) -> impl Iterator<Item=Vector3<f32>> {
    (0..imageHeight).cartesian_product(0..imageWidth)
                    .map(|(y, x)| Vector3::new(x as f32, y as f32, 1.0))
}

pub struct Camera {
    transform: Matrix4<f32>,
    cameraMatrix: Matrix3<f32>,
    cameraMatrixInverse: Matrix3<f32>,
    pub verticalFov: f32,
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
            verticalFov,
            imageWidth,
            imageHeight,
        }
    }

    pub fn getRays(&mut self) -> Vec<(f32, f32, f32, f32, f32, f32)> {
        // Compute the rotation matrix for all pixels
        let rot = Rotation3::from_axis_angle(&Vector3::y_axis(), 180.0_f32.to_radians());
        let rotationMatrix = rot.matrix();

        GenerateHomogenousPixelCoordinates(self.imageWidth as u32, self.imageHeight as u32)
            .map(|pixelCoords| {
                let direction = rotationMatrix * (&self.cameraMatrixInverse * pixelCoords).normalize();
                (
                    0.0, 0.0, 10.0,
                    direction[0], direction[1], direction[2]
                )
            })
            .collect()
    }

    pub fn resize(&mut self, imageWidth: f32, imageHeight: f32) {
        self.imageWidth = imageWidth;
        self.imageHeight = imageHeight;
        self.recomputeCameraMatrix();
    }

    pub fn setFov(&mut self, verticalFov: f32) {
        self.verticalFov = verticalFov;
        self.recomputeCameraMatrix();
    }

    fn recomputeCameraMatrix(&mut self) {
        let _cameraMatrix = ComputeCameraMatrix(self.verticalFov, self.imageWidth, self.imageHeight);
        self.cameraMatrix = _cameraMatrix;
        self.cameraMatrixInverse = _cameraMatrix.try_inverse().unwrap();
    }
}
