use primitive::{Cone, Cylinder, Sphere};
use vector::Vector;
use vertex::Vertex;

#[expect(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
pub fn sphere(
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    sphere: &Sphere,
    effects: u32,
    palette: u32,
    latitude: u32,
    longitude: u32,
) {
    let base = vertices.len() as u32;
    let pi = std::f32::consts::PI;

    for i in 0..=latitude {
        let theta = pi * (i as f32) / (latitude as f32);
        let (ts, tc) = (theta.sin(), theta.cos());

        for j in 0..=longitude {
            let phi = 2.0 * pi * (j as f32) / (longitude as f32);
            let (ps, pc) = (phi.sin(), phi.cos());

            let normal = Vector::new(ts * pc, tc, ts * ps);

            let position = Vector::new(
                sphere.center.x + sphere.radius * normal.x,
                sphere.center.y + sphere.radius * normal.y,
                sphere.center.z + sphere.radius * normal.z,
            );

            vertices.push(Vertex::new(
                position,
                normal,
                sphere.color,
                effects,
                palette,
            ));
        }
    }

    for i in 0..latitude {
        for j in 0..longitude {
            let row = longitude + 1;
            let a = base + i * row + j;
            let b = base + i * row + j + 1;
            let c = base + (i + 1) * row + j;
            let d = base + (i + 1) * row + j + 1;

            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }
}

#[expect(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
pub fn cylinder(
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    cylinder: &Cylinder,
    effects: u32,
    palette: u32,
    segments: u32,
) {
    let axis = cylinder.target - cylinder.source;
    let length = axis.magnitude();

    if length < 0.001 {
        return;
    }

    let base = vertices.len() as u32;
    let forward = axis.normalize();

    let arbitrary = if forward.y.abs() < 0.9 {
        Vector::new(0.0, 1.0, 0.0)
    } else {
        Vector::new(1.0, 0.0, 0.0)
    };

    let right = forward.cross(&arbitrary).normalize();
    let up = right.cross(&forward);

    let pi = std::f32::consts::PI;

    for i in 0..=segments {
        let angle = 2.0 * pi * (i as f32) / (segments as f32);
        let (cosine, sine) = (angle.cos(), angle.sin());

        let normal = right * cosine + up * sine;

        let source = cylinder.source + normal * cylinder.radius;
        let target = cylinder.target + normal * cylinder.radius;

        vertices.push(Vertex::new(
            source,
            normal,
            cylinder.color,
            effects,
            palette,
        ));
        vertices.push(Vertex::new(
            target,
            normal,
            cylinder.color,
            effects,
            palette,
        ));
    }

    for i in 0..segments {
        let a = base + i * 2;
        let b = base + i * 2 + 1;
        let c = base + (i + 1) * 2;
        let d = base + (i + 1) * 2 + 1;

        indices.extend_from_slice(&[a, c, b, b, c, d]);
    }

    let center = vertices.len() as u32;
    let normal = forward * -1.0;
    vertices.push(Vertex::new(
        cylinder.source,
        normal,
        cylinder.color,
        effects,
        palette,
    ));

    for i in 0..=segments {
        let angle = 2.0 * pi * (i as f32) / (segments as f32);
        let (cosine, sine) = (angle.cos(), angle.sin());

        let offset = right * cosine + up * sine;
        let position = cylinder.source + offset * cylinder.radius;

        vertices.push(Vertex::new(
            position,
            normal,
            cylinder.color,
            effects,
            palette,
        ));
    }

    for i in 0..segments {
        indices.extend_from_slice(&[center, center + 1 + (i + 1), center + 1 + i]);
    }

    let center = vertices.len() as u32;
    let normal = forward;
    vertices.push(Vertex::new(
        cylinder.target,
        normal,
        cylinder.color,
        effects,
        palette,
    ));

    for i in 0..=segments {
        let angle = 2.0 * pi * (i as f32) / (segments as f32);
        let (cosine, sine) = (angle.cos(), angle.sin());

        let offset = right * cosine + up * sine;
        let position = cylinder.target + offset * cylinder.radius;

        vertices.push(Vertex::new(
            position,
            normal,
            cylinder.color,
            effects,
            palette,
        ));
    }

    for i in 0..segments {
        indices.extend_from_slice(&[center, center + 1 + i, center + 1 + (i + 1)]);
    }
}

#[expect(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
pub fn cone(
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    cone: &Cone,
    effects: u32,
    palette: u32,
    segments: u32,
) {
    let axis = cone.apex - cone.base;
    let height = axis.magnitude();

    if height < 0.001 {
        return;
    }

    let base = vertices.len() as u32;
    let forward = axis.normalize();

    let arbitrary = if forward.y.abs() < 0.9 {
        Vector::new(0.0, 1.0, 0.0)
    } else {
        Vector::new(1.0, 0.0, 0.0)
    };

    let right = forward.cross(&arbitrary).normalize();
    let up = right.cross(&forward);

    let pi = std::f32::consts::PI;

    let ratio = cone.radius / height;
    let hypotenuse = (1.0 + ratio * ratio).sqrt();
    let lateral = 1.0 / hypotenuse;
    let axial = ratio * lateral;

    for i in 0..=segments {
        let angle = 2.0 * pi * (i as f32) / (segments as f32);
        let (cosine, sine) = (angle.cos(), angle.sin());

        let radial = right * cosine + up * sine;
        let position = cone.base + radial * cone.radius;

        let normal = (radial * lateral + forward * axial).normalize();

        vertices.push(Vertex::new(position, normal, cone.color, effects, palette));
    }

    let apex = vertices.len() as u32;
    vertices.push(Vertex::new(
        cone.apex, forward, cone.color, effects, palette,
    ));

    for i in 0..segments {
        let a = base + i;
        let b = base + i + 1;
        indices.extend_from_slice(&[a, b, apex]);
    }

    let center = vertices.len() as u32;
    let normal = forward * -1.0;
    vertices.push(Vertex::new(cone.base, normal, cone.color, effects, palette));

    for i in 0..=segments {
        let angle = 2.0 * pi * (i as f32) / (segments as f32);
        let (cosine, sine) = (angle.cos(), angle.sin());

        let offset = right * cosine + up * sine;
        let position = cone.base + offset * cone.radius;

        vertices.push(Vertex::new(position, normal, cone.color, effects, palette));
    }

    for i in 0..segments {
        indices.extend_from_slice(&[center, center + 1 + (i + 1), center + 1 + i]);
    }
}
