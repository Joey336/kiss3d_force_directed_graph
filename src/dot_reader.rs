use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
pub struct Color{
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

// the data for each node
#[derive(Clone, Debug)]
pub struct NodeData {
    pub name: String,     // name of the cluster
    pub cardinality: i32, // the cluster's cardinality
    pub radius: f32,  // the radius of the cluster
    lfd: f32,         // the local fractal dimension of the cluster
    pub color: Color,       // the color of the cluster
    degree: i32,      // the degree of the node, or amount of connected edges
}

// reading the dot file and returning a drawable force graph. Takes in the path
// of the dot file and the drawing scale of the graph
pub fn read_dot(
    dot_file: &str
) -> (HashMap<String, NodeData>, HashMap<String, (String, String, f32)>) {
    let file = File::open(dot_file).unwrap();
    let reader = BufReader::new(file);

    let mut uploaded_nodes: HashMap<String, NodeData> = HashMap::new();
    let mut uploaded_edges: HashMap<String, (String, String, f32)> = HashMap::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if is_node(&line) {
            // reading the nodes from the dot file
            let new_node_data = get_node_data(line);
            let _node_data = new_node_data.clone();
            uploaded_nodes.insert(_node_data.clone().name.to_string(), _node_data);
        } else if is_edge(&line) {
            // reading the edges from the dot file
            let (node_1, node_2, label_num) = get_edge_data(line);

            // incrementing the degree of the nodes each edge connects
            uploaded_nodes.get_mut(&node_1).unwrap().degree += 1;
            uploaded_nodes.get_mut(&node_2).unwrap().degree += 1;

            let edge_index = format!("{:?} -- {:?}", &node_1.to_string(), &node_2.to_string());
            uploaded_edges.insert(edge_index, (node_1, node_2, label_num));
        }
    }

    (uploaded_nodes, uploaded_edges)
}

// reading a line of the dot file and returning true if the line is a node, and false if not
fn is_node(file_line: &String) -> bool {
    !file_line.contains("{")
        && !file_line.contains("}")
        && !file_line.contains(" -")
        && !file_line.contains("edge")
}

// parsing the given line to retrieve its node data, and returning it to
// be attached to the appropriate graph node
fn get_node_data(node_line: String) -> NodeData {
    let _node_string_1: Vec<&str> = node_line.split('[').collect();

    let _node_string_2: Vec<&str> = _node_string_1[1].split('\\').collect();
    let _node_string_3: Vec<&str> = _node_string_2[3].split("\", ").collect();

    let _node_name: String = _node_string_1[0].replace(" ", "");
    let _node_cardinality: String = _node_string_2[1].replace("ncardinality ", "");
    let _node_radius: String = _node_string_2[2].replace("nradius ", "");
    let _node_lfd: String = _node_string_3[0].replace("nlfd ", "");
    let _node_color_1: String = _node_string_3[1].replace("color=\"", "");
    let _node_color = get_color_from_hex(_node_color_1.clone());
    let new_node_data = NodeData {
        name: _node_name,
        cardinality: _node_cardinality.parse::<i32>().unwrap(),
        radius: _node_radius.parse::<f32>().unwrap(),
        lfd: _node_lfd.parse::<f32>().unwrap(),
        color: _node_color,
        degree: 0,
    };
    return new_node_data;
}

// reading a line of the dot file and returning true if the line is an edge, and false if not
fn is_edge(file_line: &String) -> bool {
    !file_line.contains("{") && !file_line.contains("}") && file_line.contains(" -")
}

// parsing the given line, which defines either a directed
// or undirected edge, and returning the names of the two
// nodes the edge connects
fn get_edge_data(edge_line: String) -> (String, String, f32) {
    if edge_line.contains(" -> ") {
        let _edge_string_1: Vec<&str> = edge_line.split(" -> ").collect();
        let _node_1: String = _edge_string_1[0].split_whitespace().collect();
        let _node_2: String = _edge_string_1[1].to_string();

        let mut edge_label: String =  _edge_string_1[1].split("label=\"").collect();
        edge_label.remove(edge_label.len() - 1);
        edge_label.remove(edge_label.len() - 1);
        let label_num = convert_scientific_to_float(edge_label);

        return (_node_1, _node_2, label_num);
    } else {
        let _edge_string_1: Vec<&str> = edge_line.split(" -- ").collect();
        let _edge_string_2: Vec<&str> = _edge_string_1[1].split(" [").collect();


        let mut edge_label: String =  _edge_string_2[1].split("label=\"").collect();
        edge_label.remove(edge_label.len() - 1);
        edge_label.remove(edge_label.len() - 1);
        let label_num = convert_scientific_to_float(edge_label);
        
        let _node_1: String = _edge_string_1[0].split_whitespace().collect();
        let _node_2: String = _edge_string_2[0].to_string();
        
        return (_node_1, _node_2, label_num);
    }
}

// getting the color in RGB form from the hexadecimal color taken from the dot file
fn get_color_from_hex(hex_str: String) -> Color{
    let r_hex = &hex_str[1..3];
    let b_hex = &hex_str[5..7];

    let r = i32::from_str_radix(r_hex, 16).unwrap();
    let b = i32::from_str_radix(b_hex, 16).unwrap();
   
    return Color{red: r as f32, green: 0.0, blue: b as f32};
    
}


//parses scientific notation string and returns f32 version
fn convert_scientific_to_float(sci_not: String) -> f32{
    let test:  Vec<&str> = sci_not.split("e").collect();

    let base = test[0].parse::<f32>().unwrap();
    let exp = test[1].parse::<f32>().unwrap();

    
    base * f32::powf(10., exp)
}

