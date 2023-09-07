FROM postgres

ADD ./init.sql /docker-entrypoint-initdb.d

EXPOSE 5432

ENV POSTGRES_PASSWORD=secret
ENV POSTGRES_USER=lauti
ENV POSTGRES_DB=basic_ecommerce

