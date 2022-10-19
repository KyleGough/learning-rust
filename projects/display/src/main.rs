use std::fmt;

fn main() {
    point();
    complex();
}

fn point() {
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({},{})", self.x, self.y)
        }
    }

    println!("Debug:   {:?}", Point { x: 1.0, y: 3.0 });
    println!("Display: {}", Point { x: 2.0, y: 6.0 });
}

fn complex() {
    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let num: Complex = Complex { real: 3.3, imag: 7.2 };
    println!("Debug:   {:?}", num);
    println!("Display: {}", num);
}

