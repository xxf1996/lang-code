use std::rc::Rc;

/// 单个指令
#[derive(Debug)]
enum Instr {
  Cst(i32),
  Add,
  Mul,
  Pop,
  Swap,
  /// 取值指令，索引为栈顶索引
  Var(i32)
}
/// 指令集
type Instrs = Vec<Instr>;
/// 操作数
type Operand = i32;
/// 操作栈
type Stack = Vec<Operand>;
/// 具名表达式
#[derive(Debug)]
enum Expr<'a> {
  /// 常数
  Cst(i32),
  Add(&'a Expr<'a>, &'a Expr<'a>), // rust不允许递归类型有无限尺寸，因为需要通过指针来固定占用内存大小；https://stackoverflow.com/questions/25296195/why-are-recursive-struct-types-illegal-in-rust
  Mul(&'a Expr<'a>, &'a Expr<'a>),
  Var(String),
  Let(String, &'a Expr<'a>, &'a Expr<'a>)
}
/// 具名变量环境，(变量名，值)
type Env = Vec<(String, i32)>;

mod nameless {
  use std::rc::Rc;

  /// 不具名表达式
  #[derive(Debug, Clone)]
  pub enum Expr {
    /// 常数
    Cst(i32),
    Add(Rc<Expr>, Rc<Expr>), // rust不允许递归类型有无限尺寸，因为需要通过指针来固定占用内存大小；https://stackoverflow.com/questions/25296195/why-are-recursive-struct-types-illegal-in-rust
    Mul(Rc<Expr>, Rc<Expr>),
    Var(i32),
    Let(Rc<Expr>, Rc<Expr>)
  }
  pub type Env = Vec<i32>;
}

/// 具名表达式编译成不具名表达式的变量环境，这里列表只包含变量名，编译完就是静态的；对应partial evaluation；
type CompileEnv = Vec<String>;

/// 将具名的表达式编译成不具名的表达式
fn compile_to_nameless<'a>(expr: &Expr<'a>, cenv: &mut CompileEnv) -> nameless::Expr {
  match expr {
    Expr::Cst(var) => nameless::Expr::Cst(*var),
    Expr::Add(e1, e2) => {
      // 由于普通的指针不能从函数体返回（因为在函数内部创建的数据所有权在函数内，因此离开函数作用域后会自动drop掉）
      // 而Rc是一种引用计数指针，所以可以多对一的进行引用！（https://course.rs/advance/smart-pointer/rc-arc.html）
      let c1 = Rc::new(compile_to_nameless(e1, cenv));
      let c2 = Rc::new(compile_to_nameless(e2, cenv));
      nameless::Expr::Add(c1, c2)
    },
    Expr::Mul(e1, e2) => {
      let c1 = Rc::new(compile_to_nameless(e1, cenv));
      let c2 = Rc::new(compile_to_nameless(e2, cenv));
      nameless::Expr::Mul(c1, c2)
    },
    Expr::Var(name) => {
      // 找到变量名的位置
      let name_idx = cenv.iter().position(|elem| *name == *elem);
      if let Some(idx) = name_idx {
        nameless::Expr::Var(idx as i32)
      } else {
        panic!("name: {name}\ncenv: {:#?}", cenv)
      }
    },
    Expr::Let(name, e1, e2) => {
      let c1 = Rc::new(compile_to_nameless(e1, cenv));
      cenv.insert(0, name.clone()); // 入栈
      let c2 = Rc::new(compile_to_nameless(e2, cenv));
      nameless::Expr::Let(c1, c2)
    }
  }
}

/// 将不具名表达式编译成指令
fn compile(expr: &nameless::Expr, instrs: &mut Instrs) {
  match expr {
    nameless::Expr::Cst(val) => {
      instrs.push(Instr::Cst(*val));
    },
    nameless::Expr::Add(e1, e2) => {
      compile(&e1, instrs);
      compile(&e2, instrs);
      instrs.push(Instr::Add);
    },
    nameless::Expr::Mul(e1, e2) => {
      compile(&e1, instrs);
      compile(&e2, instrs);
      instrs.push(Instr::Mul);
    },
    nameless::Expr::Var(idx) => {
      instrs.push(Instr::Var(*idx));
    },
    nameless::Expr::Let(e1, e2) => {
      compile(&e1, instrs);
      compile(&e2, instrs);
      instrs.push(Instr::Swap);
      instrs.push(Instr::Pop);
    }
  }
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
    },
    Instr::Var(idx) => {
      let target = s.get(*idx as usize).unwrap();
      s.insert(0, *target); // 找到指定位置的值，并复制一份值到栈顶
    },
    Instr::Pop => {
      assert!(s.len() > 0);
      s.remove(0);
    },
    Instr::Swap => {
      assert!(s.len() > 1);
      s.swap(0, 1); // 直接交换栈顶两个元素
    }
  }
  instrs.remove(0);
  eval(instrs, s)
}

fn main() {
  let x = Expr::Var(String::from("x"));
  let y = Expr::Var(String::from("y"));
  let e1 = Expr::Add(&x, &Expr::Cst(45)); // x + 45
  let e2 = Expr::Let(String::from("x"), &Expr::Cst(11), &e1); // let x = 11; ↑
  // let e3 = Expr::Mul(&x, &Expr::Cst(2)); // x * 2 --> TODO: 这里作用域不属于let x之后，所以无法访问到正确的值？
  let e4 = Expr::Mul(&y, &Expr::Cst(3)); // y * 3;
  let e5 = Expr::Let(String::from("y"), &e2, &e4); // let y = x + 45; ↑
  let mut cenv: CompileEnv = vec![];
  let nameless_expr = compile_to_nameless(&e5, &mut cenv);
  println!("{:#?}", nameless_expr);
  let mut instrs: Instrs = vec![];
  let mut operand_stack: Stack = vec![];
  compile(&nameless_expr, &mut instrs);
  println!("指令集：{:#?}", instrs);
  eval(&mut instrs, &mut operand_stack);
  println!("stack: {:#?}", operand_stack);
}