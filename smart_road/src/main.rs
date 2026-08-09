mod vehicle;
mod intersection;
use vehicle :: {Vehicle,Direction};
use intersection::Intersection;

fn main() {
    let mut mycrossroid = Intersection::new();
    let  mycar =  Vehicle::new(10,100.0, 10.0,'s',Direction::North);
     let car2 =  Vehicle::new(5,200.0,15.0,'l',Direction::East);
     mycrossroid.add_vehicle(mycar);
     mycrossroid.add_vehicle(car2);
     println!("الكروازوة فيها دابا: {:#?}", mycrossroid.vehicles);
}
