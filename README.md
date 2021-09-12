# kiss3d_force_directed_graph

## About The Project

This project was built using the [Kiss3d](https://github.com/sebcrozet/kiss3d) graphics library for Rust.

This program takes in graph data through .dot files and uses a force-directed drawing algorithm to simulate physics between graph components.

![Alt Text](https://media1.giphy.com/media/OGpG9ceEJ1PVFAvTv2/giphy.gif?cid=790b7611551df85a6fbad1eb6b8372a5b2f98775e214cde3&rid=giphy.gif&ct=g)


## Try it for yourself

1) First, Rust must be installed on your machine.
2) clone repo: ```git clone https://github.com/Joey336/kiss3d_force_directed_graph.git```
3) enter ```cargo run``` in the terminal
4) You can modify the create_graph method in main to include the path to any file in the dot_files folder to visualize different graphs

# User Control
left-click drag will rotate the canvas
right-click drag will translate the canvas
scroll wheel will zoom in/out
