
build:
	cargo build

build-release:
	cargo build -r

clean:
	cargo clean

migrate:
	sea-orm-cli migrate refresh -d app/migration

entity:
	sea-orm-cli generate entity -l -o app/entity/src
