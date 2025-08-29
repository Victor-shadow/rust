enum CarType{
    Hatch,
    Sedan,
    SUV,
}

fn print_size(car:CarType){
     match car {
         CarType::Hatch => {
             println!("Small sized Hatch car")
         },

         CarType::Sedan => {
             println!("Medium sized car")
         },

         CarType::SUV => {
             println!("Large Sized Car")
         }
     }
}
fn main() {
   print_size(CarType::SUV);
    print_size(CarType::Hatch);
    print_size(CarType::Sedan);
}
//the match statement can be used to compare values stored in an enum