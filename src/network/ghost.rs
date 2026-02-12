// Ghost Reckoning - Dead Reckoning Navigation
use std::thread;
use std::time::Duration;
use rand::Rng;
use colored::Colorize;
use crate::error::Result;

pub struct GhostReckoner {
    lat: f64,
    lon: f64,
    heading: f64, // Degrees (0 = North)
    velocity: f64, // m/s
}

impl GhostReckoner {
    pub fn new(start_lat: f64, start_lon: f64) -> Self {
        Self {
            lat: start_lat,
            lon: start_lon,
            heading: 0.0,
            velocity: 0.0,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        println!("{} Initializing Ghost Reckoning (INS)...", "👻".cyan());
        println!("Start Fix: {:.6}, {:.6}", self.lat, self.lon);
        println!("{} Satellites acquired. Switching to Dead Reckoning.", "0".red());

        loop {
            // Simulate IMU updates (Random Walk behavior)
            self.simulate_imu_input();
            
            // Calculate new position based on velocity & heading (Dead Reckoning)
            // approximations for short distances: 
            // 1 deg lat ~= 111km, 1 deg lon ~= 111km * cos(lat)
            let dt = 1.0; // 1 second steps
            let dist_traveled = self.velocity * dt; // meters

            let r_earth = 6378137.0; // Radius of Earth in meters
            let dy = dist_traveled * self.heading.to_radians().cos();
            let dx = dist_traveled * self.heading.to_radians().sin();

            let d_lat = (dy / r_earth).to_degrees();
            let d_lon = (dx / (r_earth * self.lat.to_radians().cos())).to_degrees();

            self.lat += d_lat;
            self.lon += d_lon;

            // Output simulation state
            print!("\r\x1B[K"); 
            print!("{} [INS] Pos: {:.6}, {:.6} | Hdg: {:03.0}° | Spd: {:.1} m/s | {}", 
                "📍".yellow(),
                self.lat, 
                self.lon, 
                self.heading, 
                self.velocity,
                "ESTIMATED".red().dimmed()
            );
            
            use std::io::Write;
            std::io::stdout().flush().unwrap();
            
            thread::sleep(Duration::from_secs(1));
        }
    }

    fn simulate_imu_input(&mut self) {
        let mut rng = rand::thread_rng();
        
        // Randomly change velocity (simulate walking/stopping)
        let accel = rng.gen_range(-0.5..0.5);
        self.velocity = (self.velocity + accel).clamp(0.0, 2.0); // 0 to 2 m/s walking speed

        // Randomly change heading (simulate turning)
        let turn = rng.gen_range(-10.0..10.0);
        self.heading = (self.heading + turn).rem_euclid(360.0);
    }
}
