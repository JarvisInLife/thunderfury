DB=DATABASE_URL=sqlite:./data/db/thunderfury.db?mode=rwc
CLI=sea-orm-cli

build:
	cargo build

build-release:
	cargo build -r

clean:
	cargo clean

migrate: build
	./target/debug/thunderfury migrate

orm-cli:
	command -v $(CLI) > /dev/null || cargo install $(CLI)

entity: migrate orm-cli
	$(DB) $(CLI) generate entity -o app/thunderfury/src/entity --with-serde both
