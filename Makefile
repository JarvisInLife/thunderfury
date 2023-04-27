DB=DATABASE_URL=sqlite:./data/db/thunderfury.db?mode=rwc
CLI=sea-orm-cli

build:
	cargo build

build-release:
	cargo build -r

clean:
	cargo clean

migrate: orm-cli
	mkdir -p ./data/db/
	$(DB) $(CLI) migrate refresh -d app/migration

entity: orm-cli
	$(DB) $(CLI) generate entity -l -o app/entity/src

orm-cli:
	command -v $(CLI) > /dev/null || cargo install $(CLI)
