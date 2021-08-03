use crate::node;
use crate::main;
use crate::spring;
use kiss3d::light::Light;
use kiss3d::text::Font;
use kiss3d::window::Window;
extern crate rand;
use rand::random;
use rand::prelude::*;

pub fn create_graph(){
    let mut window = Window::new("Kiss3d: force_directed_graph");
    window.set_background_color(0.5, 0.5, 0.5);
    window.set_light(Light::StickToCamera);

    let mut nodes: Vec<node::Node> = Vec::new();

    
    let mut test12 = node::Node::new(na::Vector3::new(0., 0., 0.),window.add_sphere(1.));
    let mut test22 = node::Node::new(na::Vector3::new(5., 5., 5.),window.add_sphere(1.));

    let mut test1 = node::Node::new(na::Vector3::new(5., 0., 7.),window.add_sphere(1.));
    let mut test2 = node::Node::new(na::Vector3::new(3., 5., 4.),window.add_sphere(1.));
    
    let mut testSpring = spring::Spring::new(10.,test1, test2);
    
    
    //window.set_framerate_limit(Some(1));

    while window.render(){
        testSpring.moveNodes();
        
        
    }


}