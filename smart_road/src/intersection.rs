use crate::vehicle::Vehicle;

const SAFE_DISTANCE: f32 = 20.0;
const SAFE_TIME_GAP: f32 = 1.0;
pub struct Intersection {
    pub vehicles: Vec<Vehicle>,
}

impl Intersection {
    pub fn new() -> Self {
        Intersection {
            vehicles: Vec::new(),
        }
    }
    pub fn add_vehicle(&mut self, car: Vehicle) {
        self.vehicles.push(car);
    }

    pub fn update(&mut self, delta_time: f32) {
        let num_vehicles = self.vehicles.len();
        for i in 0..num_vehicles {
            let mut danger = false;
            let mut safe_speed = self.vehicles[i].velocity;

            for j in 0..num_vehicles {
                if i == j {
                    continue;
                }

                let car_i: &Vehicle = &self.vehicles[i];
                let car_j: &Vehicle = &self.vehicles[j];

                if car_i.direction == car_j.direction && car_i.lane == car_j.lane {
                    if car_j.distance < car_i.distance {
                        let distance_between = car_i.distance - car_j.distance;
                        if distance_between < SAFE_DISTANCE {
                            danger = true;
                            safe_speed = car_j.velocity;
                        }
                    }
                } else {
                    if car_i.velocity > 0.0 && car_j.velocity > 0.0 {
                        let dist_to_center_i = car_i.distance - (car_i.total_distance / 2.0);
                        let dist_to_center_j = car_j.distance - (car_j.total_distance / 2.0);
                        if dist_to_center_i > 0.0 && dist_to_center_j > 0.0 {
                            
                            let time_to_center_i = dist_to_center_i / car_i.velocity;
                            let time_to_center_j = dist_to_center_j / car_j.velocity;

                            let time_difference = (time_to_center_i - time_to_center_j).abs();

                            if time_difference < SAFE_TIME_GAP {
                                if time_to_center_i > time_to_center_j {
                                    safe_speed = car_i.velocity * 0.7;
                                }
                            }
                        }
                    }
                }
            }

            self.vehicles[i].velocity = safe_speed;
        }

        for car in &mut self.vehicles {
            car.update(delta_time);
        }
    }
}
