pub mod codegen {
    pub mod auth;
    pub mod forum;
}

pub mod auth;
pub mod db;
pub mod forum;
pub mod middleware;
use db::models as dbmodels;
use db::schema as dbschema;
