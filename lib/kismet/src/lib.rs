pub mod ast;
pub mod hir;
pub mod parser;
pub mod types;

pub use hir::compile;
pub use parser::parse;
