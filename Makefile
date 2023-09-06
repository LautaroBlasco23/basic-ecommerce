# ------------------------
# DOCKER COMPOSE
# ------------------------

start-project:
	docker-compose -d up

stop-project:
	docker-compose down

# ------------------------
# DATABASE MIGRATIONS
# ------------------------

create-tables-in-db:
	cd db/ && sqlx migrate run

delete-tables-in-db:
	cd db/ && sqlx migrate revert
