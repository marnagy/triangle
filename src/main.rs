// nacitat strany, obvod, typ trojuholnika podla uhlov
use std::io;

fn main() {
    let mut sides: [i32; 3] = [0, 0, 0];
    let mut input = String::new();

    for (i, item) in sides.iter_mut().enumerate() {
        println!("Input side number {}", i + 1);
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                *item = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => panic!("Incorrect number!"),
                };
            }
            Err(_) => panic!("No input? PANIC!"),
        }
        input.clear();
    }

    let trngl = match Triangle::from_array(sides) {
        Ok(triangle) => triangle,
        Err(reason) => panic!(reason),
    };

    println!("Obvod trojuholnika ja {0}", trngl.triangle_circumference());
    println!("Plocha trojuhonika je {0}", trngl.triangle_area());

    if trngl.is_equilateral() {
        println!("Trojuholnik je rovnostranny.");
    } else {
        println!("Trojuholnik nie je rovnostranny.");
    }

    if trngl.is_isosceles() {
        println!("Trojuholnik je rovnoramenny.");
    } else {
        println!("Trojuholnik nie je rovnoramenny.");
    }

    if trngl.has_right_angle() {
        println!("Trojuholnik je pravouhly.");
    } else {
        println!("Trojuholnik nie je pravouhly.");
    }
}

struct Triangle {
    a: u32,
    b: u32,
    c: u32,
}

impl Triangle {
    fn from_array(sides: [i32; 3]) -> Result<Triangle, String> {
        if Triangle::is_valid_triangle(sides) {
            Ok(Triangle {
                a: sides[0] as u32,
                b: sides[1] as u32,
                c: sides[2] as u32,
            })
        } else {
            Err(String::from("Numbers cannot make a triangle."))
        }
    }

    fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    fn is_isosceles(&self) -> bool {
        (self.a == self.b && self.c != self.a)
            || (self.b == self.c && self.a != self.b)
            || (self.c == self.a && self.b != self.c)
    }

    fn is_valid_triangle(sides: [i32; 3]) -> bool {
        sides[0] >= 1 && sides[1] >= 1 && sides[2] >= 1
            && sides[0] + sides[1] > sides[2]
            && sides[1] + sides[2] > sides[0]
            && sides[2] + sides[0] > sides[1]
    }

    fn has_right_angle(&self) -> bool {
        let mut sides = [self.a, self.b, self.c];
        sides.sort();
        sides[0] * sides[0] + sides[1] * sides[1] == sides[2] * sides[2]
    }

    fn triangle_area(&self) -> f32 {
        let s: f32 = self.sum_sides() as f32 / 2_f32;
        let s: f32 = (s - (self.a as f32)) * (s - (self.b as f32)) * (s - (self.c as f32)) * s;
        s.sqrt()
    }

    fn sum_sides(&self) -> u32 {
        self.a + self.b + self.c
    }

    fn triangle_circumference(&self) -> u32 {
        self.sum_sides()
    }
}
