// suppress for the whole module with inner attribute...
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ValeCode {
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct Argument {
    // #[serde(rename(deserialize = "argumentIndex"))]
    pub argumentIndex: u32,
    // #[serde(rename(deserialize = "resultType"))]
    pub resultType: Ref,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    // #[serde(rename(deserialize = "innerExpr"))]
    pub innerExpr: SourceExpr,
    // #[serde(rename(deserialize = "innerType"))]
    pub innerType: Ref,
}

#[derive(Debug, Deserialize)]
pub struct Call {
    // #[serde(rename(deserialize = "argExprs"))]
    pub argExprs: Vec<SourceExpr>,
    pub function: Prototype,
}

#[derive(Debug, Deserialize)]
pub struct Consecutor {
    pub exprs: Vec<SourceExpr>,
}

#[derive(Debug, Deserialize)]
pub struct ConstantI64 {
    pub value: i64,
}

#[derive(Debug, Deserialize)]
pub struct ConstantStr {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Discard {
    // #[serde(rename(deserialize = "sourceExpr"))]
    sourceExpr: Unstackify,
    // #[serde(rename(deserialize = "sourceResultType"))]
    sourceResultType: Ref,
}

#[derive(Debug, Deserialize)]
pub struct ExternCall {
    // #[serde(rename(deserialize = "argExprs"))]
    argExprs: Vec<Argument>,
    // #[serde(rename(deserialize = "argTypes"))]
    argTypes: Vec<Ref>,
    function: Prototype,
}

#[derive(Debug, Deserialize)]
pub struct Function {
    pub block: FunctionBlock,
    pub export: bool,
    pub prototype: Prototype,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "__type")]
pub enum FunctionBlock {
    Block(Block),
    Return(Return),
}

#[derive(Debug, Deserialize)]
pub struct Local {
    #[serde(rename(deserialize = "id"))]
    pub local_id: VariableId,
    #[serde(rename(deserialize = "type"))]
    pub local_type: Ref,
}

#[derive(Debug, Deserialize)]
pub struct LocalLoad {
    pub local: Local,
    // #[serde(rename(deserialize = "localName"))]
    pub localName: String,
    // #[serde(rename(deserialize = "targetOwnership"))]
    pub targetOwnership: Ownership,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "__type")]
pub enum Location {
    Inline,
    Yonder,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "__type")]
pub enum Mutability {
    Mutable,
    Immutable,
}

#[derive(Debug, Deserialize)]
pub struct NewStruct {
    // #[serde(rename(deserialize = "memberNames"))]
    pub memberNames: Vec<String>,
    // #[serde(rename(deserialize = "resultType"))]
    pub resultType: Ref,
    // #[serde(rename(deserialize = "sourceExprs"))]
    pub sourceExprs: Vec<SourceExpr>,
}

#[derive(Debug, Deserialize)]
pub struct OptSome {
    pub value: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "__type")]
pub enum Ownership {
    Share,
    Owning,
    Constraint,
}

#[derive(Debug, Deserialize)]
pub struct ExportedNameByFullname;
#[derive(Debug, Deserialize)]
pub struct Externs;
#[derive(Debug, Deserialize)]
pub struct ImmDestructorsByReferend;
#[derive(Debug, Deserialize)]
pub struct Interfaces;

#[derive(Debug, Deserialize)]
pub struct Program {
    // #[serde(rename(deserialize = "exportedNameByFullName"))]
    pub exportedNameByFullName: Vec<String>,
    pub externs: Vec<String>,
    pub functions: Vec<Function>,
    // #[serde(rename(deserialize = "immDestructorsByReferend"))]
    pub immDestructorsByReferend: Vec<String>,
    pub interfaces: Vec<String>,
    // // #[serde(rename(deserialize = "knownSizeArrays"))]
    pub knownSizeArrays: Vec<String>,
    pub structs: Vec<Struct>,
    // #[serde(rename(deserialize = "unknownSizeArrays"))]
    pub unknownSizeArrays: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Prototype {
    pub name: String,
    pub params: Vec<Ref>,
    #[serde(rename(deserialize = "return"))]
    pub return_type: Ref,
}

#[derive(Debug, Deserialize)]
pub struct Ref {
    pub location: Location,
    pub ownership: Ownership,
    pub referend: Referend,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "__type")]
pub enum Referend {
    Int,
    Never,
    Str,
    StructId,
}

#[derive(Debug, Deserialize)]
pub struct Return {
    // #[serde(rename(deserialize = "sourceExpr"))]
    pub sourceExpr: Box<SourceExpr>,
    // #[serde(rename(deserialize = "sourceType"))]
    pub sourceType: Ref,
}

#[derive(Debug, Deserialize)]
pub struct Edges;
#[derive(Debug, Deserialize)]
pub struct Members;

#[derive(Debug, Deserialize)]
pub struct Struct {
    pub edges: Vec<Edges>,
    pub export: bool,
    pub members: Vec<Members>,
    pub mutability: Mutability,
    pub name: String,
    pub weakable: bool,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "__type")]
pub enum SourceExpr {
    Argument(Argument),
    Call(Call),
    Consecutor(Consecutor),
    ConstantStr(ConstantStr),
    ConstantI64(ConstantI64),
    Discard(Discard),
    ExternCall(ExternCall),
    LocalLoad(LocalLoad),
    NewStruct(NewStruct),
    Return(Return),
    Stackify(Stackify),
    Unstackify(Unstackify),
}

#[derive(Debug, Deserialize)]
pub struct Stackify {
    pub local: Local,
    // #[serde(rename(deserialize = "optName"))]
    pub optName: OptSome,
    // #[serde(rename(deserialize = "sourceExpr"))]
    pub sourceExpr: Box<SourceExpr>,
}

#[derive(Debug, Deserialize)]
pub struct Unstackify {
    local: Local,
}

#[derive(Debug, Deserialize)]
pub struct VariableId {
    pub number: u32,
    // #[serde(rename(deserialize = "optName"))]
    pub optName: OptSome,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "__type")]
pub enum Ast {
    Program(Program),
}
