use std::fmt;
// 单元结构体
struct Nil;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Left corner point, x: {}, y: {}", self.p1.x, self.p1.y)
    }
}

fn rect_area(rec: Rectangle) -> f32 {
    (rec.p1.x - rec.p2.x) * (rec.p1.y - rec.p2.y)
}

fn square(p: Point, length: f32) -> Rectangle {
    let left_corner_point: Point = Point {
        x: p.x - length,
        y: p.y - length,
    };
    Rectangle {
        p1: left_corner_point,
        p2: p,
    }
}

fn main() {
    // 实例化结构体 `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // 访问 point 的字段
    println!("({}, {})", point.x, point.y);

    // 使用 `let` 绑定来解构 point
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // 实例化一个单元结构体
    let _nil = Nil;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // 通过两个相距最远的点计算矩形面积
    let point1: Point = Point { x: 0.0, y: 0.0 };
    let point2: Point = Point { x: 5.0, y: 3.6 };
    let rectangle: Rectangle = Rectangle {
        p1: point1,
        p2: point2,
    };
    println!("Area of rectangle: {:?}", rect_area(rectangle));

    // 根据左下角的点的坐标和边长返回正方形右上角点的坐标
    println!("{}", square(Point {x: 5.0, y: 5.0}, 3.0));
}