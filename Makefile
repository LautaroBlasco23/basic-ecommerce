
# HERE YOU HAVE SOME USEFULL SCRIPTS FOR DEV MODE.
# AND YOU CAN RUN "START-APP" to run the app to docker containers.



# ------------------------
# APP CONTAINERS
# ------------------------

start-app:
	docker-compose -d up

stop-app:
	docker-compose down

# ------------------------
# DATABASE
# ------------------------

db-create:
	docker run --name postgres-db -e POSTGRES_PASSWORD=secret -e POSTGRES_USER=lauti -e POSTGRES_DB=basic_ecommerce -p 5432:5432 -d postgres

db-init:
	cd db/ && sqlx migrate run

db-drop:
	cd db/ && sqlx migrate revert

# ---------------------------------------------
# DROP ALL CONTAINERS
# ---------------------------------------------

drop-all-containers:
	docker stop $(docker ps -aq)

# ---------------------------------------------
# DELETE ALL CONTAINERS AND IMAGES
# ---------------------------------------------

docker-remove-all:
	docker rm $(docker ps -aq) && docker rmi $(docker images -q)