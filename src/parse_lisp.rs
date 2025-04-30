//! # 736. Lisp 语法解析
//!
//! 难度 困难
//!
//! 给定一个类似 Lisp 语句的表达式 expression，求出其计算结果。
//!
//! 表达式语法如下所示:
//!
//! * 表达式可以为整数，let 语法，add 语法，mult 语法，或赋值的变量。表达式的结果总是一个整数。
//! * (整数可以是正整数、负整数、0)
//! * **let** 语法表示为 `(let v1 e1 v2 e2 ... vn en expr)`, 其中 let语法总是以字符串 `"let"` 来表示，接下来会跟随一个或多个交替变量或表达式，也就是说，第一个变量 `v1` 被分配为表达式 `e1` 的值，第二个变量 `v2` 被分配为表达式 `e2` 的值，以此类推；最终 `let` 语法的值为 `expr` 表达式的值。
//! * `add` 语法表示为 `(add e1 e2)`，其中 `add` 语法总是以字符串 `"add"` 来表示，该语法总是有两个表达式`e1`、`e2`, 该语法的最终结果是 `e1` 表达式的值与 `e2` 表达式的值之和。
//! * `mult` 语法表示为 `(mult e1 e2)` ，其中 `mult` 语法总是以字符串 `"mult"` 表示， 该语法总是有两个表达式 `e1`、`e2`，该语法的最终结果是 `e1` 表达式的值与 `e2` 表达式的值之积。
//! * 在该题目中，变量的命名以小写字符开始，之后跟随 `0` 个或多个小写字符或数字。为了方便，`"add"`，`"let"`，`"mult"` 会被定义为 "关键字"，不会在表达式的变量命名中出现。
//! * 最后，要说一下作用域的概念。计算变量名所对应的表达式时，在计算上下文中，首先检查最内层作用域（按括号计），然后按顺序依次检查外部作用域。我们将保证每一个测试的表达式都是合法的。有关作用域的更多详细信息，请参阅示例。
//!
//!
//! ## 示例：
//!
//! ```text
//! 输入: (add 1 2)
//! 输出: 3
//!
//! 输入: (mult 3 (add 2 3))
//! 输出: 15
//!
//! 输入: (let x 2 (mult x 5))
//! 输出: 10
//!
//! 输入: (let x 2 (mult x (let x 3 y 4 (add x y))))
//! 输出: 14
//! 解释:
//! 表达式 (add x y), 在获取 x 值时, 我们应当由最内层依次向外计算, 首先遇到了 x=3, 所以此处的 x 值是 3.
//!
//!
//! 输入: (let x 3 x 2 x)
//! 输出: 2
//! 解释: let 语句中的赋值运算按顺序处理即可
//!
//! 输入: (let x 1 y 2 x (add x y) (add x y))
//! 输出: 5
//! 解释:
//! 第一个 (add x y) 计算结果是 3，并且将此值赋给了 x 。
//! 第二个 (add x y) 计算结果就是 3+2 = 5 。
//!
//! 输入: (let x 2 (add (let x 3 (let x 4 x)) x))
//! 输出: 6
//! 解释:
//! (let x 4 x) 中的 x 的作用域仅在()之内。所以最终做加法操作时，x 的值是 2 。
//!
//! 输入: (let a1 3 b2 (add a1 1) b2)
//! 输出: 4
//! 解释:
//! 变量命名时可以在第一个小写字母后跟随数字.
//! ```
//!
//!
//! ## 注意:
//!
//! * 我们给定的 `expression` 表达式都是格式化后的：表达式前后没有多余的空格，表达式的不同部分(关键字、变量、表达式)之间仅使用一个空格分割，并且在相邻括号之间也没有空格。我们给定的表达式均为合法的且最终结果为整数。
//! * 我们给定的表达式长度最多为 2000 (表达式也不会为空，因为那不是一个合法的表达式)。
//! * 最终的结果和中间的计算结果都将是一个 32 位整数。
//!
//! See [leetcode](https://leetcode-cn.com/problems/parse-lisp-expression/)
//!

use std::collections::{HashMap, HashSet};
use std::iter::{Fuse, Peekable};

/// 用 Rust 写代码，无法对错误视而不见。 只有在最后调用的时候假设输入都是合法的。
///
/// 代码看起来挺长，其实很简单，代码结构也很清晰。
///
/// 主要思路是递归，先 tokenize 输入，这里因为用了 [`Iterator`](Iterator) tokenize 返回的时候是没有遍历的。
///
/// lisp 语法规则很简单，直接看 `eval()`。
///
/// 主要逻辑在 `eval()` 里，很简单，就是处理 `let` 语句稍微需要注意下，进入 `eval_let()` 之前需要入栈，`eval_let()` 返回后需要出栈。
///
/// 在 `eval_let()` 里循环处理赋值语句，最后表达式求值返回。
///
/// 另外要说的是，Rust 实现的思路一般不像 C 那样直接处理下标，而是利用高级别的抽象（例如 [`Iterator`](Iterator)）这种去处理，只要遍历一次。
///
/// 栈的实现，没有在进入下一个作用域的时候拷贝变量空间，而是把当前作用域的变量名对应的键值对入栈，出栈的时候先删除当前作用域里的变量，再从整个栈里恢复变量空间。
pub struct Solution;

