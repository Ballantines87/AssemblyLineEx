fn main() {
    

    println!("The final production at this speed and for these hours is {}", production_rate_per_hour(200, 8));

    println!("At the current speed and for this may hours, we produce {} cars per minute ", cars_produced_per_minutes(200, 8));
}


fn production_rate_per_hour(num_hours: i32, speed: i32) -> i32 {
    
    loop {
        let production = match speed {
            1..=4 => speed * num_hours,
            5..=8 => speed * num_hours * 90/100,
            9..=10 => speed * num_hours * 77/100,
            _ => 0
        };
        if (production == 0) {
            println!("Max allowed speed: 10");
            break production
        } else {
            break production

        }
    }
    
}



fn cars_produced_per_minutes(num_hours: i32, speed: i32) -> i32 {
    loop {
        let car_per_min = match speed {
            1..=4 => speed * num_hours/60,
            5..=8 => (speed * num_hours/60) * 90/100,
            9..=10 => (speed * num_hours/60) * 77/100,
            _ => 0
        };
        if (car_per_min == 0) {
            println!("Max allowed speed: 10");
            break car_per_min
        } else {
            break car_per_min

        }
    }
}





