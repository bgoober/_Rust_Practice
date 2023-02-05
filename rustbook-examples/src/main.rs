struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // implement methods to define behavior on the type
    fn x(&self) -> &T {
        &self.x
    }

    // implement y method
    fn y(&self) -> &T {
        &self.y
    }
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

struct Person<T> {
    name: T,
    age: T,
}

// implement methods for Person
impl<T> Person<T> {
    fn name(&self) -> &T {
        &self.name
    }

    fn age(&self) -> &T {
        &self.age
    }
}


fn main() {

    // create a Point instance
    let p = Point { x: 5, y: 10 };

    // call the x method
    println!("p.x = {}", p.x());

    // call the y method
    println!("p.y = {}", p.y());

    // call Person methods
    let person = Person { name: "John", age: "30" };
    println!("person.name = {}", person.name());
    println!("person.age = {}", person.age());


    // create a Point2 instance
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
