#[derive(Debug, Clone)]
pub struct Stats {
    pub total_vehicles_crossed: u32,
    pub max_speed: f32,
    pub min_speed: f32,
    pub max_time: f32,
    pub min_time: f32,
    pub near_misses: u32,
}

impl Stats {
    // Creates a new Stats instance with default initial values.
    pub fn new() -> Self {
        Stats {
            total_vehicles_crossed: 0,
            max_speed: 0.0,
            min_speed: f32::MAX,
            max_time: 0.0,
            min_time: f32::MAX,
            near_misses: 0,
        }
    }

    // Updates the maximum and minimum speed records.
    pub fn update_speeds(&mut self, current_speed: f32) {
        if current_speed > self.max_speed {
            self.max_speed = current_speed;
        }
        if current_speed < self.min_speed {
            self.min_speed = current_speed;
        }
    }
    
    // Records a vehicle exiting and updates total crossed vehicles and time stats.
    pub fn record_vehicle_exit(&mut self, time_taken: f32) {
        self.total_vehicles_crossed += 1;

        if time_taken > self.max_time {
            self.max_time = time_taken;
        }
        if time_taken < self.min_time {
            self.min_time = time_taken
        }
    }

    // Increments the count of near misses.
    pub fn record_near_miss(&mut self) {
        self.near_misses += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_initialization() {
        let stats = Stats::new();
        
        assert_eq!(stats.total_vehicles_crossed, 0);
        assert_eq!(stats.max_speed, 0.0);
        assert_eq!(stats.min_speed, f32::MAX);
        assert_eq!(stats.max_time, 0.0);
        assert_eq!(stats.min_time, f32::MAX);
        assert_eq!(stats.near_misses, 0);
    }

    #[test]
    fn test_update_speeds() {
        let mut stats = Stats::new();
        
        stats.update_speeds(50.0);
        assert_eq!(stats.max_speed, 50.0);
        assert_eq!(stats.min_speed, 50.0);

        stats.update_speeds(120.0);
        assert_eq!(stats.max_speed, 120.0); 
        assert_eq!(stats.min_speed, 50.0);  

        stats.update_speeds(10.0);
        assert_eq!(stats.max_speed, 120.0);
        assert_eq!(stats.min_speed, 10.0); 
    }

    #[test]
    fn test_record_vehicle_exit() {
        let mut stats = Stats::new();
        
        stats.record_vehicle_exit(15.5);
        assert_eq!(stats.total_vehicles_crossed, 1);
        assert_eq!(stats.max_time, 15.5);
        assert_eq!(stats.min_time, 15.5);


        stats.record_vehicle_exit(20.0);
        assert_eq!(stats.total_vehicles_crossed, 2); 
        assert_eq!(stats.max_time, 20.0); 
        assert_eq!(stats.min_time, 15.5);

        stats.record_vehicle_exit(10.0);
        assert_eq!(stats.total_vehicles_crossed, 3);
        assert_eq!(stats.max_time, 20.0);
        assert_eq!(stats.min_time, 10.0); 
    }

    #[test]
    fn test_record_near_miss() {
        let mut stats = Stats::new();
        
        assert_eq!(stats.near_misses, 0);
        
        stats.record_near_miss();
        assert_eq!(stats.near_misses, 1);
        
        stats.record_near_miss();
        assert_eq!(stats.near_misses, 2);
    }
}