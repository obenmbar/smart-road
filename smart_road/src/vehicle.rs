#[derive(Debug,Clone,PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: u32,
    pub distance: f32,
    pub total_distance: f32 ,
    pub time: f32,
    pub velocity: f32,
    pub base_velocity: f32,
    pub lane: char,
    pub direction:Direction,
    pub time_spent: f32,
}

impl Vehicle {
    pub fn new(id: u32, distance: f32, time: f32, lane: char,direction:Direction) -> Self {
        let velocity = distance / time;
        Vehicle {
            id,
            distance,
            total_distance: distance,
            time,
            velocity,
            base_velocity:velocity,
            lane,
            direction,
            time_spent: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        
            self.distance -= self.velocity * delta_time;
            self.time -= delta_time;
            self.time_spent += delta_time;
        
    }
    pub fn is_at_intersection(&self) -> bool {
        self.distance <= 460.0 && self.distance >=240.0
    }
   
    pub fn has_turned(&self) -> bool {
        if self.lane == 's' { 
            return false; 
        }

        
        let turn_point = match (&self.direction, self.lane) {
            
            (Direction::South, 'r') | (Direction::East, 'r') => 471.0,
            (Direction::North, 'r') | (Direction::West, 'r') => 495.0,
            
            (Direction::South, 'l') | (Direction::East, 'l') => 371.0,
            (Direction::North, 'l') | (Direction::West, 'l') => 395.0,
            
            _ => 0.0,
        };

        self.distance <= turn_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_velocity() {
        let car =  Vehicle::new(1, 100.0, 10.0, 's', Direction::North);
        assert_eq!(car.velocity,10.0);
    }
    #[test]
    fn test_physics_update(){
        let mut car = Vehicle::new(1, 100.0, 10.0, 's', Direction::North);
        car.update(1.0);
        assert_eq!(car.distance,90.0);
        assert_eq!(car.time,9.0);
    }
}