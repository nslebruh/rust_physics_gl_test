use rapier3d::prelude::{RigidBodyHandle, ColliderHandle};

use crate::mesh::Mesh;

pub struct World {

}

impl World {

}


pub struct Chunk {
  pub empty: bool,
  pub mesh: Mesh,
  pub blocks: [Block; 5832],
  pub rigidbody_handle: RigidBodyHandle,
  pub collider_handle: ColliderHandle,
  position: [i32; 3],
}

impl Chunk {
  //pub fn generate(position: [i32; 3]) -> Self {
  //  
  //}
}

pub struct Block {

}