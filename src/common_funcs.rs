pub fn get_magnitude(vector: na::Vector3<f32>) -> f32{
    (f32::powf(vector.x, 2.) + f32::powf(vector.y, 2.) + f32::powf(vector.z, 2.)).sqrt()
}

pub fn set_magnitude(mut vector: na::Vector3<f32>,new_mag:f32) -> na::Vector3<f32>{
    let old_mag = get_magnitude(vector);
    let ratio: f32 = new_mag / old_mag;
    vector*=ratio;
    return vector;
}