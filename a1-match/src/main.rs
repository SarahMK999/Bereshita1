fn main() {
    let grade = 87; 

    let result = match grade {
        90..=100 => "A", // 2 dots means range.
        80..=89  => "B",
        70..=79  => "C",
        60..=69  => "D",
        0..=59   => "F",
        _=> "Invalid grade", // def must have to run. 
    };

    println!("Your grade is: {}", result);
}