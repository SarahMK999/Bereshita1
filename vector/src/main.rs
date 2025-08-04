fn main() {
//    let mut names: Vec<&str> = Vec::new();

//    names.push("sarah");
//    names.push("michael");
//    names.push("david");
   
// println!("{}", names.len());

// let removed = names.removed(1);

// for name in names{
//     println!("{}", name);
// }


   let mut names: Vec<String> = Vec::new();

   names.push("sarah".to_string());
   names.push("michael".to_string());
   names.push("david".to_string());
   
println!("{}", names.len());

let remove = names.remove(1);


for name in names{
    println!("{}", name);
}


}
