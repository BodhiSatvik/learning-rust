use std::mem::size_of_val;

fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);
} 
fn main() {
    let c1 = '中';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}
fn main() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("hello, world");
    }
} 
fn main() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);
}
fn main() {
    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit())
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

// don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()")
}
use std::mem::size_of_val;

fn main() {
    let unit: () = ();
    // unit type doesn't occupy any memory space
    assert!(size_of_val(&unit) == 0);
}