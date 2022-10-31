#[derive(Debug)]
enum Expr<'a> {
  Cst(i32),
  Add(&'a Expr<'a>, &'a Expr<'a>), // rust不允许递归类型有无限尺寸，因为需要通过指针来固定占用内存大小；https://stackoverflow.com/questions/25296195/why-are-recursive-struct-types-illegal-in-rust
  Mul(&'a Expr<'a>, &'a Expr<'a>)
}

impl Expr<'_> {
  fn eval(&self) -> Option<i32> {
    match self {
      Self::Cst(val) => Some(*val),
      Self::Add(left, right) => {
        let left_val = left.eval();
        let right_val = right.eval();
        if left_val.is_some() && right_val.is_some() {
          Some(left_val.unwrap() + right_val.unwrap())
        } else {
          None
        }
      },
      Self::Mul(left, right) => {
        let left_val = left.eval();
        let right_val = right.eval();
        if left_val.is_some() && right_val.is_some() {
          Some(left_val.unwrap() * right_val.unwrap())
        } else {
          None
        }
      },
    }
  }
}

fn main() {
  let v1 = Expr::Cst(32);
  let v2 = Expr::Cst(13);
  let e1 = Expr::Add(&v1, &v2);
  let e2 = Expr::Mul(&v1, &v2);
  assert_eq!(v1.eval(), Some(32));
  assert_eq!(v2.eval(), Some(13));
  assert_eq!(e1.eval(), Some(32 + 13));
  assert_eq!(e2.eval(), Some(32 * 13));
  println!("e1: {:#?}\n res: {:?}", e1, e1.eval())
}
