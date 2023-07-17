pub fn generate_random() {
    struct Foo{}
    
    let z = &Foo{} as *const _ as usize;
    let a: usize = 16807; 
    let m: usize = 0x7fffffff;
    
    let rez: usize = (z - a) % m;

    println!("{rez}");
}
