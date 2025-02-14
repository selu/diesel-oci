use super::backend::Oracle;
use super::backend::OracleDualForEmptySelectClause;

use diesel::query_builder::NoFromClause;
use diesel::query_builder::QueryBuilder;
use diesel::query_builder::QueryFragment;
use diesel::result::Error as DieselError;

mod alias;
mod exists;
mod limit_offset;
mod returning;

pub use self::alias::Alias;

/// The Oracle query builder
#[derive(Default)]
pub struct OciQueryBuilder {
    pub(crate) sql: String,
    bind_idx: u32,
}

impl OciQueryBuilder {
    /// Constructs a new query builder with an empty query
    pub fn new() -> Self {
        OciQueryBuilder {
            sql: String::new(),
            bind_idx: 0,
        }
    }
}

impl QueryBuilder<Oracle> for OciQueryBuilder {
    fn push_sql(&mut self, sql: &str) {
        self.sql.push_str(sql);
    }

    fn push_identifier(&mut self, identifier: &str) -> Result<(), DieselError> {
        // TODO: check if there is a better way for escaping strings
        self.push_sql("\"");
        self.push_sql(&identifier.replace('`', "``").to_uppercase());
        self.push_sql("\"");
        Ok(())
    }

    fn push_bind_param(&mut self) {
        let sql = format!(":in{}", self.bind_idx);
        self.bind_idx += 1;
        self.push_sql(&sql);
    }

    fn finish(self) -> String {
        self.sql
    }
}

impl QueryFragment<Oracle, OracleDualForEmptySelectClause> for NoFromClause {
    fn walk_ast(&self, mut out: diesel::query_builder::AstPass<Oracle>) -> diesel::QueryResult<()> {
        out.push_sql(" FROM DUAL ");
        Ok(())
    }
}
