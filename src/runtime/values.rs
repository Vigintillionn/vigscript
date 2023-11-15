#[derive(Debug, Clone)]
pub enum RuntimeValue {
  Null,
  Number {
    value: i32
  },
  Bool {
    value: bool
  },
}