use anyhow::{self, Error};

use pest::{Parser as _Parser, Span, iterators::Pairs, iterators::Pair, pratt_parser::PrattParser};
use pest_derive::Parser as _Parser;

mod test;

// TODO: Carry source code span in the AST for better error reporting in the future.

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
  pub link_statements: Vec<Statement>,
  pub main_block: Vec<Item>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
  FunctionDeclaration {
    name: String,
    parameters: Vec<Expression>,
    body: Vec<Statement>,
  },
  Attribute {
    name: String,
    content: Option<String>,
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
  Expression {
    expr: Expression,
  },
  Assignment {
    lhs: Expression,
    rhs: Expression,
  },
  Link { 
    path: String 
  },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
  Integer(String),
  Float(String),
  String(String),
  Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
  // TODO: Implement other expression types
  Literal(Literal),
  CWScriptBlockID(String),
  Identifier(String),
  BinOperation {
    lhs: Box<Expression>,
    op: BinOperator,
    rhs: Box<Expression>,
  },
  UnaryOperation {
    op: UnaryOperator,
    expr: Box<Expression>,
  },
  Call {
    // Identifier or CWScriptBlockID
    function: Box<Expression>,
    arguments: Vec<Expression>,
  },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinOperator {
  Addition,
  Subtraction,
  Multiplication,
  Division,
  Power,
  Dot,
  Comma
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
  NumeralNegation,
  LogicalNegation,
  CallExpression
}

// Defines the associativity and precedence of operators
lazy_static::lazy_static! {
  static ref PRATT_PARSER: PrattParser<Rule> = {
    use pest::pratt_parser::{Assoc::*, Op};
    use Rule::*;

    // Precedence is defined lowest to highest
    PrattParser::new()
      // Addition and subtract have equal precedence
      .op(Op::infix(Comma, Left))
      .op(Op::infix(Dot, Left))
      .op(Op::infix(Addition, Left) | Op::infix(Subtraction, Left))
      .op(Op::infix(Multiplication, Left) | Op::infix(Division, Left))
      .op(Op::infix(Power, Left))

      .op(Op::prefix(NumeralNegation) | Op::prefix(LogicalNegation))

      .op(Op::postfix(CallExpression))
  };
}

#[derive(_Parser)]
#[grammar = "grammar.pest"] // relative to src
pub struct Parser;

impl Parser {
  pub fn new() -> Self {
    Parser {}
  }

  pub fn parse_rule<'a>(self: &Parser, rule: Rule, input: &'a str) -> Result<Pairs<'a, Rule>, anyhow::Error> {
    // Wrapper for the generated parser. We return anyhow::Error for better error handling in the future.
    Ok(Parser::parse(rule, input)?)
  }

  pub fn parse_program_from_str(self: &mut Parser, input: &str) -> Result<Program, anyhow::Error> {
    Ok(self.parse_program(self.parse_rule(Rule::program, input)?.next().expect("Program should match once"))?)
  }

  // Parses the entire program. Only accept a single `program` pair.
  pub fn parse_program(self: &mut Parser, input: Pair<Rule>) -> Result<Program, anyhow::Error> {
    let mut input_iter: Pairs<Rule> = input.into_inner();
    let program_header = input_iter.next().unwrap().into_inner().collect::<Vec<Pair<Rule>>>();
    let mut program_body = input_iter.collect::<Vec<Pair<Rule>>>();
    program_body.pop().expect("EOF at the end of program body should not be empty"); // Remove EOF
    Ok(
      Program {
        // TODO: Implement link logic
        link_statements: vec![],
        main_block: program_body.into_iter()
          .filter_map(|pair| self.parse_item(pair).ok().flatten())
          .collect::<Vec<Item>>(),
      }
    )
  }

  /// Parses an item, which can be either an attribute or a function declaration.
  /// 
  /// - If it's an attribute, we push it to the state and return None
  /// - If it's a function declaration, we return the parsed item
  fn parse_item(self: &mut Parser, input: Pair<Rule>) -> Result<Option<Item>, anyhow::Error> {
    match input.as_rule() {
      Rule::FunctionDeclaration => Ok(Some(Item::FunctionDeclaration(self.parse_function_declaration(input)?))),
      Rule::Attribute => Ok(Some(Item::Attribute(self.parse_attribute(input)?))),
      rule => unreachable!("Expected item, found {:?}", rule),
    }
  }

  fn parse_attribute(self: &Parser, input: Pair<Rule>) -> Result<Attribute, anyhow::Error> {
    match input.as_rule() {
      Rule::Attribute => {
        let mut input_iter = input.into_inner();
        let attribute_name = input_iter.find_first_tagged("attr_name").expect("There should be attribute name in attribute").as_str().to_string();
        let attribute_content = if let Some(content_pair) = input_iter.find_first_tagged("attr_content") {
          Some(content_pair.as_str().trim_matches('"').to_string())
        } else {
          None
        };
        match attribute_name.as_str() {
          "inline" => Ok(Attribute::Inline),
          "export_as" => Ok(Attribute::ExportAs(attribute_content.unwrap_or_else(|| attribute_name))),
          attr_str => Err(anyhow::anyhow!("Unknown attribute: {}", attr_str)),
          }
      },
      rule => unreachable!("Expected attribute, found {:?}", rule),
    }
  }

  fn parse_function_declaration(self: &Parser, input: Pair<Rule>) -> Result<FunctionDeclaration, anyhow::Error> {
    match input.as_rule() {
      Rule::FunctionDeclaration => {
        let mut input_iter: Pairs<Rule> = input.into_inner();
        let function_name = input_iter.next().expect("Function name should not be empty").as_str().to_string();
        
        // The last pair is the block. We pop it from the back.
        let body_pair = input_iter.next_back().expect("Function body should not be empty");
        let parsed_body = self.parse_block(body_pair.into_inner());

        // The remaining pair in the middle (if any) is the parameters expression
        let expanded_parameters = if let Some(params_pair) = input_iter.next() {
           let param_expr = self.parse_expression(params_pair);
           self.expand_comma_expression(param_expr)?
        } else {
           vec![]
        };

        Ok(
          Item::FunctionDeclaration {
            name: function_name,
            parameters: expanded_parameters,
            body: parsed_body,
          }
        )
      },
      rule => unreachable!("Expected function declaration, found {:?}", rule),
    }
  }

  fn parse_block(self: &Parser, input: Pairs<Rule>) -> Vec<Statement> {
    input.map(|pair| self.parse_statement(pair)).collect()
  }

  fn parse_statement(self: &Parser, input: Pair<Rule>) -> Statement {
    let statement = input;
    match statement.as_rule() {
      Rule::ExpressionStatement => {
        Statement::Expression {
          expr: self.parse_expression(statement.into_inner().next().unwrap())
        }
      }
      
      Rule::AssignmentStatement => {
        let inner_statement = statement.into_inner().next().unwrap();
        match inner_statement.as_rule() {
          Rule::LetStatement | Rule::ReassignmentStatement => {
            let mut inner_statement_iter = inner_statement.into_inner();
            let lhs_expr = inner_statement_iter.next().expect("LHS of assignment should not be empty");
            let rhs_expr = inner_statement_iter.next().expect("RHS of assignment should not be empty");
            Statement::Assignment {
              lhs: self.parse_singlet(lhs_expr),
              rhs: self.parse_expression(rhs_expr),
            }
          },
          rule => unreachable!("Expected assignment statement, found {:?}", rule),
        }
      },
      // TODO: Implement link statement parsing
      Rule::LinkStatement => todo!(),
      rule => unreachable!("Expected statement, found {:?}", rule),
    }
  }

  fn parse_singlet(self: &Parser, input: Pair<Rule>) -> Expression {
    match input.as_rule() {
      Rule::Expression => self.parse_expression(input),
      Rule::CWScriptBlockID => Expression::CWScriptBlockID(input.as_str().to_string()),
      Rule::Identifier => Expression::Identifier(input.as_str().to_string()),
      Rule::string_literal => Expression::Literal(Literal::String(input.as_str().trim_matches('"').to_string())),
      Rule::float_literal => Expression::Literal(Literal::Float(input.as_str().to_string())),
      Rule::number_literal => Expression::Literal(Literal::Integer(input.as_str().to_string())),
      Rule::boolean_literal => Expression::Literal(Literal::Bool(self.parse_boolean_literal(input).unwrap())),
      rule => unreachable!("Expected singlet expression, found {:?}", rule),
    }
  }

  fn parse_expression(self: &Parser, input: Pair<Rule>) -> Expression {
    PRATT_PARSER
      .map_primary(|primary| match primary.as_rule() {
        Rule::CWScriptBlockID => Expression::CWScriptBlockID(primary.as_str().to_string()),
        Rule::Identifier => Expression::Identifier(primary.as_str().to_string()),
        Rule::string_literal => Expression::Literal(Literal::String(primary.as_str().trim_matches('"').to_string())),
        Rule::float_literal => Expression::Literal(Literal::Float(primary.as_str().to_string())),
        Rule::number_literal => Expression::Literal(Literal::Integer(primary.as_str().to_string())),
        Rule::boolean_literal => Expression::Literal(Literal::Bool(self.parse_boolean_literal(primary).unwrap())),
        Rule::Expression => self.parse_expression(primary),
        rule => unreachable!("Expected expressions, found {:?}", rule)
      })
      .map_infix(|lhs, op, rhs| {
        let op = match op.as_rule() {
          Rule::Addition => BinOperator::Addition,
          Rule::Subtraction => BinOperator::Subtraction,
          Rule::Multiplication => BinOperator::Multiplication,
          Rule::Division => BinOperator::Division,
          Rule::Power => BinOperator::Power,
          Rule::Dot => BinOperator::Dot,
          Rule::Comma => BinOperator::Comma,
          rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
        };
        Expression::BinOperation {
          lhs: Box::new(lhs),
          op,
          rhs: Box::new(rhs),
        }
      })
      .map_prefix(|prefix, expr| {
        let op = match prefix.as_rule() {
          Rule::NumeralNegation => UnaryOperator::NumeralNegation,
          Rule::LogicalNegation => UnaryOperator::LogicalNegation,
          rule => unreachable!("Expr::parse expected prefix operation, found {:?}", rule),
        };
        Expression::UnaryOperation {
          op,
          expr: Box::new(expr),
        }
      })
      .map_postfix(|expr, postfix| {
        match postfix.as_rule() {
          Rule::CallExpression => {
            let postfix_inner_iter = postfix.clone().into_inner();
            Expression::Call {
              function: Box::new(expr), 
              arguments: if postfix_inner_iter.len() > 0 {
                postfix_inner_iter.map(|arg_pair| self.parse_expression(arg_pair)).collect::<Vec<Expression>>() 
              } else { vec![] }
            }
          },
          rule => unreachable!("Expr::parse expected postfix operation, found {:?}", rule),
        }
      })
      .parse(input.into_inner())
  }

  pub fn parse_boolean_literal(self: &Parser, input: Pair<Rule>) -> Result<bool, anyhow::Error> {
    let pair = input.into_inner().next().unwrap();
    match pair.as_rule() {
      Rule::true_literal => Ok(true),
      Rule::false_literal => Ok(false),
      _ => Err(anyhow::anyhow!("Expected boolean literal, found {:?}", pair.as_rule())),
    }
  }

  fn expand_comma_expression(self: &Parser, input: Expression) -> Result<Vec<Expression>, anyhow::Error> {
    match input {
      Expression::BinOperation { lhs, op: BinOperator::Comma, rhs } => {
        let mut result = self.expand_comma_expression(*lhs)?;
        result.push(*rhs);
        Ok(result)
      },
      expr => Ok(vec![expr]),
    }
  }
}