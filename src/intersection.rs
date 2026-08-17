use crate::stats::Stats;
use crate::vehicle::{Direction, Vehicle};
const SAFE_DISTANCE: f32 = 140.0;
const CRITICAL_DISTANCE: f32 = 45.0;
const SAFE_TIME_GAP: f32 = 0.5;
const CAR_LENGTH: f32 = 40.0;

pub struct Intersection {
    pub vehicles: Vec<Vehicle>,
    pub stats: Stats,
}

impl Intersection {
    // Creates a new Intersection with an empty list of vehicles and new stats.
    pub fn new() -> Self {
        Intersection {
            vehicles: Vec::new(),
            stats: Stats::new(),
        }
    }

    // Adds a new vehicle to the intersection if the spawn point is not blocked.
    pub fn add_vehicle(&mut self, car: Vehicle) {
        let spawn_blocked = self.vehicles.iter().any(|v| {
            v.direction == car.direction
                && v.lane == car.lane
                && (v.total_distance - v.distance) <= SAFE_DISTANCE
        });
        if !spawn_blocked {
            self.vehicles.push(car);
        }
    }

    // Updates the state of all vehicles in the intersection for a given time delta.
    pub fn update(&mut self, delta_time: f32) {
        let num_vehicles = self.vehicles.len();

        for i in 0..num_vehicles {
            let mut target_speed = 120.0;
            
             let mut has_near_missed: bool = false;
            for j in 0..num_vehicles {
                if i == j {
                    continue;
                }

                let car_i =  &self.vehicles[i];
                let car_j =  &self.vehicles[j];

                if car_i.direction == car_j.direction && car_i.lane == car_j.lane {
                    if car_j.distance < car_i.distance {
                        let gap: f32 = car_i.distance - car_j.distance;

                        if gap < CRITICAL_DISTANCE {
                            target_speed = 0.0;
                        } else if gap < SAFE_DISTANCE {
                            if target_speed > 50.0 {
                                target_speed = 50.0;
                            }
                            if target_speed > car_j.velocity {
                                target_speed = car_j.velocity;
                            }
                        }
                    }
                } else {
                    if car_i.lane == 'r' || car_j.lane == 'r' {
                        continue;
                    }

                    let is_opposite = match (&car_i.direction, &car_j.direction) {
                        (Direction::North, Direction::South)
                        | (Direction::South, Direction::North) => true,
                        (Direction::East, Direction::West) | (Direction::West, Direction::East) => {
                            true
                        }
                        _ => false,
                    };

                    if is_opposite && car_i.lane == 's' && car_j.lane == 's' {
                        continue;
                    }

                    let dist_to_center_i = match car_i.direction {
                        Direction::North | Direction::West => car_i.distance - 400.0 - 40.0,
                        _ => car_i.distance - 400.0,
                    };
                    let dist_to_center_j = match car_j.direction {
                        Direction::North | Direction::West => car_j.distance - 400.0 - 40.0,
                        _ => car_j.distance - 400.0,
                    };

                    let mut speed_for_this_j = 100.0;

                    let conflict_radius = 50.0;

                    let dist_enter_i = dist_to_center_i - conflict_radius;
                    let dist_exit_i: f32 = dist_to_center_i + conflict_radius + CAR_LENGTH;

                    let dist_enter_j = dist_to_center_j - conflict_radius;
                    let dist_exit_j = dist_to_center_j + conflict_radius + CAR_LENGTH;

                    let t_enter_i = dist_enter_i / 100.0;
                    let t_exit_i = dist_exit_i / 100.0;

                    let t_enter_j = dist_enter_j / 100.0;
                    let t_exit_j = dist_exit_j / 100.0;

                    let time_overlap = (t_enter_i < t_exit_j + SAFE_TIME_GAP)
                        && (t_enter_j < t_exit_i + SAFE_TIME_GAP);

                    if time_overlap {
                        let i_must_yield = dist_to_center_i > dist_to_center_j
                            || (dist_to_center_i == dist_to_center_j && car_i.id > car_j.id);

                        if dist_to_center_i > 360.0 {
                            speed_for_this_j = 100.0;
                        } else if i_must_yield {
                            if dist_exit_j > 0.0 {
                                let current_v_j: f32 = if car_j.velocity > 5.0 {
                                    car_j.velocity
                                } else {
                                    5.0
                                };
                                let t_exit_j = dist_exit_j / current_v_j;
                                let desired_t_enter_i = t_exit_j + SAFE_TIME_GAP;
                                let mut ideal_speed: f32 = dist_enter_i / desired_t_enter_i;

                                if ideal_speed > 100.0 {
                                    ideal_speed = 100.0;
                                }

                                if ideal_speed > 0.0 && ideal_speed < 25.0 {
                                    if dist_enter_i < CAR_LENGTH {
                                        ideal_speed = 0.0;
                                        if car_i.velocity > 1.0 && !has_near_missed {
                                            self.stats.record_near_miss();
                                            has_near_missed = true;
                                        }
                                    } else {
                                        ideal_speed = 25.0;
                                    }
                                }

                                speed_for_this_j = ideal_speed;
                            }
                        } else {
                            speed_for_this_j = 100.0;
                        }
                    }

                    target_speed = target_speed.min(speed_for_this_j);
                }
            }

            let current_v = self.vehicles[i].velocity;

            let ease_factor: f32 = 3.0 * delta_time;

            let mut new_v = current_v + (target_speed - current_v) * ease_factor;

            if target_speed == 0.0 && new_v < 1.0 {
                new_v = 0.0;
            }

            self.vehicles[i].velocity = new_v;
            self.stats.update_speeds(new_v);
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

        let car1 = Vehicle::new(1, 50.0, 10.0, 's', Direction::North);
        let car2 = Vehicle::new(2, 80.0, 10.0, 's', Direction::North);

       crossroad.vehicles.push(car1);
        crossroad.vehicles.push(car2);

        crossroad.update(0.1);

        assert!(crossroad.vehicles[1].velocity < 8.0,);
    }
    #[test]
    fn test_conflict_intersection() {
        let mut crossroad = Intersection::new();

        let car1 = Vehicle::new(1, 100.0, 10.0, 's', Direction::North);
        let car2 = Vehicle::new(2, 110.0, 11.0, 's', Direction::East);

        crossroad.add_vehicle(car1);
        crossroad.add_vehicle(car2);

        crossroad.update(0.1);

        assert!(crossroad.vehicles[1].velocity < 100.0,);
    }
}
