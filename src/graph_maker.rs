use crate::node;
use crate::spring;
use crate::dot_reader;
use kiss3d::light::Light;
use kiss3d::window::Window;
extern crate rand;
use rand::prelude::*;
use std::collections::HashMap;
use kiss3d::nalgebra::{Point3};


pub fn create_graph(dot_file: &str){
    let mut window = Window::new("Kiss3d: force_directed_graph");
    window.set_background_color(0.5, 0.5, 0.5);
    window.set_light(Light::StickToCamera);

    let (nodes_data, edges_data) = dot_reader::read_dot(dot_file);

    let mut nodes_hashmap = create_nodes(nodes_data, &mut window);
    let springs_vector = create_springs(&edges_data);


    
    window.set_framerate_limit(Some(10));

    while window.render(){
        
        //iterate through all springs to apply acceleration of nodes
        for spring in springs_vector.iter(){
            spring.move_nodes(&mut nodes_hashmap);
        }

        //update position on canvas of all nodes (after all spring forces applied, each node will have its final acceleration for this frame)
        for (_key, value) in &mut nodes_hashmap{
            value.update_position();
        }

        //draw visible edges between nodes after updated position on canvas
        draw_edges(&nodes_hashmap, &edges_data, &mut window)
      
    }


}


//draws edges between appropriate nodes
fn draw_edges(nodes_hashmap: &HashMap<String, node::Node>, edges_data: &HashMap<String, (String, String, f32)>, window: &mut Window){
    for (_key, data) in edges_data{
        let node1 = data.0.clone();
        let node2 = data.1.clone();

        let node_ob1 = nodes_hashmap.get(&node1).unwrap();
        let node_ob2 = nodes_hashmap.get(&node2).unwrap();

        let point1 = node::Node::get_point(&node_ob1);
        let point2 = node::Node::get_point(&node_ob2);

        window.draw_line(&point1, &point2, &Point3::new(0., 1.0,1.0));
        window.set_line_width(1.);
    }
}


//creates spring for each edge in graph
fn create_springs(edges_data: &HashMap<String, (String, String, f32)>) -> Vec<spring::Spring>{
    let spring_multiplier = 50.;

    let mut return_vec: Vec<spring::Spring> = Vec::new();

    for (_key, data) in edges_data{
        //resting length scaled by spring_multiplier
        let new_spring = spring::Spring::new(data.2 * spring_multiplier, data.0.clone(), data.1.clone());
        return_vec.push(new_spring);
    }

    return_vec
}

//given node_data from dot file, returns hashmap of Node objects
fn create_nodes(node_data: HashMap<String, dot_reader::NodeData>, window: &mut Window) -> HashMap<String, node::Node>{
    //initialize x, y, z at random positions
    let mut return_hashmap = HashMap::new();    
    
 
    for (_key, value) in &node_data{
        //randomly generate initial x,y,z position of node
        let mut rng = thread_rng();
        let x = rng.gen_range(0., 100.0);
        let y = rng.gen_range(0., 100.0);
        let z = rng.gen_range(0., 100.0);

        let mut sphere = window.add_sphere(1.);

        //RBG scaled to float between 0.0 - 1.0 (kiss3d RGB parameters are 0.0 - 1.0)
        sphere.set_color((1./255.)*value.color.red, (1./255.)*value.color.green, (1./255.)*value.color.blue);

        let new_node = node::Node::new(na::Vector3::new(x, y, z),value.cardinality as f32, sphere);
        return_hashmap.insert(value.name.clone(), new_node);
       
    }

    return_hashmap
}