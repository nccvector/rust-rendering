use nalgebra::{Matrix3, Matrix4, Vector2, Vector3, Vector4};

pub trait Transform<T, M> {
    fn transform(&self, matrix: &M) -> Vec<T>;
}

impl Transform<Vector3<f32>, Matrix3<f32>> for Vec<Vector3<f32>> {
    fn transform(&self, matrix: &Matrix3<f32>) -> Vec<Vector3<f32>> {
        self.iter().map(|&point| matrix * point).collect()
    }
}

impl Transform<Vector4<f32>, Matrix4<f32>> for Vec<Vector4<f32>> {
    fn transform(&self, matrix: &Matrix4<f32>) -> Vec<Vector4<f32>> {
        self.iter().map(|&point| matrix * point).collect()
    }
}

pub trait Normalize<T> {
    fn normalize(&self) -> Vec<T>;
}

impl Normalize<Vector2<f32>> for Vec<Vector2<f32>> {
    fn normalize(&self) -> Vec<Vector2<f32>> {
        self.iter()
            .map(|v| v.normalize())
            .collect()
    }
}

impl Normalize<Vector3<f32>> for Vec<Vector3<f32>> {
    fn normalize(&self) -> Vec<Vector3<f32>> {
        self.iter()
            .map(|v| v.normalize())
            .collect()
    }
}

impl Normalize<Vector4<f32>> for Vec<Vector4<f32>> {
    fn normalize(&self) -> Vec<Vector4<f32>> {
        self.iter()
            .map(|v| v.normalize())
            .collect()
    }
}

//  Define a trait to handle extending vectors with a ones row.
pub trait AppendOne<T, U> {
    fn append_one(&self) -> Vec<U>;
}

impl AppendOne<Vector2<f32>, Vector3<f32>> for Vec<Vector2<f32>> {
    fn append_one(&self) -> Vec<Vector3<f32>> {
        self.iter()
            .map(|&v| Vector3::new(v[0], v[1], 1.0))
            .collect()
    }
}

impl AppendOne<Vector3<f32>, Vector4<f32>> for Vec<Vector3<f32>> {
    fn append_one(&self) -> Vec<Vector4<f32>> {
        self.iter()
            .map(|&v| Vector4::new(v[0], v[1], v[2], 1.0))
            .collect()
    }
}

pub trait Zeros<T> {
    fn zeros(&self) -> Vec<T>;
}

impl Zeros<Vector2<f32>> for Vec<Vector2<f32>> {
    fn zeros(&self) -> Vec<Vector2<f32>> {
        self.iter()
            .map(|_| Vector2::new(0.0, 0.0))
            .collect()
    }
}

impl Zeros<Vector3<f32>> for Vec<Vector3<f32>> {
    fn zeros(&self) -> Vec<Vector3<f32>> {
        self.iter()
            .map(|_| Vector3::new(0.0, 0.0, 0.0))
            .collect()
    }
}

impl Zeros<Vector4<f32>> for Vec<Vector4<f32>> {
    fn zeros(&self) -> Vec<Vector4<f32>> {
        self.iter()
            .map(|_| Vector4::new(0.0, 0.0, 0.0, 0.0))
            .collect()
    }
}


pub trait ScalarOperations<T> {
    fn add_scalar(&self, scalar: T) -> Self;
    fn sub_scalar(&self, scalar: T) -> Self;
    fn mul_scalar(&self, scalar: T) -> Self;
    fn div_scalar(&self, scalar: T) -> Self;
}

impl ScalarOperations<f32> for Vec<Vector2<f32>> {
    fn add_scalar(&self, scalar: f32) -> Self {
        self.iter().map(|v| v + Vector2::new(scalar, scalar)).collect()
    }

    fn sub_scalar(&self, scalar: f32) -> Self {
        self.iter().map(|v| v - Vector2::new(scalar, scalar)).collect()
    }

    fn mul_scalar(&self, scalar: f32) -> Self {
        self.iter().map(|v| v * scalar).collect()
    }

    fn div_scalar(&self, scalar: f32) -> Self {
        self.iter().map(|v| v / scalar).collect()
    }
}

impl ScalarOperations<f32> for Vec<Vector3<f32>> {
    fn add_scalar(&self, scalar: f32) -> Self {
        self.iter().map(|v| v + Vector3::new(scalar, scalar, scalar)).collect()
    }

    fn sub_scalar(&self, scalar: f32) -> Self {
        self.iter().map(|v| v - Vector3::new(scalar, scalar, scalar)).collect()
    }

    fn mul_scalar(&self, scalar: f32) -> Self {
        self.iter().map(|v| v * scalar).collect()
    }

    fn div_scalar(&self, scalar: f32) -> Self {
        self.iter().map(|v| v / scalar).collect()
    }
}

impl ScalarOperations<f32> for Vec<Vector4<f32>> {
    fn add_scalar(&self, scalar: f32) -> Self {
        self.iter().map(|v| v + Vector4::new(scalar, scalar, scalar, scalar)).collect()
    }

    fn sub_scalar(&self, scalar: f32) -> Self {
        self.iter().map(|v| v - Vector4::new(scalar, scalar, scalar, scalar)).collect()
    }

    fn mul_scalar(&self, scalar: f32) -> Self {
        self.iter().map(|v| v * scalar).collect()
    }

    fn div_scalar(&self, scalar: f32) -> Self {
        self.iter().map(|v| v / scalar).collect()
    }
}

// Define a trait for vector operations (addition, element-wise multiplication, and division)
pub trait VectorOperations<T> {
    fn add_vector(&self, vec: T) -> Self;
    fn mul_vector(&self, vec: T) -> Self;
    fn div_vector(&self, vec: T) -> Self;
}

// Implementation for Vec<Vector2<f32>>
impl VectorOperations<Vector2<f32>> for Vec<Vector2<f32>> {
    fn add_vector(&self, vec: Vector2<f32>) -> Self {
        self.iter().map(|v| v + vec).collect()
    }

    fn mul_vector(&self, vec: Vector2<f32>) -> Self {
        self.iter().map(|v| v.component_mul(&vec)).collect()
    }

    fn div_vector(&self, vec: Vector2<f32>) -> Self {
        self.iter().map(|v| v.component_div(&vec)).collect()
    }
}

// Implementation for Vec<Vector3<f32>>
impl VectorOperations<Vector3<f32>> for Vec<Vector3<f32>> {
    fn add_vector(&self, vec: Vector3<f32>) -> Self {
        self.iter().map(|v| v + vec).collect()
    }

    fn mul_vector(&self, vec: Vector3<f32>) -> Self {
        self.iter().map(|v| v.component_mul(&vec)).collect()
    }

    fn div_vector(&self, vec: Vector3<f32>) -> Self {
        self.iter().map(|v| v.component_div(&vec)).collect()
    }
}

// Implementation for Vec<Vector4<f32>>
impl VectorOperations<Vector4<f32>> for Vec<Vector4<f32>> {
    fn add_vector(&self, vec: Vector4<f32>) -> Self {
        self.iter().map(|v| v + vec).collect()
    }

    fn mul_vector(&self, vec: Vector4<f32>) -> Self {
        self.iter().map(|v| v.component_mul(&vec)).collect()
    }

    fn div_vector(&self, vec: Vector4<f32>) -> Self {
        self.iter().map(|v| v.component_div(&vec)).collect()
    }
}