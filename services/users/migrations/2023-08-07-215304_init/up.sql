CREATE TABLE "users" (
    "id" varchar(36) PRIMARY KEY NOT NULL,
    "email" text NOT NULL,
    "pwd_hash" text NOT NULL,
    "first_name" text NOT NULL,
    "last_name" text NOT NULL,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX "users_email" ON "users" ("email");

CREATE TABLE "liked_movies" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "user_id" varchar(36) NOT NULL,
  "movie_id" INT NOT NULL,
  FOREIGN KEY ("user_id") REFERENCES "users"("id")
);