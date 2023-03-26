.PHONY: migrate.apply
migrate.apply:
	@go run github.com/k0kubun/sqldef/cmd/psqldef -U${DATABASE_USER} -W${DATABASE_PASSWORD} -h${DATABASE_HOST} -p${DATABASE_PORT} ${DATABASE_NAME} < schema.sql

.PHONY: migrate.plan
migrate.plan:
	@go run github.com/k0kubun/sqldef/cmd/psqldef -U${DATABASE_USER} -W${DATABASE_PASSWORD} -h${DATABASE_HOST} -p${DATABASE_PORT} ${DATABASE_NAME} --dry-run < schema.sql