type Result<T> = std::result::Result<T, String>;

impl Solution {

    pub fn evaluate(expression: String) -> i32 {
        let mut e = Evaluation::new(tokenize(&expression));
        // assume all inputs will be valid
        let res = e.eval(None).unwrap_or_else(|e| panic!("{}", e));
        // 如果还有符号，说明输入是非法的
        assert!(e.is_end(), "Unexpected input");
        res
    }
}

struct Evaluation<T: Iterator> {
    /// stacks for each level local variables
    stacks: Vec<HashMap<String, i32>>,

    /// entire namespace in the current level
    vars: HashMap<String, i32>,

    tokens: Peekable<Fuse<T>>,
}

impl<T: Iterator> Evaluation<T> {
    fn new(tokens: T) -> Self {
        Self {
            stacks: Vec::new(),
            vars: HashMap::new(),
            tokens: tokens.fuse().peekable(),
        }
    }
}

impl<T: Iterator<Item = Token>> Evaluation<T> {

    #[inline]
    fn expect(&mut self, expect: Token) -> Result<()> {
        self.tokens.next()
            .ok_or_else(|| format!("Invalid, missing required token: {:?}", expect))
            .and_then(|token| {
                if token == expect {
                    Ok(())
                } else {
                    Err(format!("Invalid, expect token: {:?}, found: {:?}", expect, token))
                }
            })
    }

    #[inline]
    fn is_end(&mut self) -> bool {
        self.tokens.peek().is_none()
    }

    // Evaluate expression
    fn eval(&mut self, local: Option<&HashSet<String>>) -> Result<i32> {
        match self.tokens.next() {
            Some(Token::LeftParenthesis) => {
                self.eval(local).and_then(|res| {
                    // expect RightParenthesis
                    self.expect(Token::RightParenthesis)?;
                    Ok(res)
                })
            }
            Some(Token::Let) => {
                // 进入 let 语句的时候，需要入栈
                // 退出 let 语句的时候，需要出栈
                self.push_stack(local);
                let mut local = HashSet::new();
                let res = self.eval_let(&mut local)?;
                self.pop_stack(Some(&local));
                Ok(res)
            }
            Some(Token::Add) => {
                self.eval(local).and_then(|e1| self.eval(local).map(|e2| e1 + e2 ))
            }
            Some(Token::Mult) => {
                self.eval(local).and_then(|e1| self.eval(local).map(|e2| e1 * e2 ))
            }
            Some(Token::Const(num))  => {
                Ok(num)
            }
            Some(Token::Ident(var)) => {
                self.vars.get(&var).map(|&v| v).ok_or_else(||format!("Not found var: `{}`", var))
            }
            _ => {
                Err("Invalid token".into())
            }
        }
    }

    /// Evaluate `let` expression, from the first item after `let`
    fn eval_let(&mut self, local: &mut HashSet<String>) -> Result<i32> {
        loop {
            // 分两种情况，先处理键值对的赋值语句，然后处理 `let` 语句中最后的 expr 部分
            match self.tokens.peek() {

                // 如果下一个表达式是标识符，那么有以下几种符合的情况
                Some(Token::Ident(_var)) => {
                    let ident = match self.tokens.next().unwrap() {
                        Token::Ident(s) => s,
                        _ => unreachable!(),
                    };

                    match self.tokens.peek() {
                        // assignment a var with a parenthesis expression (...) or a constant or a variable
                        // 赋值表达式，给之前的 ident 赋值，可以是 括号表达式、常量表达式、变量表达式，求值后更新变量空间，同时记录 local 变量名，循环处理
                        Some(Token::LeftParenthesis) | Some(Token::Const(_)) | Some(Token::Ident(_)) => {
                            let res = self.eval(Some(&local))?;
                            self.vars.insert(ident.clone(), res);
                            local.insert(ident);
                        }
                        // evaluate the last expression of let
                        // 如果标识符之后接着是右括号，说明这个标识符是 `let` 语句的最后一部分，也就是说这个标识符是变量表达式，可以直接求值返回
                        Some(Token::RightParenthesis) => {
                            return self.vars.get(&ident).map(|&v| v).ok_or_else(||format!("Not found var: `{}`", ident));
                        }
                        _ => {
                            return Err("Invalid let, expect expression".into());
                        }
                    }
                }

                // let expr part is const or is (...) expr
                // 如果最后一个表达式是常量表达式或者是括号表达式，可以直接求值回返，紧接着之后一定是一个右括号，否则就不合法
                Some(Token::Const(_)) | Some(Token::LeftParenthesis) => {
                    return self.eval(Some(&local));
                    // next should be Token::RightParenthesis
                }

                _ => {
                    return Err("Invalid `let` expression".into());
                }
            }
        }
    }

