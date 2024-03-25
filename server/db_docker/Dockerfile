FROM postgres

RUN apt update -y
RUN apt install postgresql-16-postgis-3 -y

COPY ./*.sql /sql_scirpts/
COPY ./*.sh /docker-entrypoint-initdb.d/

RUN chmod a+r -R /sql_scirpts
