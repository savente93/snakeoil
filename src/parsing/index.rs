use color_eyre::Result;
use rustpython_parser::{
    Mode,
    ast::{
        Arguments, Constant, Expr, ExprConstant, Identifier, Mod, Stmt, StmtAsyncFunctionDef,
        StmtClassDef, StmtExpr, StmtFunctionDef, TypeParam,
    },
    parse,
};
use std::{fs::File, io::Read, path::Path};

#[derive(Debug)]
pub struct FunctionDocumentation {
    name: Identifier,
    docstring: Option<String>,
    return_type: Option<Expr>,
    args: Arguments,
    generics: Vec<TypeParam>,
}

impl From<&StmtFunctionDef> for FunctionDocumentation {
    fn from(value: &StmtFunctionDef) -> Self {
        Self {
            name: value.name.clone(),
            docstring: extract_docstring_from_body(&value.body),
            return_type: value.returns.as_ref().map(|r| *r.clone()),
            args: *value.args.clone(),
            generics: value.type_params.clone(),
        }
    }
}

impl From<&StmtAsyncFunctionDef> for FunctionDocumentation {
    fn from(value: &StmtAsyncFunctionDef) -> Self {
        Self {
            name: value.name.clone(),
            docstring: extract_docstring_from_body(&value.body),
            return_type: value.returns.as_ref().map(|r| *r.clone()),
            args: *value.args.clone(),
            generics: value.type_params.clone(),
        }
    }
}

impl TryFrom<&Stmt> for FunctionDocumentation {
    type Error = ();

    fn try_from(value: &Stmt) -> std::result::Result<Self, Self::Error> {
        match value {
            Stmt::FunctionDef(stmt_function_def) => {
                Ok(FunctionDocumentation::from(stmt_function_def))
            }
            Stmt::AsyncFunctionDef(stmt_async_function_def) => {
                Ok(FunctionDocumentation::from(stmt_async_function_def))
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct ClassDocumentation {
    name: Identifier,
    docstring: Option<String>,
    methods: Vec<FunctionDocumentation>,
}

impl From<&StmtClassDef> for ClassDocumentation {
    fn from(value: &StmtClassDef) -> Self {
        Self {
            name: value.name.clone(),
            docstring: extract_docstring_from_body(&value.body),
            methods: value
                .body
                .iter()
                .filter_map(|s| FunctionDocumentation::try_from(s).ok())
                .collect(),
        }
    }
}

impl TryFrom<&Stmt> for ClassDocumentation {
    type Error = ();

    fn try_from(value: &Stmt) -> std::result::Result<Self, Self::Error> {
        match value {
            Stmt::ClassDef(stmt_class_def) => Ok(ClassDocumentation::from(stmt_class_def)),
            _ => Err(()),
        }
    }
}

#[derive(Default, Debug)]
pub struct ModuleDocumentation {
    docstring: Option<String>,
    functions: Vec<FunctionDocumentation>,
    classes: Vec<ClassDocumentation>,
}

pub fn parse_python_file(path: &Path) -> Result<Mod> {
    let mut file = File::open(path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    let program = parse_python_str(&file_content)?;
    Ok(program)
}

pub fn parse_python_str(content: &str) -> Result<Mod> {
    let parsed = parse(content, Mode::Module, "<embedded>");
    Ok(parsed?)
}

// just a conveneience function
fn extract_module_documentation(input_module: &Mod) -> ModuleDocumentation {
    if let Mod::Module(mod_module) = input_module {
        extract_documentation_from_statements(&mod_module.body)
    } else {
        ModuleDocumentation::default()
    }
}
fn extract_documentation_from_statements(statements: &[Stmt]) -> ModuleDocumentation {
    let mut free_functions = vec![];
    let mut class_definitions = vec![];
    let docstring = extract_docstring_from_body(statements);
    for statement in statements {
        if let Stmt::FunctionDef(stmt_function_def) = statement {
            free_functions.push(stmt_function_def.into());
        }
        if let Stmt::AsyncFunctionDef(stmt_async_function_def) = statement {
            free_functions.push(stmt_async_function_def.into());
        }
        if let Stmt::ClassDef(stmt_class_def) = statement {
            class_definitions.push(stmt_class_def.into());
        }
    }

    ModuleDocumentation {
        docstring,
        functions: free_functions,
        classes: class_definitions,
    }
}

fn extract_docstring_from_body(body: &[Stmt]) -> Option<String> {
    match body.first() {
        Some(Stmt::Expr(StmtExpr { range: _, value })) => {
            if let Expr::Constant(ExprConstant {
                range: _,
                value: Constant::Str(s),
                kind: _,
            }) = &**value
            {
                Some(s.clone())
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod test {

    use std::io::Write;

    use super::*;
    use assert_fs::prelude::*;

    fn test_python_func_no_types() -> &'static str {
        "
def is_odd(i):
    return bool(i & 1)
        "
    }
    fn test_python_async_func_no_types() -> &'static str {
        "
async def is_odd(i):
    return bool(i & 1)
        "
    }

    fn test_python_func_docstring() -> &'static str {
        "
def is_odd(i):
    '''
    Determine whether a number is odd.

    Returns
    -------
    bool: True iff input number is odd
    '''
    return bool(i & 1)
        "
    }

    fn test_python_lambda() -> &'static str {
        "
def is_odd(i):
    inner = lambda x: x % 2
    return inner(i)
        "
    }
    fn test_python_closure() -> &'static str {
        "
def is_odd(i):
    def inner_func(i):
        return bool(i&0)

    return not inner_func(i)
        "
    }
    fn test_python_no_func() -> &'static str {
        "
# this is a comment
a = 4
b = a + 6
assert b > 0
f = lambda a,b: [*a, *b]
        "
    }
    fn test_python_func_with_types() -> &'static str {
        "
def is_even(i: int) -> bool:
    return bool(i & 1)

        "
    }
    fn test_python_func_annotated() -> &'static str {
        "
def return_none(foo: str, bar, *args, unused: Dict[Any, str] = None) -> 4+9:
    return 22/7
        "
    }
    fn test_python_class() -> &'static str {
        r#"
class Greeter:
    '''
    this is a class docstring.

    '''

    class_var = "whatever"

    def greet(self):
        print("Hello, world!")
        def inner():
            print("this is a closure!")
        inner()
    "#
    }

