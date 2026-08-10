use crate::vehicle::Vehicle;
use crate::stats::Stats;
const SAFE_DISTANCE: f32 = 20.0;
const CRITICAL_DISTANCE: f32 = 5.0;
const SAFE_TIME_GAP: f32 = 1.0;
 const CAR_LENGTH : f32 = 4.0 ;
pub struct Intersection {
    pub vehicles: Vec<Vehicle>,
    pub stats: Stats,
}

impl Intersection {
    pub fn new() -> Self {
        Intersection {
            vehicles: Vec::new(),
            stats: Stats::new(),
        }
    }

    pub fn add_vehicle(&mut self, car: Vehicle) {

        let spawn_blocked = self.vehicles.iter().any(|v|{
            v.direction == car.direction && (v.total_distance - v.distance) < SAFE_DISTANCE 
        });
        if !spawn_blocked {
                    self.vehicles.push(car);  
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let num_vehicles = self.vehicles.len();
        for i in 0..num_vehicles {
            let mut safe_speed = self.vehicles[i].base_velocity;

            self.stats.update_speeds(safe_speed);
            for j in 0..num_vehicles {
                if i == j {
                    continue;
                }

                let car_i: &Vehicle = &self.vehicles[i];
                let car_j: &Vehicle = &self.vehicles[j];

                if car_i.direction == car_j.direction && car_i.lane == car_j.lane {
                    if car_j.distance < car_i.distance {
                        let distance_between = car_i.distance - car_j.distance;

                        if distance_between < CRITICAL_DISTANCE {
                            safe_speed = 0.0;
                            self.stats.record_near_miss();
                        } else if distance_between < SAFE_DISTANCE {

                            safe_speed = car_j.velocity;
                        }
                    }
                } else {
                    if car_i.base_velocity > 0.0 && car_j.base_velocity > 0.0 {

                        let dist_to_center_i = car_i.distance - (car_i.total_distance / 2.0);
                        let dist_to_center_j = car_j.distance - (car_j.total_distance / 2.0);

                           if dist_to_center_i > -(CAR_LENGTH/2.0) && dist_to_center_j > -(CAR_LENGTH/2.0) {
                            if dist_to_center_i < CRITICAL_DISTANCE && dist_to_center_j < CRITICAL_DISTANCE {
                              if dist_to_center_i > dist_to_center_j || (dist_to_center_i == dist_to_center_j && car_i.id > car_j.id) {
                                    safe_speed = 0.0;
                                    self.stats.record_near_miss();
                                }
                            }
                           }else if dist_to_center_i > 0.0 && dist_to_center_j > 0.0 {

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
        self.vehicles.retain_mut(|car| {
         if car.distance <= -CAR_LENGTH {
            self.stats.record_vehicle_exit(car.time_spent);
            false
         }else {
            true
         }
        });
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::vehicle::Direction;
    #[test]
    fn test_safe_distance_rear_end() {
        let mut crossroad = Intersection::new();

        let car1 = Vehicle::new(1, 50.0, 5.0, 's', Direction::North);

        let car2 = Vehicle::new(2, 60.0, 3.0, 's', Direction::North);

        crossroad.add_vehicle(car1);
        crossroad.add_vehicle(car2);

        crossroad.update(0.1);

        assert_eq!(crossroad.vehicles[1].velocity, 10.0);
    }
    #[test]
    fn test_conflict_intersection() {
        let mut crossroad = Intersection::new();

        let car1 = Vehicle::new(1, 100.0, 10.0, 's', Direction::North);
        let car2 = Vehicle::new(2, 110.0, 11.0, 's', Direction::East);

        crossroad.add_vehicle(car1);
        crossroad.add_vehicle(car2);

        crossroad.update(0.1);

        assert_eq!(crossroad.vehicles[1].velocity, 7.0);
    }
}
