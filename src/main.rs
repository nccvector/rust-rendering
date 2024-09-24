mod venom_core;
mod objloader;
mod acceleration;

use crate::venom_core::*;
use crate::objloader::*;
use crate::acceleration::*;


fn main() {
    let scene = Scene::new();
    Scene::load_obj("assets/Sponza/sponza.obj").unwrap();

    let max_depth = 10;
    let octree = OctreeNode::build_octree(&scene, max_depth);
}