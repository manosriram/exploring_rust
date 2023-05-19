#![allow(dead_code, unused_variables)]
//
// use rand::Rng;
// fn say_hello(x: u32) -> u32 { x = x + 100;
    // x + 1
// }

// use std::{ops::Add, collections::HashMap};

// fn get_sum<T: Add<Output = T>>(x: T, y: T) -> T {
    // return x + y;
// }

use std::{fs::File, io::Read};

mod hello;

fn read_file() -> Result<String, std::io::Error> {
    let mut f = File::open("Cargo.toml")?;
    let mut data = String::new();

    f.read_to_string(&mut data)?;
    Ok(data)
}

fn main() {

    let r = read_file();
    match r {
        Ok(data) => {
            println!("{}", data);
        },
        Err(e) => println!("error reading file")
    }


    // println!("{:?}", f_data);
    // println!("{:?}", f);

    // let f_res = match f {
        // Ok(x) => x,
        // Err(e) => panic!("{}", e)
    // };


    // const PI: f32 = 3.14;

    // trait Shape {
        // fn new(l: f32, w: f32) -> Self;
        // fn area(&self) -> f32;
    // }

    // struct Rectangle {
        // l: f32,
        // w: f32,
    // }

    // struct Circle {
        // l: f32,
        // w: f32,
    // }

    // impl Shape for Rectangle {
        // fn new(l: f32, w: f32) -> Self {
            // Rectangle { l, w }
        // }

        // fn area(&self) -> f32 {
            // self.l * self.w
        // }
    // }

    // impl Shape for Circle {
        // fn new(l: f32, w: f32) -> Self {
            // Circle { l, w }
        // }

        // fn area(&self) -> f32 {
            // (self.l / 2.0).powf(2.0) * PI
        // }
    // }

    // let r: Rectangle = Shape::new(12.23, 123.3);
    // let c: Circle = Shape::new(12.23, 123.3);
    // println!("r area = {}", r.area());
    // println!("c area = {}", c.area());


    // hello::hello::h();
    // hello::hello::world::w();

    // let c: hello::Country = hello::Country{
        // name: String::from("india"),
    // };
    // println!("{}", c.name);

    // let mut h = HashMap::new();
    // h.insert("key1", "val1");
    // h.insert("key2", "val2");

    // for (k, v) in h.iter() {
        // println!("{}: {}", k, v);
    // }

    // if h.contains_key(&"key1") {
        // println!("yes");
    // }

    // println!("{}", get_sum(1, 2));
    // println!("{}", get_sum(1.123, 2.123));

    // let mut v1: Vec<i32> = Vec::new();
    // v1.push(123);

    // let mut v2 = vec![1, 2, 3, 44];

    // for x in v2.iter_mut() {
        // *x = *x + 1;
        // print!("{} ", x);
    // }

    // for xy in vx.iter() {
        // print!("{} ", xy);
    // }


    // {
        // let asd = "mano";
        // println!("{}", asd);
    // }
    // println!("{}", asd);


    // enum Day {
        // Monday,
        // Tuesday,
        // Wednesday,
        // Thursday,
        // Friday,
        // Saturday,
        // Sunday,
    // }

    // impl Day {
        // fn is_weekend(&self) -> bool {
            // match self {
                // Day::Saturday | Day::Sunday => true,
                // _ => false
            // }
        // }
    // }

    // println!("{}", Day::Monday.is_weekend());



    // let mut x = "hello";
    // println!("{x}");
    // x = "world";
    // println!("{x}");


    // let f: f32 = 1.1111111111111;
    // println!("{f}");

    // let rnd = rand::thread_rng().gen_range(1..101);
    // println!("{}", rnd);

    // if rnd > 50 {
        // println!("greater than 50");
    // } else if rnd < 50 {
        // println!("lesser than 50");
    // } else {
        // println!("equal to 50");
    // }


    // let mut y = 31;

    // y = say_hello(y);
    // println!("{}", y);

    // let ax = [1, 2, 3];

    // let mut idx = 0;

    // loop {
        // if idx >= ax.len() {
            // break;
        // }

        // println!("{}: {}", idx, ax[idx]);
        // idx += 1;
    // }

    // while idx < ax.len() {
        // println!("{}: {}", idx, ax[idx]);
        // idx += 1;
    // }


    // for v in ax {
        // println!("{}", v);
    // }

    // let heap_string = String::from("heap").to_string();

    // let b_string = "a some string";

    // let bb = b_string.bytes();
    // // println!("{}", bb);

    // for x in bb {
        // println!("{}", x);
    // }

    // let sl = &b_string[0..3];
    // println!("{}", sl);

}
