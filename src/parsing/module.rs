use std::path::PathBuf;

use rustpython_parser::ast::{Mod, Stmt};

use super::{
    class::{ClassDocumentation, is_private_class},
    function::{FunctionDocumentation, is_private_function},
    utils::extract_docstring_from_body,
};

#[derive(Default, Debug)]
pub struct ModuleDocumentation {
    pub name: Option<String>,
    pub prefix: Option<String>,
    pub docstring: Option<String>,
    pub functions: Vec<FunctionDocumentation>,
    pub classes: Vec<ClassDocumentation>,
    pub sub_modules: Vec<ModuleReference>,
}

#[derive(Debug)]
pub struct ModuleReference {
    pub name: String,
    pub path: PathBuf,
}
// just a conveneience function
pub fn extract_module_documentation(
    input_module: &Mod,
    name: Option<String>,
    prefix: Option<String>,
    skip_private: bool,
    skip_undoc: bool,
) -> ModuleDocumentation {
    if let Mod::Module(mod_module) = input_module {
        extract_documentation_from_statements(
            &mod_module.body,
            name,
            prefix,
            skip_private,
            skip_undoc,
        )
    } else {
        ModuleDocumentation::default()
    }
}
fn extract_documentation_from_statements(
    statements: &[Stmt],
    name: Option<String>,
    prefix: Option<String>,
    skip_private: bool,
    skip_undoc: bool,
) -> ModuleDocumentation {
    assert_ne!(name, Some(String::from("")));
    assert_ne!(prefix, Some(String::from("")));
    let mut free_functions = vec![];
    let mut class_definitions = vec![];
    let docstring = extract_docstring_from_body(statements);
    for statement in statements {
        if let Stmt::FunctionDef(stmt_function_def) = statement {
            let function_doc: FunctionDocumentation = stmt_function_def.into();
            if (function_doc.docstring.is_none() && skip_undoc)
                || (is_private_function(&function_doc) && skip_private)
            {
                continue;
            };
            free_functions.push(function_doc);
        }
        if let Stmt::AsyncFunctionDef(stmt_async_function_def) = statement {
            let function_doc: FunctionDocumentation = stmt_async_function_def.into();
            if (function_doc.docstring.is_none() && skip_undoc)
                || (is_private_function(&function_doc) && skip_private)
            {
                continue;
            };
            free_functions.push(function_doc);
        }
        if let Stmt::ClassDef(stmt_class_def) = statement {
            let class_doc: ClassDocumentation = stmt_class_def.into();
            if (class_doc.docstring.is_none() && skip_undoc)
                || (is_private_class(&class_doc) && skip_private)
            {
                continue;
            };
            class_definitions.push(class_doc);
        }
    }

    ModuleDocumentation {
        name,
        prefix,
        docstring,
        functions: free_functions,
        classes: class_definitions,
        sub_modules: vec![],
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use color_eyre::Result;
    use rustpython_parser::{Mode, parse};

    #[test]
    fn test_doc_extraction_interactive_module() -> Result<()> {
        let expr = parse("1 + 2", Mode::Expression, "<embedded>")?;
        let docs = extract_module_documentation(&expr, None, None, false, false);

        assert_eq!(docs.docstring, None);
        assert_eq!(docs.functions.len(), 0);
        assert_eq!(docs.classes.len(), 0);

        Ok(())
    }
}
