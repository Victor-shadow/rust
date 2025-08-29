fn main(){
    let week_day = "Sunday";
    let day = match week_day {
        "Mon" => "Monday",
        "Tue" => "Tuesday",
        "Wed" => "Wednesday",
        "Thur" => "Thursday",
        "Fri" => "Friday",
        "Sat" => "Saturday",
        "Sunday" => {println!("Found match for Sunday"); "Sunday"},
        _=> "Unknown day of the week"
    };
    println!("Week day is: {}", day);
}