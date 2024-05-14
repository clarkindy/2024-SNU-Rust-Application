#![allow(dead_code)]
pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: usize,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        Self {
            name,
            age,
            height,
            visit_count: 0,
            last_blood_pressure: None,
        }
    }

    pub fn visit_doctor<'a>(&'a mut self, measurements: Measurements) -> HealthReport<'a> {
        self.visit_count += 1;
        let to_i32 = |x: u32| i32::try_from(x).unwrap();
        let ret = HealthReport {
            patient_name: &self.name,
            visit_count: u32::try_from(self.visit_count).unwrap_or(u32::MAX),
            height_change: measurements.height - self.height,
            blood_pressure_change: self.last_blood_pressure.map(|x| {
                (
                    to_i32(measurements.blood_pressure.0) - to_i32(x.0),
                    to_i32(measurements.blood_pressure.1) - to_i32(x.1),
                )
            }),
        };
        self.height = measurements.height;
        self.last_blood_pressure = Some(measurements.blood_pressure);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visit() {
        let mut bob = User::new(String::from("Bob"), 32, 155.2);
        assert_eq!(bob.visit_count, 0);

        let report = bob.visit_doctor(Measurements {
            height: 156.1,
            blood_pressure: (120, 80),
        });
        assert_eq!(report.patient_name, "Bob");
        assert_eq!(report.visit_count, 1);
        assert_eq!(report.blood_pressure_change, None);

        let report = bob.visit_doctor(Measurements {
            height: 156.1,
            blood_pressure: (115, 76),
        });
        assert_eq!(report.visit_count, 2);
        assert_eq!(report.blood_pressure_change, Some((-5, -4)));
    }
}
