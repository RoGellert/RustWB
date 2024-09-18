// структура точки
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// конструктор
impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

// формула расстояния
fn distance_between_points(p1: &Point, p2: &Point) -> f32 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}

fn main() {
    // тест
    let p1 = Point::new(2.0, 0.0);
    let p2 = Point::new(4.0, 2.0);

    println!(
        "Расстояние между {:?} и {:?} = {:?} ",
        &p1,
        &p2,
        distance_between_points(&p1, &p2)
    );
}