    #[test]
    fn parse_doesnt_extract_lambda() -> Result<()> {
        let program = parse_python_str(test_python_lambda())?;
        let documentation = extract_module_documentation(&program);
        assert_eq!(documentation.functions.len(), 1);
        assert_eq!(documentation.classes.len(), 0);
        Ok(())
    }
    #[test]
    fn parse_test_python_async_func() -> Result<()> {
        let program = parse_python_str(test_python_async_func_no_types())?;
        let documentation = extract_module_documentation(&program);
        assert_eq!(documentation.functions.len(), 1);
        assert_eq!(documentation.classes.len(), 0);
        Ok(())
    }
    #[test]
    fn parse_doesnt_extract_closure() -> Result<()> {
        let program = parse_python_str(test_python_closure())?;
        let documentation = extract_module_documentation(&program);
        assert_eq!(documentation.functions.len(), 1);
        assert_eq!(documentation.classes.len(), 0);
        Ok(())
    }
    #[test]
    fn parse_test_python_func_no_types() -> Result<()> {
        let program = parse_python_str(test_python_func_no_types())?;
        let documentation = extract_module_documentation(&program);
        assert_eq!(documentation.functions.len(), 1);
        assert_eq!(documentation.classes.len(), 0);
        Ok(())
    }
    #[test]
    fn parse_test_python_no_func() -> Result<()> {
        let program = parse_python_str(test_python_no_func())?;
        let documentation = extract_module_documentation(&program);
        assert_eq!(documentation.functions.len(), 0);
        assert_eq!(documentation.classes.len(), 0);
        Ok(())
    }
    #[test]
    fn parse_test_python_func_dict_type() -> Result<()> {
        let program = parse_python_str(test_python_func_annotated())?;

        let documentation = extract_module_documentation(&program);
        assert_eq!(documentation.functions.len(), 1);
        assert_eq!(documentation.classes.len(), 0);
        Ok(())
    }
    #[test]
    fn parse_test_python_class() -> Result<()> {
        let program = parse_python_str(test_python_class())?;
        let documentation = extract_module_documentation(&program);
        assert_eq!(documentation.functions.len(), 0);
        assert_eq!(documentation.classes.len(), 1);

        // we checked before there is at least one class, so this is safe
        #[allow(clippy::unwrap_used)]
        let class = documentation.classes.first().unwrap();

        assert_eq!(class.methods.len(), 1);

        Ok(())
    }
    #[test]
    fn parse_empty_string() -> Result<()> {
        let program = parse_python_str("")?;
        let documentation = extract_module_documentation(&program);

        assert_eq!(documentation.docstring, None);
        assert_eq!(documentation.functions.len(), 0);
        assert_eq!(documentation.classes.len(), 0);

        Ok(())
    }
    #[test]
    fn parse_test_python_func_with_types() -> Result<()> {
        let program = parse_python_str(test_python_func_with_types())?;

        let documentation = extract_module_documentation(&program);
        assert_eq!(documentation.functions.len(), 1);
        assert_eq!(documentation.classes.len(), 0);
        Ok(())
    }
    #[test]
    fn parse_test_python_func_docstring() -> Result<()> {
        let program = parse_python_str(test_python_func_docstring())?;
        let documentation = extract_module_documentation(&program);
        assert_eq!(documentation.functions.len(), 1);
        assert_eq!(documentation.classes.len(), 0);
        Ok(())
    }
    #[test]
    fn parse_test_python_class_docstring() -> Result<()> {
        let program = parse_python_str(test_python_class())?;

        let documentation = extract_module_documentation(&program);

        // we checked before there is at least one class, so this is safe
        #[allow(clippy::unwrap_used)]
        let docstring = documentation.classes.first().unwrap().docstring.clone();

        assert_eq!(
            docstring,
            Some(String::from(
                r"
    this is a class docstring.

    "
            ))
        );
        Ok(())
    }
    #[test]
    fn parse_test_python_function_docstring() -> Result<()> {
        let program = parse_python_str(test_python_func_docstring())?;

        let documentation = extract_module_documentation(&program);
        // we checked before there is at least one class, so this is safe
        #[allow(clippy::unwrap_used)]
        let function = documentation.functions.first().unwrap();
        let docstring = function.docstring.clone();
        assert_eq!(
            docstring,
            Some(String::from(
                r"
    Determine whether a number is odd.

    Returns
    -------
    bool: True iff input number is odd
    "
            ))
        );
        Ok(())
    }

    #[test]
    fn parse_test_python_file_on_disk() -> Result<()> {
        let file_contents = test_python_class();

        let temp_dir = assert_fs::TempDir::new()?;
        let child = temp_dir.child("foo.py");
        child.touch()?;
        let root_pkg_path = child.path();
        let mut file = File::create(root_pkg_path)?;
        file.write_all(file_contents.as_bytes())?;

        let program = parse_python_file(root_pkg_path)?;
        let docs = extract_module_documentation(&program);

        assert_eq!(docs.docstring, None);
        assert_eq!(docs.functions.len(), 0);
        assert_eq!(docs.classes.len(), 1);

        Ok(())
    }

    #[test]
    fn test_doc_extraction_interactive_module() -> Result<()> {
        let expr = parse("1 + 2", Mode::Expression, "<embedded>")?;
        let docs = extract_module_documentation(&expr);

        assert_eq!(docs.docstring, None);
        assert_eq!(docs.functions.len(), 0);
        assert_eq!(docs.classes.len(), 0);

        Ok(())
    }
}
