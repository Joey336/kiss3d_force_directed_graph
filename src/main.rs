extern crate kiss3d;
extern crate nalgebra as na;
mod graph_maker;
mod node;
mod spring;
mod common_funcs;
mod dot_reader;


fn main() {

    //spring_mult 0.1
    //graph_maker::create_graph("dot_files/arrhythmia/graph_neighborhood_optimal_4.dot");
    //graph_maker::create_graph("dot_files/arrhythmia/cluster_cardinality_optimal_0.dot");
    //graph_maker::create_graph("dot_files/arrhythmia/component_cardinality_optimal_3.dot");
    //graph_maker::create_graph("dot_files/arrhythmia/stationary_probabilities_optimal_8.dot");


    //spring mult 5.
    //graph_maker::create_graph("dot_files/ionosphere/parent_cardinality_optimal_7.dot");
    //graph_maker::create_graph("dot_files/ionosphere/stationary_probabilities_optimal_8.dot");

    //graph_maker::create_graph("dot_files/lympho/cluster_cardinality_optimal_0.dot");
    graph_maker::create_graph("dot_files/lympho/component_cardinality_optimal_2.dot");
    

    //spring_mul 3.
    //graph_maker::create_graph("dot_files/glass/graph_neighborhood_optimal_4.dot");
    //graph_maker::create_graph("dot_files/glass/parent_cardinality_optimal_16.dot");
    //graph_maker::create_graph("dot_files/glass/stationary_probabilities_optimal_9.dot");
    //graph_maker::create_graph("dot_files/glass/component_cardinality_optimal_22.dot");

    
    
   
}