use crate::node;
use crate::common_funcs;

pub struct Spring{
    nat_len: f32,
    k: f32,
    node1: node::Node,
    node2: node::Node,
}



impl Spring{
    pub fn new(nat_len: f32, node1: node::Node, node2: node::Node) -> Self {
        Spring{
            nat_len: nat_len,
            k: 0.005,
            node1: node1,
            node2: node2,
        }
    }

    pub fn moveNodes(&mut self){
        
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
      
        self.node1.updatePosition();
        self.node2.updatePosition();
    }
}
