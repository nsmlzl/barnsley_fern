use plotters::prelude::*;

mod barnsley_farn {
    #[derive(Debug)]
    pub struct Point {
        pub x: f32,
        pub y: f32,
    }
    pub struct PointU {
        pub x: i32,
        pub y: i32,
    }

    enum Case {
        F1,
        F2,
        F3,
        F4,
    }

    fn get_rnd_case() -> Case {
        use rand::Rng;
        let rnd_nbr = rand::thread_rng().gen_range(1, 101);
        match rnd_nbr {
            1 => Case::F1,
            2..=86 => Case::F2,
            87..=93 => Case::F3,
            94..=100 => Case::F4,
            _ => panic!("random number out of range!"),
        }
    }

    struct BarnsleyParameters(f32, f32, f32, f32, f32, f32);
    const P1: BarnsleyParameters = BarnsleyParameters( 0.00,  0.00,  0.00,  0.16,  0.00,  0.00);
    const P2: BarnsleyParameters = BarnsleyParameters( 0.85,  0.04, -0.04,  0.85,  0.00,  1.60);
    const P3: BarnsleyParameters = BarnsleyParameters( 0.20, -0.26,  0.23,  0.22,  0.00,  1.60);
    const P4: BarnsleyParameters = BarnsleyParameters(-0.15,  0.28,  0.26,  0.24,  0.00,  0.44);

    impl Point {
        pub fn next(&self) -> Point {
            let rnd_case = get_rnd_case();
            // get corresponding parameters
            let p = match rnd_case {
                Case::F1 => P1,
                Case::F2 => P2,
                Case::F3 => P3,
                Case::F4 => P4,
            };
            // calculate point
            let x = p.0 * self.x + p.1 * self.y + p.4;
            let y = p.2 * self.x + p.3 * self.y + p.5;
            Point {
                x,
                y,
            }
        }

        pub fn print(&self) {
            println!("{:?}", self);
        }
        pub fn convert(&self, max_x: u32, max_y: u32) -> PointU{
            let x = (self.x + 2.2) * max_x as f32 / 4.9;
            let x = x as i32;
            let y = max_y as f32 - (self.y * max_y as f32 / 10.0);
            let y = y as i32;
            PointU {
                x,
                y,
            }
        }
    }
}

fn main() {

    println!("barnsley farn");

    let plot_dim = [10000, 20000];
    let plot = BitMapBackend::new("fern.png", (plot_dim[0], plot_dim[1])).into_drawing_area();
    plot.fill(&BLACK);

    let mut pnt = barnsley_farn::Point {
        x: 0.0,
        y: 0.0,
    };
    for _ in 0..20000 {
        pnt = pnt.next();
        pnt.print();
        if pnt.x < -2.2 || pnt.x > 2.7 || pnt.y < 0.0 || pnt.y > 10.0 {
            panic!("Point out of range!");
        }
        let pnt_u = pnt.convert(plot_dim[0], plot_dim[1]);
        plot.draw(&Circle::new((pnt_u.x, pnt_u.y), 4, Into::<ShapeStyle>::into(&WHITE).filled()));
    }
}
