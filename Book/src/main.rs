fn main() {
    let b :bool= false;

    let _v: i32 = match b {
        true => 1,
        false => {
            println!("success");
            panic!("just panicking");
        }
    };
    println!("failed");
}