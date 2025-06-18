/// Trait で式を表現する
trait ExpSYM<T> {
    fn lit(n: i32) -> T;
    fn neg(e: T) -> T;
    fn add(e1: T, e2: T) -> T;
}

struct Expr;

/// 式を評価するインタープリタ
impl ExpSYM<i32> for Expr {
    fn lit(n: i32) -> i32 {
        n
    }

    fn neg(e: i32) -> i32 {
        -e
    }

    fn add(e1: i32, e2: i32) -> i32 {
        e1 + e2
    }
}

/// 式の文字列表現を生成するインタープリタ
impl ExpSYM<String> for Expr {
    fn lit(n: i32) -> String {
        n.to_string()
    }

    fn neg(e: String) -> String {
        format!("-{e}")
    }

    fn add(e1: String, e2: String) -> String {
        format!("({e1} + {e2})")
    }
}

// あとから新しい式 Mul を追加する
trait MulSYM<T> {
    fn mul(e1: T, e2: T) -> T;
}

impl MulSYM<i32> for Expr {
    fn mul(e1: i32, e2: i32) -> i32 {
        e1 * e2
    }
}

impl MulSYM<String> for Expr {
    fn mul(e1: String, e2: String) -> String {
        format!("({e1} * {e2})")
    }
}

fn main() {
    let eval_result: i32 = Expr::add(
        Expr::lit(1),
        Expr::neg(Expr::mul(Expr::lit(2), Expr::lit(3))),
    );
    dbg!(eval_result);

    let view_result: String = Expr::add(
        Expr::lit(1),
        Expr::neg(Expr::mul(Expr::lit(2), Expr::lit(3))),
    );
    dbg!(view_result);
}
