use pgrx::prelude::*;

::pgrx::pg_module_magic!();

extern "C" {
    fn init_pg_duckdb();
}

#[pg_guard]
pub extern "C-unwind" fn _PG_init() {
    unsafe { init_pg_duckdb() };
}
