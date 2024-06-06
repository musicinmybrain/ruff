use crate::module::ModuleName;
use crate::salsa_db::semantic::ast_ids::{
    AnnotatedAssignmentId, AssignmentId, ClassId, FunctionId,
};

use crate::Name;

// TODO: I think we should instead reference the node for `ImportDefinition` and `ImportFromDefinition` too
// or it will be impossible to render nice diagnostics in an error message.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Definition {
    // For the import cases, we don't need reference to any arbitrary AST subtrees (annotations,
    // RHS), and referencing just the import statement node is imprecise (a single import statement
    // can assign many symbols, we'd have to re-search for the one we care about), so we just copy
    // the small amount of information we need from the AST.
    Import(ImportDefinition),
    ImportFrom(ImportFromDefinition),
    Function(FunctionId),
    Class(ClassId),
    Assignment(AssignmentId),
    AnnotatedAssignment(AnnotatedAssignmentId),
}

impl From<ImportDefinition> for Definition {
    fn from(value: ImportDefinition) -> Self {
        Definition::Import(value)
    }
}

impl From<ImportFromDefinition> for Definition {
    fn from(value: ImportFromDefinition) -> Self {
        Definition::ImportFrom(value)
    }
}

impl From<FunctionId> for Definition {
    fn from(value: FunctionId) -> Self {
        Definition::Function(value)
    }
}

impl From<ClassId> for Definition {
    fn from(value: ClassId) -> Self {
        Definition::Class(value)
    }
}

impl From<AssignmentId> for Definition {
    fn from(value: AssignmentId) -> Self {
        Definition::Assignment(value)
    }
}

impl From<AnnotatedAssignmentId> for Definition {
    fn from(value: AnnotatedAssignmentId) -> Self {
        Definition::AnnotatedAssignment(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportDefinition {
    pub module: ModuleName,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportFromDefinition {
    pub module: Option<ModuleName>,
    pub name: Name,
    pub level: u32,
}

impl ImportFromDefinition {
    pub fn module(&self) -> Option<&ModuleName> {
        self.module.as_ref()
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn level(&self) -> u32 {
        self.level
    }
}