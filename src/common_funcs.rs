pub fn getMagnitude(vector: na::Vector3<f32>) -> f32{
    (f32::powf(vector.x, 2.) + f32::powf(vector.y, 2.) + f32::powf(vector.z, 2.)).sqrt()
}

pub fn setMagnitude(mut vector: na::Vector3<f32>,newMag:f32) -> na::Vector3<f32>{
    let oldMag = getMagnitude(vector);
    let ratio: f32 = newMag / oldMag;
    vector*=ratio;
    return vector;
}