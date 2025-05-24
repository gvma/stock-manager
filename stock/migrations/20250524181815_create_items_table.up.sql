-- Add up migration script here
CREATE TABLE "items" (
  "id" BIGSERIAL PRIMARY KEY,
  "name" VARCHAR(255) NOT NULL,
  "amount" INTEGER NOT NULL DEFAULT 0
);