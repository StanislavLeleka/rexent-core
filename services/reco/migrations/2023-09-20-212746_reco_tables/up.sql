CREATE TABLE IF NOT EXISTS "recommendation_types" (
    "reco_type_id" SERIAL PRIMARY KEY NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "description" TEXT
);

CREATE TABLE IF NOT EXISTS "user_recommendations" (
    "user_reco_id" SERIAL PRIMARY KEY NOT NULL,
    "reco_id" INT NOT NULL,
    "user_id" VARCHAR(36) NOT NULL,
    CONSTRAINT "fk_reco_type" FOREIGN KEY("reco_id") REFERENCES "recommendation_types"("reco_type_id")
);

CREATE UNIQUE INDEX "recommendation_type_unique_name" ON "recommendation_types" ("name");
CREATE UNIQUE INDEX "user_recommendations_user_rec" ON "user_recommendations" ("user_id", "reco_id");

INSERT INTO "recommendation_types" ("name", "description") VALUES ('food', 'Food');
INSERT INTO "recommendation_types" ("name", "description") VALUES ('activity', 'Activity');
INSERT INTO "recommendation_types" ("name", "description") VALUES ('place', 'Place');
INSERT INTO "recommendation_types" ("name", "description") VALUES ('events', 'Event');
INSERT INTO "recommendation_types" ("name", "description") VALUES ('movie', 'Movie');
INSERT INTO "recommendation_types" ("name", "description") VALUES ('book', 'Book');
INSERT INTO "recommendation_types" ("name", "description") VALUES ('music', 'Music');
INSERT INTO "recommendation_types" ("name", "description") VALUES ('tv', 'TV Series and Shows');
INSERT INTO "recommendation_types" ("name", "description") VALUES ('podcast', 'Podcast');
INSERT INTO "recommendation_types" ("name", "description") VALUES ('game', 'Game');
INSERT INTO "recommendation_types" ("name", "description") VALUES ('travel', 'Travel Destination');
