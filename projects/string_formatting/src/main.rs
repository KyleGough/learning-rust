use std::fmt;

fn main() {
    let pi: f64 = 3.141592;

    // Print floats to specified decimal places.
    println!("PI: {}", pi);
    println!("PI to 1dp: {:.1}", pi);
    println!("PI to 2dp: {:.2}", pi);
    println!("PI to 3dp: {pi:.n$}", pi=pi, n=3);

    // Right-align text with placeholder character.
    let board_row = "01001";
    println!("{board_row:x>10}");
    println!("{board_row:>10}");

    // Numerical bases.
    let x = 12345;
    println!("Base 10: {}", x);
    println!("Binary:  {:b}", x);
    println!("Octal:   {:o}", x);
    println!("Hex:     {:x}", x); // Lowercase hex
    println!("Hex:     {:X}", x); // Uppercase hex

    // Mixture.
    println!("{:5}cm", 20);
    println!("{x:>10}"); // Right-align in 10-wide column
    println!("{x:0>8}"); // Pad number with zeroes
    println!("{x:0>width$}", width=6); // Named arguments by appending '$'
    println!("{:^10}", x); // Centre align with width 10
    println!("{:+}", x); // Display numerical sign (+/-)
    println!("{:.dp$}", pi, dp=3); // Display number to N d.p.
    println!("{:>+20.2}", pi); // 2 d.p. right-aligned, 20 width, with sign.

    
    // fmt::Debug trait
    #[derive(Debug)]
    struct Structure(i32);

    println!("Print structure: {:?}", Structure(16));

    // fmt::Display
    struct DisplayStructure(i32);

    impl fmt::Display for DisplayStructure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    println!("Display: {}", DisplayStructure(32));
}
