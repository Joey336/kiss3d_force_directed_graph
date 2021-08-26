use crate::node;
use crate::common_funcs;
use std::collections::HashMap;

#[derive(Debug,)]
pub struct Spring{
    nat_len: f32,
    k: f32,
    node1: String, //String's reference hash table
    node2: String,
}


impl Spring{
    pub fn new(nat_len: f32, hash_code1: String, hash_code2: String) -> Self {
        Spring{
            nat_len: nat_len,
            k: 0.005,
            node1: hash_code1,
            node2: hash_code2,
        }
    }

    //apply acceleration to both nodes at each end of spring
    pub fn move_nodes(&self, nodes:&mut HashMap<String, node::Node>){
        //borrow ownership of nodes spring is connected to
        let node1 = nodes.get(&self.node1).unwrap();
        let node2 = nodes.get(&self.node2).unwrap();


        let force = node2.get_position() - node1.get_position();
        let force_magnitude = common_funcs::get_magnitude(force);

        let new_magnitude = self.k * (force_magnitude - self.nat_len);

        let mut new_force = common_funcs::set_magnitude(force, new_magnitude);

        //drop ownership to get_mut from hashmap
        std::mem::drop(node1);
        std::mem::drop(node2);

        let node1 = nodes.get_mut(&self.node1).unwrap();
         node1.accelerate(new_force);
         std::mem::drop(node1);

        //reverse direction of force for node on opposite side
        new_force *= -1.;
       
        let node2 = nodes.get_mut(&self.node2).unwrap();
         node2.accelerate(new_force);
         std::mem::drop(node2);   
    }

}
