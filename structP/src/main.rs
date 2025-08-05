// mod book;
// use book::Book;

// fn main() {
//     let mut b = Book::new("peter-pan".to_string(), 345);
//     b.print();

//     if b.is_big() {
//         println!("This is a big book!");
//     } else {
//         println!("This is a small book.");
//     }
// }

mod rectangle;
use rectangle:: Rectangle;

fn main() {

    let r = Rectangle::new(7, 3);
    println!("{}", r.area());
}
