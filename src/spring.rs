use crate::node;
use crate::common_funcs;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct Spring{
    nat_len: f32,
    k: f32,
    node1: i32, //i32's reference hash table
    node2: i32,
}



impl Spring{
    pub fn new(nat_len: f32, hash_code1: i32, hash_code2: i32) -> Self {
        Spring{
            nat_len: nat_len,
            k: 0.005,
            node1: hash_code1,
            node2: hash_code2,
        }
    }
    pub fn moveNodes(self, nodes:&mut HashMap<i32, node::Node>){
        //borrow ownership of nodes spring is connected to
        let node1 = nodes.get(&self.node1).unwrap();
        let node2 = nodes.get(&self.node2).unwrap();


        let force = node2.getPosition() - node1.getPosition();
        let force_magnitude = common_funcs::getMagnitude(force);

         let new_magnitude = self.k * (force_magnitude - self.nat_len);

         let mut new_force = common_funcs::setMagnitude(force, new_magnitude);
        std::mem::drop(node1);
        std::mem::drop(node2);
        let node1 = nodes.get_mut(&self.node1).unwrap();
         //println!("{}",new_force);
        
         node1.accelerate(new_force);
         std::mem::drop(node1);
         new_force *= -1.;
         //println!("{}",new_force);
        let node2 = nodes.get_mut(&self.node2).unwrap();
         node2.accelerate(new_force);
         std::mem::drop(node2);
       

    }
/*
    pub fn moveNodes(&mut self){
        let node1 = 
        let force = self.node2.getPosition() - self.node1.getPosition();
        let force_magnitude = common_funcs::getMagnitude(force);

        let new_magnitude = self.k * (force_magnitude - self.nat_len);
        

  
        //println!("force: {} getMAG: {}    magnitude: {}",force, getMagnitude(force), magnitude );
        let mut new_force = common_funcs::setMagnitude(force, new_magnitude);
        
        //println!("{}",new_force);
        self.node1.accelerate(new_force);
        new_force *= -1.;
        //println!("{}",new_force);
        self.node2.accelerate(new_force);
      
    }
    */
}
