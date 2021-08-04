use crate::node;
use crate::main;
use crate::spring;
use kiss3d::light::Light;
use kiss3d::text::Font;
use kiss3d::window::Window;
extern crate rand;
use rand::random;
use rand::prelude::*;
use std::collections::HashMap;


pub fn create_graph(){
    let mut window = Window::new("Kiss3d: force_directed_graph");
    window.set_background_color(0.5, 0.5, 0.5);
    window.set_light(Light::StickToCamera);

 

    let mut nodes = create_nodes(100, &mut window);
    let mut springs = create_springs(&nodes);
   
   
/*
    let mut test1 = node::Node::new(na::Vector3::new(50., 0., 0.),window.add_sphere(1.));
    
    let mut test2 = node::Node::new(na::Vector3::new(0., 50., 0.),window.add_sphere(1.));
    let mut test3 = node::Node::new(na::Vector3::new(0., 0.,50.),window.add_sphere(1.));

    nodes.insert(1,test1);
    nodes.insert(2,test2);
    nodes.insert(3,test3);
    

    
    let mut testSpring12 = spring::Spring::new(10.,1, 2);
    let mut testSpring23 = spring::Spring::new(10.,2, 3);
    let mut testSpring13 = spring::Spring::new(10.,1, 3);
    */
    
    //window.set_framerate_limit(Some(1));

    while window.render(){
        
        for i in springs.iter(){
            i.moveNodes(&mut nodes);
        }

        for (key, value) in &mut nodes{
            value.updatePosition();
        }
      
       // springs[0].moveNodes(&mut nodes);
        //let moveNodes borrow ownership of the nodes hashMap
        //println!("{}",nodes.get(&1).unwrap().getAcceleration());

        /*
        testSpring12.moveNodes(&mut nodes);
        testSpring23.moveNodes(&mut nodes);
        testSpring13.moveNodes(&mut nodes);
        for (key, value) in &mut nodes{
            value.updatePosition();
        }
        */
    }


}

fn create_springs(hashmap: &HashMap<i32, node::Node>) -> Vec<spring::Spring>{
    let mut return_vec: Vec<spring::Spring> = Vec::new();

    for i in 0..hashmap.len(){
        for j in i + 1..hashmap.len(){
            let new_spring = spring::Spring::new(50.,i as i32, j as i32);
            return_vec.push(new_spring);
        }
    }

    return_vec
}

fn create_nodes(numNodes: i32, window: &mut Window) -> HashMap<i32, node::Node>{
    //initialize x, y, z at random positions
    let mut return_hashmap = HashMap::new();    

    for i in 0..numNodes{
        let mut rng = thread_rng();
        let x = rng.gen_range(0., 100.0);
        let y = rng.gen_range(0., 100.0);
        let z = rng.gen_range(0., 100.0);

        let new_node = node::Node::new(na::Vector3::new(x, y, z),window.add_sphere(1.));
        return_hashmap.insert(i, new_node);
    }

    return_hashmap
}