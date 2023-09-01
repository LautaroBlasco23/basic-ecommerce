# ----------------------------------------------------------
# START PROJECT USING DOCKER COMPOSE
# ----------------------------------------------------------

start-project:
	docker-compose -d up

# ----------------------------------------------------------
# STOP PROJECT
# ----------------------------------------------------------

stop-project:
	docker-compose down

# ------------------------
# Database Migrations
# ------------------------

# migrations
create-table-in-db:
	cd db/ && sqlx migrate run

# !!!!! Todo
#delete-table-in-db:
#	cd db/ && sqlx migrate revert
