use effect::{Effect, Effected};
use primitive::{Arrow, Cone, Cylinder, Sphere};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Field {
    Sphere(Sphere),
    Cylinder(Cylinder),
    Cone(Cone),
    Union {
        left: Box<Field>,
        right: Box<Field>,
    },
    Intersect {
        left: Box<Field>,
        right: Box<Field>,
    },
    Subtract {
        left: Box<Field>,
        right: Box<Field>,
    },
    Effected {
        base: Box<Field>,
        effects: Vec<Effect>,
    },
}

impl From<Sphere> for Field {
    fn from(sphere: Sphere) -> Self {
        Field::Sphere(sphere)
    }
}

impl From<Cylinder> for Field {
    fn from(cylinder: Cylinder) -> Self {
        Field::Cylinder(cylinder)
    }
}

impl From<Cone> for Field {
    fn from(cone: Cone) -> Self {
        Field::Cone(cone)
    }
}

impl From<Arrow> for Field {
    fn from(arrow: Arrow) -> Self {
        let axis = arrow.target - arrow.source;
        let length = axis.magnitude();

        if length < 0.001 {
            return Field::Sphere(Sphere::new(arrow.source, 0.0, arrow.color));
        }

        let direction = axis.normalize();
        let head = length.min(arrow.radius * 4.0);
        let junction = arrow.target - direction * head;

        let shaft = Cylinder::new(arrow.source, junction, arrow.radius, arrow.color);
        let tip = Cone::new(junction, arrow.target, arrow.radius * 2.0, arrow.color);

        Field::Union {
            left: Box::new(Field::Cylinder(shaft)),
            right: Box::new(Field::Cone(tip)),
        }
    }
}

impl<T: Into<Field>> From<Effected<T>> for Field {
    fn from(effected: Effected<T>) -> Self {
        Field::Effected {
            base: Box::new(effected.inner.into()),
            effects: effected.effects,
        }
    }
}
