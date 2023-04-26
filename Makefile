DB=DATABASE_URL=sqlite:./data/db/thunderfury.db?mode=rwc

build:
	cargo build

build-release:
	cargo build -r

clean:
	cargo clean

migrate: orm-cli
	mkdir -p ./data/db/
	$(DB) sea-orm-cli migrate refresh -d app/migration

entity: orm-cli
	$(DB) sea-orm-cli generate entity -l -o app/entity/src

orm-cli:
	command -v sea-orm-cli > /dev/null || cargo install sea-orm-cli
