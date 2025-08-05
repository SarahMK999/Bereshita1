use crate::address::Address;


pub struct Person{
    pub name:String,
    pub age: i32,
    pub address: Address,
}

impl Person{
    pub fn new(name: String, age:i32, address: Address )->Self{
        Person{
            name, age, address
        }
    }

    pub fn print(&self){
        println!("name: {}, age: {}, street: {}, city: {}, zip: {}",
         self.name, self.age, self.address.street, self.address.city, self.address.zip);
    }
}