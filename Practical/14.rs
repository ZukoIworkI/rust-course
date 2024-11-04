use std::cmp::{max, min};

struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

// Функція для обчислення площі перекриття двох прямокутників
fn overlap_area(r1: &Rectangle, r2: &Rectangle) -> i32 {
    let x_overlap = max(0, min(r1.b.x, r2.b.x) - max(r1.a.x, r2.a.x));
    let y_overlap = max(0, min(r1.b.y, r2.b.y) - max(r1.a.y, r2.a.y));
    x_overlap * y_overlap
}

// Основна функція для обчислення загальної площі без перекриттів
fn area_occupied(rectangles: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;

    for rect in rectangles {
        let width = rect.b.x - rect.a.x;
        let height = rect.b.y - rect.a.y;
        total_area += width * height;
    }

    for i in 0..rectangles.len() {
        for j in (i + 1)..rectangles.len() {
            total_area -= overlap_area(&rectangles[i], &rectangles[j]);
        }
    }

    total_area
}

// Тестові дані
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle { a: Point { x: 2, y: 9 }, b: Point { x: 5, y: 3 } },
        Rectangle { a: Point { x: 1, y: 8 }, b: Point { x: 11, y: 7 } },
        Rectangle { a: Point { x: 9, y: 10 }, b: Point { x: 13, y: 2 } },
    ]
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}