//! Monomorphic IR that is very close to Viper and can be directly converted
//! into it.

pub mod ast;
pub mod cfg;
pub mod domain;
pub mod program;

pub use self::{
    ast::{
        expression::{self, BinaryOpKind, Expression, PermAmount, Trigger, UnaryOpKind},
        field::FieldDecl,
        function::FunctionDecl,
        position::Position,
        predicate::PredicateDecl,
        statement::Statement,
        ty::{self, Type},
        variable::VariableDecl,
    },
    cfg::{BasicBlock, Label, MethodDecl, ProcedureDecl, Successor},
    domain::{DomainAxiomDecl, DomainDecl, DomainFunctionDecl},
    program::Program,
};
