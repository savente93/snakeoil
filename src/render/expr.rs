use rustpython_parser::ast::{Comprehension, Constant, Expr, Operator};

pub fn render_expr(expr: Expr) -> String {
    let mut out = String::new();

    #[allow(unused_variables)]
    match expr {
        Expr::BoolOp(expr_bool_op) => {
            out.push_str(match expr_bool_op.op {
                rustpython_parser::ast::BoolOp::And => " And ",
                rustpython_parser::ast::BoolOp::Or => " Or ",
            });
            out.push_str(
                &expr_bool_op
                    .values
                    .into_iter()
                    .map(render_expr)
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        Expr::NamedExpr(expr_named_expr) => todo!(),
        Expr::BinOp(expr_bin_op) => {
            out.push_str(&render_expr(*expr_bin_op.left));
            out.push_str(render_operator(expr_bin_op.op));
            out.push_str(&render_expr(*expr_bin_op.right));
        }
        Expr::UnaryOp(expr_unary_op) => todo!(),
        Expr::Lambda(expr_lambda) => todo!(),
        Expr::IfExp(expr_if_exp) => todo!(),
        Expr::Dict(expr_dict) => {
            out.push('{');
            out.push_str(
                &expr_dict
                    .keys
                    .iter()
                    .zip(expr_dict.values)
                    .map({
                        |(k, v)| {
                            if let Some(key) = k {
                                format!("{}: {}", render_expr(key.clone()), render_expr(v))
                            } else {
                                format!("**{}", render_expr(v))
                            }
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            out.push('}');
        }
        Expr::Set(expr_set) => {
            out.push('{');
            out.push_str(
                &expr_set
                    .elts
                    .into_iter()
                    .map(render_expr)
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            out.push('}');
        }
        Expr::ListComp(expr_list_comp) => {
            out.push('[');
            out.push_str(&render_expr(*expr_list_comp.elt));
            out.push(' ');
            out.push_str(
                &expr_list_comp
                    .generators
                    .into_iter()
                    .map(render_comprehension)
                    .collect::<Vec<_>>()
                    .join(" "),
            );
            out.push(']');
        }
        Expr::SetComp(expr_set_comp) => todo!(),
        Expr::DictComp(expr_dict_comp) => todo!(),
        Expr::GeneratorExp(expr_generator_exp) => todo!(),
        Expr::Await(expr_await) => todo!(),
        Expr::Yield(expr_yield) => todo!(),
        Expr::YieldFrom(expr_yield_from) => todo!(),
        Expr::Compare(expr_compare) => todo!(),
        Expr::Call(expr_call) => todo!(),
        Expr::FormattedValue(expr_formatted_value) => todo!(),
        Expr::JoinedStr(expr_joined_str) => todo!(),
        Expr::Constant(expr_constant) => out.push_str(&render_constant(expr_constant.value)),
        Expr::Attribute(expr_attribute) => todo!(),
        Expr::Subscript(expr_subscript) => todo!(),
        Expr::Starred(expr_starred) => todo!(),
        Expr::Name(expr_name) => out.push_str(expr_name.id.as_str()),
        Expr::List(expr_list) => {
            out.push('[');
            out.push_str(
                &expr_list
                    .elts
                    .into_iter()
                    .map(render_expr)
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            out.push(']');
        }
        Expr::Tuple(expr_tuple) => {
            out.push('(');
            out.push_str(
                &expr_tuple
                    .elts
                    .into_iter()
                    .map(render_expr)
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            out.push(')');
        }
        Expr::Slice(expr_slice) => todo!(),
    }

    out
}

fn render_comprehension(comp: Comprehension) -> String {
    let mut out = String::new();
    out.push_str("for ");
    out.push_str(&render_expr(comp.target));
    out.push_str(" in ");
    out.push_str(&render_expr(comp.iter));

    out
}

fn render_constant(constant: Constant) -> String {
    match constant {
        Constant::None => String::from("None"),
        Constant::Bool(b) => {
            if b {
                String::from("True")
            } else {
                String::from("False")
            }
        }
        Constant::Str(s) => format!("\"{s}\""),
        Constant::Bytes(_) => todo!(),
        Constant::Int(big_int) => format!("{big_int}"),
        Constant::Tuple(constants) => format!(
            "({})",
            constants
                .into_iter()
                .map(render_constant)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Constant::Float(f) => format!("{f}"),
        Constant::Complex { real, imag } => {
            if real.abs() < f64::EPSILON {
                format!("{imag}j")
            } else if imag.abs() < f64::EPSILON {
                format!("{real}")
            } else {
                format!("{real}+{imag}j")
            }
        }
        Constant::Ellipsis => String::from("..."),
    }
}

fn render_operator(op: Operator) -> &'static str {
    match op {
        Operator::Add => "+",
        Operator::Sub => "-",
        Operator::Mult => "*",
        Operator::MatMult => "@",
        Operator::Div => "/",
        Operator::Mod => "%",
        Operator::Pow => "**",
        Operator::LShift => "<<",
        Operator::RShift => ">>",
        Operator::BitOr => "|",
        Operator::BitXor => "^",
        Operator::BitAnd => "&",
        Operator::FloorDiv => "//",
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use color_eyre::Result;
    use pretty_assertions::assert_eq;
    use rustpython_parser::{Mode, ast::Mod, parse};

    fn get_expr(s: &str) -> Result<Expr> {
        let parsed = parse(s, Mode::Expression, "<embedded>")?;
        if let Mod::Expression(mod_expr) = parsed {
            let expr = *mod_expr.body.clone();

            Ok(expr)
        } else {
            panic!()
        }
    }

    #[test]
    fn test_render_name() -> Result<()> {
        let s = "a";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }

    #[test]
    fn test_render_int() -> Result<()> {
        let s = "24";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_none() -> Result<()> {
        let s = "None";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_bool_true() -> Result<()> {
        let s = "True";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_bool_false() -> Result<()> {
        let s = "False";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_float() -> Result<()> {
        let s = "24.242424";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_ellipses() -> Result<()> {
        let s = "...";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_str() -> Result<()> {
        let s = "\"24\"";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_tuple() -> Result<()> {
        let s = "(24, 3)";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_binary_op_min() -> Result<()> {
        let s = "24-3";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_binary_op_div() -> Result<()> {
        let s = "24/3";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_empty_tuple() -> Result<()> {
        let s = "()";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_complex_tuple() -> Result<()> {
        let s = "(3+5j, True)";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_list() -> Result<()> {
        let s = "[24, 3]";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_empty_list() -> Result<()> {
        let s = "[]";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_complex_list() -> Result<()> {
        let s = "[3+5j, True]";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_set() -> Result<()> {
        let s = "{24, 3}";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_empty_dict() -> Result<()> {
        let s = "{}";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_complex_set() -> Result<()> {
        let s = "{3+5j, True}";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_complex() -> Result<()> {
        let s = "3+5j";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_list_comp() -> Result<()> {
        let s = "[a for a in b]";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
    #[test]
    fn test_render_dict() -> Result<()> {
        let s = "{a: 1, **d}";
        let expr = get_expr(s)?;

        let rendered = render_expr(expr);

        assert_eq!(rendered, s);

        Ok(())
    }
}
