fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;

        x_cube + x_squared + x;
    };

    let z = {
        2 * x;
    };
}

fn main() {
    let v = {
        let mut x = 1;
        x += 2
    };
 
    assert_eq!(v, ());
 }
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
}
fn main() {
    let v = {
        let x = 3;
        x
    };
 
    assert!(v == 3);
}
fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}