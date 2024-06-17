use pgrx::prelude::*;
use sha3::{Digest, Keccak256};
use hex;

pgrx::pg_module_magic!();

#[pg_extern]
fn hello_hello_world_pgrx(val: &str) -> String {
    let vote = val.to_owned();
    let mut hasher = Keccak256::new();
    hasher.update(vote.as_bytes());
    let result = hasher.finalize();
    let s = hex::encode(result);
    return format!("{}", s);
    // "Hello, hello_world_pgrx"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_hello_world_pgrx() {
        assert_eq!("Hello, hello_world_pgrx", crate::hello_hello_world_pgrx());
    }

}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
