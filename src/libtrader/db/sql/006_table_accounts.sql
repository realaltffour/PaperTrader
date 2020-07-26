CREATE TABLE accounts_schema.accounts (
	id				BIGSERIAL PRIMARY KEY,
	username		TEXT UNIQUE NOT NULL,
	email			TEXT UNIQUE NOT NULL,
	is_pass			BOOLEAN NOT NULL,
	pass_hash		TEXT NOT NULL,
	transactions	transaction[]
)
