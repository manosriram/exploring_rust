// mod h;


pub mod hello {
    pub fn h() {
        println!("hello fn");
    }

    pub mod world {
        pub fn w() {
            println!("world fn");
        }
    }
}


pub struct Country {
    pub name: String,
}
