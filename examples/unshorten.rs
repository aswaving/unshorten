use std::env::args;
use unshorten::unshorten_recurse;

fn main() {
    let url = args().nth(1).unwrap();
    //    println!("shortened: {}", is_shortened(&url).unwrap());
    //    println!("target   : {}", unshorten(&url).unwrap());
    println!("targets: {:?}", unshorten_recurse(&url).unwrap());
}
