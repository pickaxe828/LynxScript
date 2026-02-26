#[test]
fn test_simple_function_compiling() {
  use crate::{codegen::structures, parser};

  let input = parser::Program {
    link_statements: vec![],
    main_block: vec![
      parser::Item::FunctionDeclaration(
        parser::FunctionDeclaration {         
          name: "add".to_string(),
          parameters: vec![
            parser::Expression::Identifier("a".to_string()),
            parser::Expression::Identifier("b".to_string()),
          ],
          body: vec![
            parser::Statement::Expression {
              expr: parser::Expression::Call {
                function: Box::new(parser::Expression::CWScriptBlockID("0".to_string())),
                arguments: vec![
                  parser::Expression::Literal(parser::Literal::String("".to_string())),
                  parser::Expression::Literal(parser::Literal::String("Hello, World!".to_string())),
                ]
              }
            }
          ] 
        }
      )
    ]
  };

  let mut compiler = super::Compiler::new(input);
  
  let structure_res = compiler.compile();

  let expected_structure = structures::Program {
    main_block: vec![
      structures::Item::FunctionDeclaration {
        name: "add".to_string(),
        parameters: vec![
          structures::Variable {
            name: "a".to_string(),
          },
          structures::Variable {
            name: "b".to_string(),
          },
        ],
        body: vec![
          structures::Statement {
            dependencies: vec![
              structures::Call::CWScriptBlockCall {
                dependencies: vec![],
                block_id: structures::CWScriptBlockID {
                  id: "0".to_string(),
                },
                arguments: vec![
                  structures::Argument::RawString(
                    "".to_string()
                  ),
                  structures::Argument::Literal(
                    structures::Literal {
                      value: "Hello, World!".to_string(),
                    },
                  ),
                    ],
                    return_var: None,
                  },
                ],
                content: vec![],
              },
          ],
      },
  ],
};

  assert_eq!(expected_structure, structure_res);
}