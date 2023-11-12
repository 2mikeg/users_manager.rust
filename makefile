run:
	source .env && cargo run

#init a new migration file
new_mig:
	sqlx migrate add -r init

up_mig:
	source .env && sqlx migrate run