    /// 从变量空间里找到当前使用域里的变量，入栈
    fn push_stack(&mut self, local: Option<&HashSet<String>>) {
        local.map(|local| {
            let mut stack = HashMap::new();
            for k in local.iter() {
                if let Some(v) = self.vars.get(k) {
                    stack.insert(k.clone(), v.clone());
                }
            }
            self.stacks.push(stack);
        });
    }

    /// 从变量空间里删除当前作用域里的变量，然后从栈里恢复所有变量空间里的值，然后弹出栈顶
    fn pop_stack(&mut self, local: Option<&HashSet<String>>) {
        local.map(|local| {
            for k in local.iter() {
                self.vars.remove(k);
            }

            for stack in &self.stacks {
                for (k, v) in stack {
                    self.vars.insert(k.clone(), v.clone());
                }
            }
            self.stacks.pop();
        });
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    /// 常量
    Const(i32),

    /// 标识符，可以是用于赋值左侧的变量名，也可以是一个求值表达式
    Ident(String),

    /// Keyword `let`
    Let,

    /// Keyword `add`
    Add,

    /// Keyword `mult`
    Mult,

    /// Left parenthesis `(`
    LeftParenthesis,

    /// Right parenthesis `)`
    RightParenthesis,
}

impl Token {
    fn from_str<S: AsRef<str>>(s: S) -> Self {
        let s = s.as_ref();
        match s {
            "(" => Self::LeftParenthesis,
            ")" => Self::RightParenthesis,
            "let" => Self::Let,
            "add" => Self::Add,
            "mult" => Self::Mult,
            c if c.len() > 0 => {
                match c.chars().next().unwrap() {
                    'a'..='z' => Self::Ident(s.into()),
                    _ => Self::Const(s.parse().unwrap()),  // assume all expr valid
                }
            }
            _ => {
                panic!("Invalid");
            }
        }
    }
}

fn tokenize<'a>(s: &'a str) -> impl Iterator<Item = Token> + 'a {
    let mut chars = s.chars();
    let mut cache = String::new();
    std::iter::from_fn(move || {
        while let Some(c) = chars.next() {
            match c {
                '('  => return Some(Token::from_str("(")),
                v @ ')' => {
                    if cache.len() > 0 {
                        let next = Some(Token::from_str(cache.drain(..).collect::<String>()));
                        cache.push(v);
                        return next;
                    } else {
                        return Some(Token::from_str(")"));
                    }
                }
                ' ' => {
                    return Some(Token::from_str(cache.drain(..).collect::<String>()));
                }
                c => cache.push(c),
            }
        }

        if cache.len() > 0 {
            Some(Token::from_str(cache.drain(..).collect::<String>()))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (3, "(add 1 2)"),
            (15, "(mult 3 (add 2 3))"),
            (10, "(let x 2 (mult x 5))"),
            (14, "(let x 2 (mult x (let x 3 y 4 (add x y))))"),
            (2, "(let x 3 x 2 x)"),
            (5, "(let x 1 y 2 x (add x y) (add x y))"),
            (6, "(let x 2 (add (let x 3 (let x 4 x)) x))"),
            (4, "(let a1 3 b2 (add a1 1) b2)"),
            (14, "(let x 2 (mult (let x 3 y 4 (add x y)) x))"),
            (-128534112, "(let x0 -4 x1 2 x2 -4 x3 3 x4 2 x5 3 x6 2 x7 2 x8 -1 x9 -1 (mult (mult (mult x2 -8) (add -5 (let x0 1 x5 -3 (add (add x7 (add (let x0 -5 x9 -4 (add (mult 1 1) -10)) (mult -8 (mult x3 -5)))) (add (let x0 3 x8 -1 (let x0 -1 x9 1 (add x4 -6))) x9))))) (mult (add (mult (add (mult -6 (mult (add x1 x4) -4)) (let x0 -2 x7 4 (mult (mult (let x0 -3 (mult 1 1)) (add (mult 1 1) (mult 1 1))) (mult -5 (mult -9 (mult 1 1)))))) -10) x5) (mult (mult x5 -7) x8))))"),
        ];
        for (expect, arg) in cases {
            assert_eq!(expect, Solution::evaluate(arg.into()));
        }
    }
}
