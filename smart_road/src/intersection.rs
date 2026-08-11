use crate::vehicle::{Vehicle,Direction};
use crate::stats::Stats;
const SAFE_DISTANCE: f32 = 120.0;
const CRITICAL_DISTANCE: f32 = 60.0;
const SAFE_TIME_GAP: f32 = 1.0;
 const CAR_LENGTH : f32 = 40.0 ;


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
            v.direction == car.direction && v.lane == car.lane && (v.total_distance - v.distance) <=SAFE_DISTANCE 
        });
        if !spawn_blocked {
                    self.vehicles.push(car);  
        }
    }
pub fn update(&mut self, delta_time: f32) {
        let num_vehicles = self.vehicles.len();
        
        for i in 0..num_vehicles {
            let mut target_speed = 100.0; 

            for j in 0..num_vehicles {
                if i == j { continue; }

                let car_i = &self.vehicles[i];
                let car_j = &self.vehicles[j];

                if car_i.direction == car_j.direction && car_i.lane == car_j.lane {
                    if car_j.distance < car_i.distance { 
                        let gap = car_i.distance - car_j.distance;
                        
                        if gap < CRITICAL_DISTANCE {
                            target_speed = 0.0;
                        } else if gap < SAFE_DISTANCE {
                           
                            if target_speed > 50.0 { target_speed = 50.0; }
                            if target_speed > car_j.velocity { target_speed = car_j.velocity; }
                        }
                    }
                } 
                else {
                
                 let dist_to_center_i = match car_i.direction {
                        Direction::North | Direction::West => car_i.distance - 400.0 - 40.0,
                        _ => car_i.distance - 400.0,
                    };
                    
                    let dist_to_center_j = match car_j.direction {
                        Direction::North | Direction::West => car_j.distance - 400.0 - 40.0,
                        _ => car_j.distance - 400.0,
                    };
                    if car_i.distance > (300.0 - CAR_LENGTH -10.0) && car_j.distance > (300.0 - CAR_LENGTH -10.0) {
                        
                        if (dist_to_center_i - dist_to_center_j).abs() < 60.0 {
                            
                            if dist_to_center_i > dist_to_center_j || (dist_to_center_i == dist_to_center_j && car_i.id > car_j.id) {
                                
                                if dist_to_center_i < 110.0 {
                                    target_speed = 0.0;
                                    self.stats.record_near_miss();
                                } 
                                else if target_speed > 50.0 {
                                    target_speed = 50.0;
                                }
                            }
                        }
                    }
                }
            }

            self.vehicles[i].velocity = target_speed;
        }

        for car in &mut self.vehicles {
          
            car.update(delta_time);
        }
        
        self.vehicles.retain_mut(|car| {
            if car.distance <= -50.0 {
                self.stats.record_vehicle_exit(car.time_spent);
                false
            } else {
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

