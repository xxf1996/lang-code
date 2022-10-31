/// 单个指令
enum Instr {
  Cst(i32),
  Add,
  Mul
}
/// 指令集
type Instrs = Vec<Instr>;
/// 操作数
type Operand = i32;
/// 操作栈
type Stack = Vec<Operand>;

/// 合法的表达式
#[derive(Debug)]
enum Expr<'a> {
  /// 常数
  Cst(i32),
  Add(&'a Expr<'a>, &'a Expr<'a>), // rust不允许递归类型有无限尺寸，因为需要通过指针来固定占用内存大小；https://stackoverflow.com/questions/25296195/why-are-recursive-struct-types-illegal-in-rust
  Mul(&'a Expr<'a>, &'a Expr<'a>)
}

/// 模拟stack machine执行过程
fn eval(instrs:&mut Instrs, s:&mut Stack) {
  if instrs.len() == 0 {
    return;
  }
  let instr = instrs.get(0).unwrap();
  match instr {
    Instr::Cst(val) => {
      s.insert(0, *val);
    },
    Instr::Add => {
      assert!(s.len() > 1, "stack: {:#?}", s);
      let v2 = s.remove(0);
      let v1 = s.remove(0);
      s.insert(0, v1 + v2);
    },
    Instr::Mul => {
      assert!(s.len() > 1);
      let v2 = s.remove(0);
      let v1 = s.remove(0);
      s.insert(0, v1 * v2);
    }
  }
  instrs.remove(0);
  eval(instrs, s)
}

/// 将表达式编译为指令
fn compile(expr: &Expr<'_>, instrs:&mut Instrs) {
  match expr {
    Expr::Cst(val) => {
      instrs.push(Instr::Cst(*val));
    },
    Expr::Add(e1, e2) => {
      compile(*e1, instrs);
      compile(*e2, instrs);
      instrs.push(Instr::Add);
    },
    Expr::Mul(e1, e2) => {
      compile(*e1, instrs);
      compile(*e2, instrs);
      instrs.push(Instr::Mul);
    }
  }
}

fn main() {
  let mut instrs: Instrs = vec![];
  let mut operand_stack: Stack = vec![];
  let v1 = Expr::Cst(5);
  let v2 = Expr::Cst(2);
  let e1 = Expr::Add(&v1, &v2);
  let e2 = Expr::Mul(&v1, &v2);
  let e3 = Expr::Add(&e1, &e2);
  compile(&e3, &mut instrs);
  eval(&mut instrs, &mut operand_stack);
  println!("{:#?}", operand_stack);
}