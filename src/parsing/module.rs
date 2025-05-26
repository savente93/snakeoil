use rustpython_parser::ast::{Mod, Stmt};

use super::{
    class::ClassDocumentation, function::FunctionDocumentation, utils::extract_docstring_from_body,
};

#[derive(Default, Debug)]
pub struct ModuleDocumentation {
    pub docstring: Option<String>,
    pub functions: Vec<FunctionDocumentation>,
    pub classes: Vec<ClassDocumentation>,
}
// just a conveneience function
pub fn extract_module_documentation(input_module: &Mod) -> ModuleDocumentation {
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

#[cfg(test)]
mod test {
    use super::*;
    use color_eyre::Result;
    use rustpython_parser::{Mode, parse};

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
