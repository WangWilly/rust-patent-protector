fn echo(id: &i32) {
    println!("{}", id)
}

fn mut_echo(id: &mut i32) {
    println!("{}", id)
}

fn main() {
    let mut a1 = 1;
    let a2 = &a1;
    let a3 = &a1;
    println!("{:?} {:?} {:?}", a1, a2, a3);

    let mut b1 = 1;
    mut_echo(&mut b1);
    mut_echo(&mut b1);
    println!("{:?}", b1);

    let mut c1 = 1;
    echo(&c1);
    mut_echo(&mut c1);
    println!("{:?}", c1);

    let mut d1 = 1;
    // let d2 = &d1;
    // let d3 = &mut d1;
    // println!("{:?} {:?} {:?}", d1, d2, d3);

    mut_echo(&mut d1);
    echo(&d1);
    println!("{:?}", d1);
}
