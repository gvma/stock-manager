-- Add up migration script here
CREATE TABLE "users" (
  "uuid" UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  "name" VARCHAR(255) NOT NULL,
  "email" VARCHAR(255) NOT NULL,
  "password" VARCHAR(255) NOT NULL,
  "password_reset_code" VARCHAR(255),
  "password_reset_code_expires_at" TIMESTAMP
);