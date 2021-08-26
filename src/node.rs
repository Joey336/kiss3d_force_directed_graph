extern crate nalgebra as na;
extern crate  kiss3d;
use crate::common_funcs;
use kiss3d::nalgebra::{Translation3, Point3};

extern crate rand;
use kiss3d::scene::SceneNode;


pub struct Node{
    position: na::Vector3<f32>,
    friction: f32,
    max_speed: f32,
    velocity: na::Vector3<f32>,
    acceleration: na::Vector3<f32>,
    sphere: kiss3d::scene::SceneNode,
    point: kiss3d::nalgebra::Point3<f32>,
    mass: f32,
    
}


impl Node{
    pub fn new(position: na::Vector3<f32>, mass: f32, sphere: SceneNode) -> Self {
        let mut new_node = Node{
            position: position, //x, y, z
            friction: 0.98,
            max_speed: 5.,
            velocity: na::Vector3::new(0.,0.,0.),
            acceleration: na::Vector3::new(0., 0., 0.,),
            sphere: sphere,
            point: Point3::new(position[0],position[1],position[2]),
            mass: mass,
        };
        //translates sphere object to initial positon
        new_node.sphere.append_translation(&Translation3::new(new_node.position.x, new_node.position.y, new_node.position.z));

        new_node
    }

    // F = M * A 
    //updates acceleration of node
    pub fn accelerate(&mut self, force: na::Vector3<f32>){
        // A = F / M
        self.acceleration = self.acceleration + (force / self.mass);
    }

    //applies acceleration to velocity, applies velocity of node's position then updates sphere object on canvas
    pub fn update_position(&mut  self){
        self.velocity += self.acceleration;
        self.velocity *=self.friction;  //reduce velocity by applying friction
        
        //if current velocity > max_speed, set velocity to max speed (to prevent extreme rubber banding in some graphs)
        if common_funcs::get_magnitude(self.velocity) > self.max_speed{
            self.velocity = common_funcs::set_magnitude(self.velocity, self.max_speed);
        }
        
        //sets back to origin (look further into kiss3d so you dont have to use translations)
        self.sphere.append_translation(&Translation3::new(-self.position.x, -self.position.y, -self.position.z));
     
        self.position += self.velocity;
   
        //resets accel
        self.acceleration.x = 0.;
        self.acceleration.y = 0.;
        self.acceleration.z = 0.;

        //appends translation to sphere canvas object at the x,y,z (from origin)
        self.sphere.append_translation(&Translation3::new(self.position.x, self.position.y, self.position.z));

        //updates position of point object (kiss3d draws lines between point objects so these are how visible edges are drawn in graph)
        self.point.x = self.position[0];
        self.point.y = self.position[1];
        self.point.z = self.position[2];

    }

    pub fn get_position(&self) -> na::Vector3<f32>{
        self.position
    }

    pub fn get_point(&self) -> kiss3d::nalgebra::Point3<f32>{
        self.point
    }
   



}

