extern crate kiss3d;
extern crate nalgebra as na;



mod graph_maker;
mod node;
mod spring;
mod common_funcs;
fn main() {
    graph_maker::create_graph();
}