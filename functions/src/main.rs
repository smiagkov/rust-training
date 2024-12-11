fn main() {
    test();
}

fn test() {
    assert_eq!(double_int32(20), 40);
    assert_eq!(double_int64(20), 40_i64);
    assert!((double_float32(20.) - 40.).abs() < f32::EPSILON);
    assert!((double_float64(20.) - 40_f64).abs() < f64::EPSILON);
    assert!((int_plus_float_to_float(10, 20.) - 30_f64).abs() < f64::EPSILON);
    assert_eq!(int_plus_float_to_int(10, 20.1), 30_i64);
    assert_eq!(tuple_sum((10, 20)), 30_i64);
    assert_eq!(array_sum([10, 20, 30]), 60_i64);
}

fn double_int32(number: i32) -> i32 {
    number * 2
}

fn double_int64(number: i32) -> i64 {
    (number as i64) * 2
}

fn double_float32(number: f32) -> f32 {
    number * 2.
}

fn double_float64(number: f32) -> f64 {
    (number as f64) * 2.
}

fn int_plus_float_to_float(num1: i32, num2: f32) -> f64 {
    num1 as f64 + num2 as f64
}

fn int_plus_float_to_int(num1: i32, num2: f32) -> i64 {
    num1 as i64 + num2 as i64
}

fn tuple_sum(tuple: (i32, i32)) -> i64 {
    (tuple.0 + tuple.1) as i64
}

fn array_sum(array: [i32; 3]) -> i64 {
    (array[0] + array[1] + array[2]) as i64
}
