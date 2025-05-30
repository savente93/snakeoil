use rustpython_parser::ast::{Arg, ArgWithDefault, Arguments};

use super::expr::render_expr;

pub(crate) fn render_args(args: Arguments) -> String {
    let mut out = String::new();

    out.push_str(
        &args
            .posonlyargs
            .into_iter()
            .map(render_arg_with_default)
            .collect::<Vec<_>>()
            .join(", "),
    );
    out.push_str(
        &args
            .args
            .into_iter()
            .map(render_arg_with_default)
            .collect::<Vec<_>>()
            .join(", "),
    );
    if let Some(varg) = &args.vararg {
        out.push_str(", *");
        out.push_str(&render_arg(*varg.clone()));
    }

    if !args.kwonlyargs.is_empty() {
        out.push_str(", ");
        out.push_str(
            &args
                .kwonlyargs
                .into_iter()
                .map(render_arg_with_default)
                .collect::<Vec<_>>()
                .join(", "),
        );
    }
    if let Some(kwarg) = args.kwarg {
        out.push_str(", **");
        out.push_str(&render_arg(*kwarg));
    }

    out
}
pub(crate) fn render_arg_with_default(arg: ArgWithDefault) -> String {
    let mut out = String::new();

    out.push_str(&render_arg(arg.def));
    if let Some(default) = arg.default {
        out.push_str(" = ");
        out.push_str(&render_expr(*default));
    }

    out
}
pub(crate) fn render_arg(arg: Arg) -> String {
    let mut out = String::new();
    out.push_str(arg.arg.as_ref());
    if let Some(annon) = arg.annotation {
        out.push_str(": ");
        out.push_str(&render_expr(*annon));
    }

    out
}
