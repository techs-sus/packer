use full_moon::{
    ast::{punctuated::Pair, Call, Expression, FunctionArgs, FunctionCall, Prefix, Suffix, Value},
    tokenizer::{StringLiteralQuoteType, Symbol, Token, TokenReference, TokenType},
    visitors::VisitorMut,
    ShortString,
};
use std::default::Default;

#[derive(Default)]
struct FunctionCallVisitor {}

fn visit_function_args(mut node: FunctionArgs) -> Option<String> {
    if let FunctionArgs::Parentheses {
        ref mut arguments, ..
    } = node
    {
        let file_path = arguments.pop();
        if let Option::Some(Pair::End(Expression::Value { ref value, .. })) = file_path {
            if let Value::String(token_reference) = &**value {
                let token = token_reference.token().token_type();
                if let TokenType::StringLiteral { literal, .. } = token {
                    return Some(literal.to_string());
                }
            }
        }
    }
    None
}

impl VisitorMut for FunctionCallVisitor {
    fn visit_function_call(&mut self, node: FunctionCall) -> FunctionCall {
        let prefix = node.prefix().to_string();
        if prefix == "import" || prefix == "nls_import" {
            println!("bundling fcall! Type: {}", prefix);
            let suffix = node.suffixes().cloned().collect::<Vec<Suffix>>().pop();
            if let Some(Suffix::Call(Call::AnonymousCall(function_args))) = suffix {
                let left_paren = Token::new(TokenType::Symbol {
                    symbol: Symbol::LeftParen,
                });
                let right_paren = Token::new(TokenType::Symbol {
                    symbol: Symbol::RightParen,
                });
                let file = visit_function_args(function_args).expect("Invalid import (no file)");
                let code: String =
                    String::from_utf8_lossy(&std::fs::read(file).expect("File not readable by OS"))
                        .to_string();
                if prefix == "import" {
                    return node
                        .with_prefix(Prefix::Name(TokenReference::new(
                            vec![
                                left_paren.clone(),
                                Token::new(TokenType::Symbol {
                                    symbol: Symbol::Function,
                                }),
                                left_paren.clone(),
                                right_paren.clone(),
                                Token::new(TokenType::Whitespace {
                                    characters: ShortString::new(" "),
                                }),
                            ],
                            Token::new(TokenType::Identifier {
                                identifier: ShortString::new(ye(&code)), // resolved code goes here
                            }),
                            vec![
                                Token::new(TokenType::Symbol {
                                    symbol: Symbol::End,
                                }),
                                right_paren.clone(),
                                left_paren,
                                right_paren,
                            ],
                        )))
                        .with_suffixes(vec![]);
                } else if prefix == "nls_import" {
                    return node
                        .with_prefix(Prefix::Name(TokenReference::new(
                            vec![
                                Token::new(TokenType::Identifier {
                                    identifier: ShortString::new("NLS"),
                                }),
                                left_paren,
                            ],
                            Token::new(TokenType::StringLiteral {
                                literal: ShortString::new(ye(&code)),
                                multi_line: Some(2),
                                quote_type: StringLiteralQuoteType::Brackets,
                            }),
                            vec![
                                Token::new(TokenType::Symbol {
                                    symbol: Symbol::Comma,
                                }),
                                Token::new(TokenType::Identifier {
                                    identifier: ShortString::new("owner.PlayerGui"),
                                }),
                                right_paren,
                            ],
                        )))
                        .with_suffixes(vec![]);
                };
            }
        }
        node
    }
}

fn ye(code: &str) -> std::string::String {
    let ast = full_moon::parse(code).expect("pls pass valid code");
    let mut visitor = FunctionCallVisitor::default();
    full_moon::print(&visitor.visit_ast(ast))
}

fn main() {
    println!("{}", ye("local x = 1; print(x); nls_import('hehe.lua')"))
}
