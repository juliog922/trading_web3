build:
	@ docker compose build

up:
	@ docker compose up -d

down:
	@ docker compose down

migration:
	docker-compose exec backend diesel migration run

redo:
	docker-compose exec backend diesel migration redo

launch:
	docker compose exec backend cargo run

test_crypto: 
	docker-compose exec crypto_service /bin/bash -c "cargo test"

test_backend: 
	docker-compose exec backend /bin/bash -c "cargo test"

ping:
	@ docker compose exec postgres psql -U postgres -d backend_db -c "SELECT * FROM users;"