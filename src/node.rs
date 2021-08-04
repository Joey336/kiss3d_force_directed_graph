extern crate nalgebra as na;
extern crate  kiss3d;
use kiss3d::nalgebra::{Point, Point2, Point3, Translation3, UnitQuaternion, Vector3};

extern crate rand;
use rand::prelude::*;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::text::Font;
use kiss3d::window::Window;
use std::collections::HashMap;
use crate::spring;
use crate::common_funcs;

pub struct Node{
    position: na::Vector3<f32>,
    friction: f32,
    max_speed: f32,
    velocity: na::Vector3<f32>,
    acceleration: na::Vector3<f32>,
    size: f32,
    sphere: kiss3d::scene::SceneNode,
    
}


impl Node{
    pub fn new(position: na::Vector3<f32>,mut sphere: SceneNode) -> Self {
        let mut new_node = Node{
            position: position,
            friction: 0.98,
            max_speed: 5.,
            velocity: na::Vector3::new(0.,0.,0.),
            acceleration: na::Vector3::new(0., 0., 0.,),
            size: 1.,
            sphere: sphere,
      
        };
        //translates sphere object to initial positon
        new_node.sphere.append_translation(&Translation3::new(new_node.position.x, new_node.position.y, new_node.position.z));

        new_node
    }

    // F = M * A (mass = 1 so far)
    pub fn accelerate(&mut self, force: na::Vector3<f32>){
        self.acceleration = self.acceleration + force;
    }

    pub fn updatePosition(&mut  self){
        self.velocity += self.acceleration;
        self.velocity *=self.friction;  //reduce velocity by applying friction
        if common_funcs::getMagnitude(self.velocity) > self.max_speed{
            self.velocity = common_funcs::setMagnitude(self.velocity, self.max_speed);
        }

        //sets back to origin (look further into kiss3d so you dont have to use translations)
        self.sphere.append_translation(&Translation3::new(-self.position.x, -self.position.y, -self.position.z));
     
        self.position += self.velocity;
   
        //resets accel (fix later so you dont make a new vec3 each time)
        self.acceleration = na::Vector3::new(0.,0.,0.);
        
        //appends translation to position sphere at the x,y,z (from origin)
        self.sphere.append_translation(&Translation3::new(self.position.x, self.position.y, self.position.z))

    }

    pub fn getPosition(&self) -> na::Vector3<f32>{
        self.position
    }
    pub fn getAcceleration(&self) -> na::Vector3<f32>{
        self.acceleration
    }




}

