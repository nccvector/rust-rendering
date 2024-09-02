### Scalar
Application uses a commonly defined type called scalar, instead of f32 or f64. The application can re-target this Scalar on to low or high precision data types.

### Transform
Transform is a 4x4 matrix of scalar type.

### Vertex
A Vertex contains:
- position
- normals
- uvs

### Face
A face contains 3 indices of vertices that make up a triangle.

### Acceleration Structure
Acceleration structure provides the following implementations:
- `intersectRay`

There are two types of acceleration structures:
- Top Acceleration Structure: this is scene level, the leaf nodes are AABB of models.
- Bottom Acceleration Structure: this is Mesh level, the leaf nodes are faces of mesh.

### Resource Manager
A global application wide resource manager is used to add or remove resources. The resources managed are:
- Textures
- Materials
- Mesh
- Models

### Mesh
Mesh contains:
- `Vec[Vertex]`
- `Vec[Face]`
- BottomLevelAccelerationStructure

Mesh provides a `build` implementation, that initializes or updates its acceleration structure.

### Texture
Textures are maintained by resource manager. And are loaded into memory when the application starts, until the application's lifetime ends.

### Material
Materials will be PBR only, and will contain PBR fields. In addition to common PBR fields, they will also contain references to 

### Model
Model consists of the following:
- `Vec[ref Material]`
- `Vec[ref Texture]`
- `Vec[ref Mesh]`
- Transform
- AABB

### Scene
A scene consists of:
- `Vec[ref Model]`
- TopLevelAccelerationStructure

Scene provides following implementations:
- `build`: build TopLevelAccelerationStructure
