use oxc_allocator::Allocator;
use oxc_parser::{Parser, ParserReturn};
use oxc_span::SourceType;

pub fn extract_classname(tsx_content: &str) -> Vec<String> {
    // Memory arena where AST nodes get stored
    let allocator = Allocator::default();
    // Infers TypeScript + JSX + ESM modules
    let source_type = SourceType::from_path("Counter.tsx").unwrap();

    let ParserReturn {
        program,  // AST
        errors,   // Syntax errors
        panicked, // Parser encountered an error it couldn't recover from
        trivias,  // Comments, whitespace, etc.
    } = Parser::new(&allocator, tsx_content, source_type).parse();

    // クラス名を抽出
    let mut classnames = Vec::new();

    for node in program.body {
        match node {
            oxc_ast::ast::Statement::ExportNamedDeclaration(e) => {
                for d in &e.declaration {
                    match d {
                        oxc_ast::ast::Declaration::FunctionDeclaration(c) => {
                            c.body.iter().for_each(|b| {
                                for st in b.statements.iter() {
                                    match st {
                                        oxc_ast::ast::Statement::ReturnStatement(r) => {
                                            if let Some(oxc_ast::ast::Expression::JSXElement(e)) =
                                                &r.argument
                                            {
                                                for attr in e.opening_element.attributes.iter() {
                                                    println!("{:?}", attr);
                                                }
                                                // classnames.push(e.opening_element.attributes[0]);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            })
                        }
                        _ => {
                            // Handle other types of declarations
                        }
                    }
                }
            }
            _ => {
                // Handle other types of statements
            }
        }
    }

    classnames
}
