build:
	@ docker compose build

up:
	@ docker compose up -d

down:
	@ docker compose down

test: 
	docker-compose exec crypto_service /bin/bash -c "cargo test"