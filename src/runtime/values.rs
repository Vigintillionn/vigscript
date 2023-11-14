#[derive(Debug)]
pub enum RuntimeValue {
  Null,
  Number {
    value: i32
  }
}