#!/bin/bash
set -e

psql internat_management < /sql_scirpts/schema.sql
psql -U postgres -d internat_management -a -f /sql_scirpts/seed.sql
