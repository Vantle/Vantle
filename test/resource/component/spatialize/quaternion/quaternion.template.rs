use quaternion::Quaternion;
use vector::Vector;

fn identity() -> [f32; 4] {
    let q = Quaternion::identity();
    [q.w, q.x, q.y, q.z]
}

fn conjugate(input: [f32; 4]) -> [f32; 4] {
    let q = Quaternion {
        w: input[0],
        x: input[1],
        y: input[2],
        z: input[3],
    };
    let c = q.conjugate();
    [c.w, c.x, c.y, c.z]
}

fn magnitude(input: [f32; 4]) -> f32 {
    let q = Quaternion {
        w: input[0],
        x: input[1],
        y: input[2],
        z: input[3],
    };
    q.magnitude()
}

fn rotate(quaternion: [f32; 4], point: [f32; 3]) -> [f32; 3] {
    let q = Quaternion {
        w: quaternion[0],
        x: quaternion[1],
        y: quaternion[2],
        z: quaternion[3],
    };
    q.rotate(Vector::from(point)).array()
}
