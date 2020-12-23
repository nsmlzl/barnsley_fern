use plotters::prelude::*;

mod barnsley_fern {
    struct BParameters(f64, f64, f64, f64, f64, f64);
    const P1: BParameters = BParameters( 0.00,  0.00,  0.00,  0.16,  0.00,  0.00);
    const P2: BParameters = BParameters( 0.85,  0.04, -0.04,  0.85,  0.00,  1.60);
    const P3: BParameters = BParameters( 0.20, -0.26,  0.23,  0.22,  0.00,  1.60);
    const P4: BParameters = BParameters(-0.15,  0.28,  0.26,  0.24,  0.00,  0.44);

    // Barnsley Set
    enum BSet {
        F1,
        F2,
        F3,
        F4,
    }

    impl BSet {
        // get new random set
        fn new() -> BSet {
            use rand::Rng;
            let rnd_nbr = rand::thread_rng().gen_range(1, 101);
            match rnd_nbr {
                1 => BSet::F1,
                2..=86 => BSet::F2,
                87..=93 => BSet::F3,
                94..=100 => BSet::F4,
                _ => panic!("random number out of range!"),
            }
        }

        fn get_parameters(&self) -> &BParameters {
            match self {
                BSet::F1 => &P1,
                BSet::F2 => &P2,
                BSet::F3 => &P3,
                BSet::F4 => &P4,
            }
        }
    }

    #[derive(Debug)]
    pub struct Point<T> {
        pub x: T,
        pub y: T,
    }

    impl Point<f64> {
        pub fn print(&self) {
            println!("{:?}", self);
        }

        pub fn new() -> Point<f64> {
            Point {
                x: 0.0,
                y: 0.0,
            }
        }

        pub fn next(&self) -> Point<f64> {
            let barnsley_set = BSet::new();
            let p = barnsley_set.get_parameters();
            // calculate next point
            let x = p.0 * self.x + p.1 * self.y + p.4;
            let y = p.2 * self.x + p.3 * self.y + p.5;
            Point {
                x,
                y,
            }
        }

        // convert point<f64> to point<i32> (for plotting)
        pub fn convert(&self, max_x: u32, max_y: u32) -> Point<i32>{
            // convert from barnsley fern range to plot range
            // -2.2 < x <  2.7 => 0 < x < max_x
            //  0.0 < y < 10.0 => 0 < y < max_y
            let x = (self.x + 2.2) * max_x as f64 / 4.9;
            let x = x as i32;
            let y = max_y as f64 - self.y * max_y as f64 / 10.0;
            let y = y as i32;
            Point {
                x,
                y,
            }
        }
    }
}

fn main() {
    println!("barnsley fern");

    // init plot
    let plot_dim = [10000, 20000];
    let plot = BitMapBackend::new("fern.png", (plot_dim[0], plot_dim[1])).into_drawing_area();
    plot.fill(&BLACK).unwrap();

    let mut pnt = barnsley_fern::Point::new();
    for _i in 0..20_000_000 {
        pnt = pnt.next();
        pnt.print();

        if pnt.x < -2.2 || pnt.x > 2.7 || pnt.y < 0.0 || pnt.y > 10.0 {
            panic!("Point out of range!");
        }

        let pnt_i = pnt.convert(plot_dim[0], plot_dim[1]);
        plot.draw(&Circle::new((pnt_i.x, pnt_i.y), 1, Into::<ShapeStyle>::into(&WHITE).filled())).unwrap();
    }
}
