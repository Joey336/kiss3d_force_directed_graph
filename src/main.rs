extern crate kiss3d;
extern crate nalgebra as na;
mod graph_maker;
mod node;
mod spring;
mod common_funcs;
mod dot_reader;


fn main() {


    //spring mult 150.
    //graph_maker::create_graph("dot_files/annthyroid/depth_3.dot");
    //graph_maker::create_graph("dot_files/annthyroid/depth_6.dot");
    //graph_maker::create_graph("dot_files/annthyroid/depth_10.dot");
    
    //spring mult 50. or 150. ?
    //graph_maker::create_graph("dot_files/glass/depth_8.dot");
    graph_maker::create_graph("dot_files/glass/depth_12.dot");
    //graph_maker::create_graph("dot_files/glass/depth_18.dot");

    //spring mult 3.
    //graph_maker::create_graph("dot_files/breastw/depth_3.dot");
    //graph_maker::create_graph("dot_files/breastw/depth_5.dot");
    //graph_maker::create_graph("dot_files/breastw/depth_10.dot");
}