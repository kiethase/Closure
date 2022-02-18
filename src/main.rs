use std::thread;
use std::time::Duration;



fn main() {
    let simu_intensity = 10;
    let simu_rand_number = 8;


    generate_workout(simu_intensity, simu_rand_number);
}

// Create struct to make the make not duplicate the func and just take the value in there
// use generic and trait to make it
struct Cacher<T> where T: Fn(u32) ->u32 {
    calculation:T,
    value: Option<u32>,
} 

// Make a Imp for the Cacher to reduct duplicate func running and the delay just take the value in
impl<T> Cacher<T> 
where T: Fn(u32) -> u32{
    fn new(calculation:T)-> Cacher<T>{
        Cacher{
            calculation,
            value: None,
        }
    }

    fn value(&mut self,arg: u32)->u32{
        match self.value{
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// Create func generate workout
// And now we see a lot of duplicate call func in if else
// Now we gonna use closure to reduce duplicattion call

fn generate_workout(intensity: u32, rand_number: u32) {
    // First we gonna use let to get result from workout calculation function
    // Second we gonna use Closure to make the func more efficient with rand_number = 3

    let mut cacher_result = Cacher::new( |num: u32| -> u32 {
        println!("Calculating ......");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, we gonna do {} pushups!",
            cacher_result.value(intensity)
        );
        println!(
            "Next, we gonna do {} situps!",
            cacher_result.value(intensity)
        );
    } else {
        if rand_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today run for {} minutes",
                cacher_result.value(intensity)
            );
        }
    }
}
