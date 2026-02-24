#[test]
fn test_simple_function_parsing() {
  use crate::parser::{self, Parser};

  let input = r#"
  function add(a, b) {
    #0("", "Hello, World!");
  }"#;

  let syntax_tree = super::Parser.parse_program_from_str(input).unwrap();

  let expected_syntax_tree = parser::Program {
    link_statements: vec![],
    main_block: vec![
      parser::Item::FunctionDeclaration {
        name: "add".to_string(),
        parameters: vec![
          parser::Expression::Identifier("a".to_string()),
          parser::Expression::Identifier("b".to_string()),
        ],
        body: vec![
          parser::Statement::Expression {
            expr: parser::Expression::Call {
                function: Box::new(parser::Expression::CWScriptBlockID("#0".to_string())),
              arguments: vec![
                parser::Expression::BinOperation {
                  lhs: Box::new(parser::Expression::Literal(parser::Literal::String("".to_string()))),
                  op: parser::BinOperator::Comma,
                  rhs: Box::new(parser::Expression::Literal(parser::Literal::String("Hello, World!".to_string()))),
                }
              ]
            }
          }
        ]
      }
    ]
  };

  assert_eq!(syntax_tree, expected_syntax_tree);
}