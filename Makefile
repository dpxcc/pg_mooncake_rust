run:
	$(MAKE) -C pg_duckdb duckdb all-static-lib DUCKDB_BUILD=ReleaseStatic DUCKDB_GEN=make PG_CONFIG=~/.pgrx/17.4/pgrx-install/bin/pg_config
	cargo pgrx run
