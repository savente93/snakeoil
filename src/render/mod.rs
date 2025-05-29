pub mod args;
pub mod expr;

use args::render_args;
use expr::render_expr;

use crate::parsing::{
    class::ClassDocumentation, function::FunctionDocumentation, module::ModuleDocumentation,
};

pub fn render_module(mod_doc: ModuleDocumentation) -> String {
    let mut out = String::new();
    if mod_doc.name.is_some() | mod_doc.prefix.is_some() {
        out.push_str("# ");
    }
    if let Some(prefix) = &mod_doc.prefix {
        out.push_str(prefix);
    }
    if let Some(name) = &mod_doc.name {
        if mod_doc.prefix.is_some() {
            out.push('.');
        }
        out.push_str(name);
    }
    if mod_doc.name.is_some() | mod_doc.prefix.is_some() {
        out.push('\n');
    }

    if let Some(docstring) = &mod_doc.docstring {
        out.push('\n');
        out.push_str(docstring);
        out.push('\n');
    }

    for fn_docs in mod_doc.functions {
        out.push('\n');
        out.push_str(&render_function_docs(fn_docs, &mod_doc.prefix, 2));
    }

    for class_docs in mod_doc.classes {
        out.push_str(&render_class_docs(class_docs, &mod_doc.prefix, 2));
    }

    out
}

fn render_class_docs(
    class_docs: ClassDocumentation,
    prefix: &Option<String>,
    header_level: usize,
) -> String {
    let mut out = String::new();
    out.push('\n');
    let fully_qualified_class_name = if let Some(p) = prefix {
        format!("{}.{}", p, &class_docs.name)
    } else {
        format!("{}", &class_docs.name)
    };
    out.push_str(&"#".repeat(header_level));
    out.push(' ');

    out.push_str(&fully_qualified_class_name);
    out.push('\n');

    if let Some(docstring) = class_docs.docstring {
        let indent = detect_docstring_indent_prefix(&docstring);
        out.push_str(&docstring.replace(&indent, ""));
    }
    let method_prefix = if let Some(p) = prefix {
        Some(format!("{}.{}", p, class_docs.name))
    } else {
        Some(class_docs.name.to_string())
    };
    for fn_docs in class_docs.methods {
        out.push_str(&render_function_docs(fn_docs, &method_prefix, 3));
    }
    out
}

fn render_function_docs(
    fn_docs: FunctionDocumentation,
    prefix: &Option<String>,
    header_level: usize,
) -> String {
    let mut out = String::new();

    let fully_qualified_function_name = if let Some(p) = prefix {
        format!("{}.{}", p, &fn_docs.name)
    } else {
        fn_docs.name.to_string()
    };
    out.push_str(&"#".repeat(header_level));
    out.push(' ');

    out.push_str(&fully_qualified_function_name);
    out.push('\n');
    out.push('\n');
    out.push_str(&fn_docs.name);
    out.push('(');
    out.push_str(&render_args(fn_docs.args));
    out.push(')');
    if let Some(return_annotaiton) = fn_docs.return_type {
        out.push_str(&format!(" -> {}", render_expr(return_annotaiton)));
    }
    out.push('\n');

    if let Some(docstring) = fn_docs.docstring {
        out.push('\n');

        let indent = detect_docstring_indent_prefix(&docstring);
        out.push_str(docstring.replace(&indent, "").trim());
        out.push('\n');
    }
    out
}

/// Detects the common indentation prefix of a Python docstring.
/// Returns the leading whitespace (spaces/tabs) of the least-indented non-empty line after the first.
/// This handles both spaces and tabs without normalization.
fn detect_docstring_indent_prefix(docstring: &str) -> String {
    docstring
        .lines()
        .filter(|line| !line.trim().is_empty()) // skip lines that are fully empty or just whitespace
        .map(|line| {
            line.chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>()
        })
        .min_by_key(|prefix| prefix.len()) // get the shortest non-empty indent
        .unwrap_or_default()
}

#[cfg(test)]
mod test {

    use color_eyre::Result;
    use pretty_assertions::assert_eq;

    use crate::{
        parsing::{module::extract_module_documentation, utils::parse_python_str},
        render::render_module,
    };
    fn test_dirty_module_str() -> &'static str {
        r"'''This is a module that is used to test snakeoil.'''

from typing import Any

def foo(bar: int) -> Dict[str, Any]:
    '''this is a docstring for the foo function'''

    bar += 15
    bar << bar | 19
    return 0

class Greeter:
    '''
    this is a class docstring.

    '''

    class_var = 'whatever'

    def greet(self, name, *args, foo: str = 'bar', **kwargs) -> Callable[[], None]:
        '''
        Greet the world.

        Parameters
        ----------
        name: str
            just a parameter. it's actually used for anything

        Returns
        -------
        Callable[[], None]
            just a random closure to make the types interesting to render.
        '''
        print('Hello, world!')
        def inner():
            print('this is a closure!')
        inner()
        "
    }

    fn expected_module_docs_rendered() -> &'static str {
        r#"# snakeoil.testing.test_module

This is a module that is used to test snakeoil.

## snakeoil.testing.foo

foo(bar: int) -> Dict[str, Any]

this is a docstring for the foo function

## snakeoil.testing.Greeter

this is a class docstring.

### snakeoil.testing.Greeter.greet

greet(self, name, *args, foo: str="bar", **kwargs) -> Callable[[], None]

Greet the world.

Parameters
----------
name: str
    just a parameter. it's actually used for anything

Returns
-------
Callable[[], None]
    just a random closure to make the types interesting to render.
"#
    }

    #[test]
    fn render_module_documentation() -> Result<()> {
        let parsed = parse_python_str(test_dirty_module_str())?;
        let mod_documentation = extract_module_documentation(
            &parsed,
            Some(String::from("test_module")),
            Some(String::from("snakeoil.testing")),
        );

        let rendered = render_module(mod_documentation);

        assert_eq!(rendered, expected_module_docs_rendered());

        Ok(())
    }
    fn expected_module_docs_no_prefix_rendered() -> &'static str {
        r#"
This is a module that is used to test snakeoil.

## foo

foo(bar: int) -> Dict[str, Any]

this is a docstring for the foo function

## Greeter

this is a class docstring.

### Greeter.greet

greet(self, name, *args, foo: str="bar", **kwargs) -> Callable[[], None]

Greet the world.

Parameters
----------
name: str
    just a parameter. it's actually used for anything

Returns
-------
Callable[[], None]
    just a random closure to make the types interesting to render.
"#
    }

    #[test]
    fn render_module_documentation_no_prefix() -> Result<()> {
        let parsed = parse_python_str(test_dirty_module_str())?;
        let mod_documentation = extract_module_documentation(&parsed, None, None);

        let rendered = render_module(mod_documentation);

        assert_eq!(rendered, expected_module_docs_no_prefix_rendered());

        Ok(())
    }
}
