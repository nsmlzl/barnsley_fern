mod barnsley_farn {
    #[derive(Debug)]
    pub struct Point {
        pub x: f32,
        pub y: f32,
    }

    impl Point {
        pub fn next(&self) -> Point {
            Point {
                x: self.x + 1.0,
                y: self.y + 1.0,
            }
        }
        pub fn print(&self) {
            println!("{:?}", self);
        }
    }
}

fn main() {
    println!("barnsley farn");

    let mut pnt = barnsley_farn::Point {
        x: 0.0,
        y: 0.0,
    };
    for _ in 0..10 {
        pnt = pnt.next();
        pnt.print();
    }
}
