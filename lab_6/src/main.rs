//...existing code...
pub mod exercise_1;
pub mod exercise_2;
pub mod exercise_3;
pub mod exercise_4;
pub mod exercise_5;

fn main() {
    println!("Hello, world!");

    exercise_1::run();
    exercise_2::run();
    exercise_3::run();
    exercise_4::run().expect("exercise 4 failed");
    exercise_5::run();
}
