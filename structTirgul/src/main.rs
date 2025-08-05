
mod address;
mod person;

use address::Address;
use person::Person;



fn main(){

    let p = Person:: new("sara".to_string(), 29, Address::new("hadera".to_string(), "ashdod".to_string(), 77878));
    p.print();


}


