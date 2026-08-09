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

    pub fn update_speeds(&mut self, current_speed: f32) {
        if current_speed > self.max_speed {
            self.max_speed = current_speed;
        }
        if current_speed < self.min_speed {
            self.min_speed = current_speed;
        }
    }
    
    pub fn record_vehicle_exit(&mut self, time_taken: f32) {
        self.total_vehicles_crossed += 1;

        if time_taken > self.max_time {
            self.max_time = time_taken;
        }
        if time_taken < self.min_time {
            self.min_time = time_taken
        }
    }

    pub fn record_near_miss(&mut self) {
        self.near_misses += 1;
    }
}
