pub struct Address {
    pub street: String,
    pub city: String,
    pub zip: i32,
}

impl Address{
    pub fn new( street: String, city: String, zip: i32) ->Self{
        Address{street, city, zip}
    }

}