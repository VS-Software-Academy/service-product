--  Ensure the UUID extension is installed.
CREATE EXTENSION IF NOT EXISTS "uuid-ossp"
;
CREATE TABLE "product_category" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    "description" VARCHAR(100) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
;
INSERT INTO "product_category" ("description") VALUES ('Default category')
;
CREATE TABLE "product" (
    "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    "description" VARCHAR(32) NOT NULL,
    "category" UUID NOT NULL REFERENCES "product_category" (id),
    "price" NUMERIC(20, 8) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
;
