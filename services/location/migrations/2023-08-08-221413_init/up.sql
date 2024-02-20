CREATE TABLE IF NOT EXISTS "countries" (
  "country_id" SERIAL PRIMARY KEY NOT NULL, 
  "country_code" VARCHAR(3) NOT NULL, 
  "country_name" VARCHAR NOT NULL
);
CREATE TABLE IF NOT EXISTS "cities" (
  "city_id" SERIAL PRIMARY KEY NOT NULL, 
  "city_name" CHARACTER VARYING NOT NULL, 
  "city_code" CHARACTER VARYING NOT NULL, 
  "country_id" INT NOT NULL, 
  CONSTRAINT "fk_country" FOREIGN KEY("country_id") REFERENCES "countries"("country_id")
);
CREATE TABLE IF NOT EXISTS "user_locations" (
  "loc_id" SERIAL PRIMARY KEY NOT NULL, 
  "LAT" FLOAT NOT NULL,
  "LNG" FLOAT NOT NULL,
  "formatted_address" CHARACTER VARYING NOT NULL, 
  "user_id" varchar(36) NOT NULL, 
  "city_id" INT NOT NULL, 
  "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  CONSTRAINT "fk_city" FOREIGN KEY("city_id") REFERENCES "cities"("city_id")
);
CREATE UNIQUE INDEX "locations_unique_user_id" ON "user_locations" USING BTREE ("user_id");
CREATE UNIQUE INDEX "country_unique_name" ON "countries" USING BTREE ("country_name");