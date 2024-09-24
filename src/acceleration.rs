use crate::venom_core::*;

impl OctreeNode {
    pub fn new(bounds: BoundingBox) -> Self {
        Self {
            bounds,
            vertices: Vec::new(),
            children: None,
        }
    }

    pub fn insert(&mut self, vertex: Vertex, max_depth: usize, current_depth: usize) {
        if current_depth == max_depth || self.children.is_none() {
            self.vertices.push(vertex);
        } else {
            let center = self.bounds.center();
            let mut octants = [
                BoundingBox {
                    min: self.bounds.min,
                    max: center,
                },
                BoundingBox {
                    min: (center.0, self.bounds.min.1, self.bounds.min.2),
                    max: (self.bounds.max.0, center.1, center.2),
                },
                BoundingBox {
                    min: (self.bounds.min.0, center.1, self.bounds.min.2),
                    max: (center.0, self.bounds.max.1, center.2),
                },
                BoundingBox {
                    min: (self.bounds.min.0, self.bounds.min.1, center.2),
                    max: (center.0, center.1, self.bounds.max.2),
                },
                BoundingBox {
                    min: center,
                    max: self.bounds.max,
                },
                BoundingBox {
                    min: (self.bounds.min.0, center.1, center.2),
                    max: (center.0, self.bounds.max.1, self.bounds.max.2),
                },
                BoundingBox {
                    min: (center.0, self.bounds.min.1, center.2),
                    max: (self.bounds.max.0, center.1, self.bounds.max.2),
                },
                BoundingBox {
                    min: (center.0, center.1, self.bounds.min.2),
                    max: (self.bounds.max.0, self.bounds.max.1, center.2),
                },
            ];

            if self.children.is_none() {
                self.children = Some(Box::new([
                    OctreeNode::new(octants[0].clone()),
                    OctreeNode::new(octants[1].clone()),
                    OctreeNode::new(octants[2].clone()),
                    OctreeNode::new(octants[3].clone()),
                    OctreeNode::new(octants[4].clone()),
                    OctreeNode::new(octants[5].clone()),
                    OctreeNode::new(octants[6].clone()),
                    OctreeNode::new(octants[7].clone()),
                ]));
            }

            for child in self.children.as_mut().unwrap().iter_mut() {
                if child.bounds.contains_point(&vertex.position) {
                    child.insert(vertex, max_depth, current_depth + 1);
                    break;
                }
            }
        }
    }

    pub fn build_octree(scene: &Scene, max_depth: usize) -> OctreeNode {
        let mut root_bounds = BoundingBox {
            min: (Scalar::MAX, Scalar::MAX, Scalar::MAX),
            max: (Scalar::MIN, Scalar::MIN, Scalar::MIN),
        };

        // Determine the bounding box for the entire scene
        for model in &scene.models {
            for mesh in &model.meshes {
                for vertex in &mesh.vertices {
                    root_bounds.min.0 = root_bounds.min.0.min(vertex.position.0);
                    root_bounds.min.1 = root_bounds.min.1.min(vertex.position.1);
                    root_bounds.min.2 = root_bounds.min.2.min(vertex.position.2);
                    root_bounds.max.0 = root_bounds.max.0.max(vertex.position.0);
                    root_bounds.max.1 = root_bounds.max.1.max(vertex.position.1);
                    root_bounds.max.2 = root_bounds.max.2.max(vertex.position.2);
                }
            }
        }

        let mut root_node = OctreeNode::new(root_bounds);

        // Insert all vertices into the octree
        for model in &scene.models {
            for mesh in &model.meshes {
                for vertex in &mesh.vertices {
                    root_node.insert(vertex.clone(), max_depth, 0);
                }
            }
        }

        root_node
    }
}
