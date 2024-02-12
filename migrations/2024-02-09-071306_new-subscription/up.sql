-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "subscriptions"(
	"id" UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
	"user_id" UUID NOT NULL,
	"name" VARCHAR NOT NULL,
	"updated_at" TIMESTAMP NOT NULL DEFAULT NOW(),
	"created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
	FOREIGN KEY ("user_id") REFERENCES "users"("id")
	ON DELETE CASCADE
);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_name
BEFORE UPDATE ON subscriptions
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

