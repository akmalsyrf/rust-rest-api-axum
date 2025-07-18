dev:
	RUST_LOG=debug cargo watch -c -x run
migrate_create:
	sqlx migrate add $(ARGS)
	# example uses: make migrate_create ARGS=create_user_table
