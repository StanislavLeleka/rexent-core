CREATE TABLE "user_social_accounts" (
    "id" varchar(255) NOT NULL,
    "user_id" varchar(36) NOT NULL,
    "platform" varchar(255) NOT NULL,
    "account_name" varchar(255) NOT NULL,
    "access_token" varchar(255) NOT NULL,
    "refresh_token" varchar(255) NOT NULL,
    "expires_at" timestamp NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT now(),
    "updated_at" timestamp NOT NULL DEFAULT now(),
    CONSTRAINT "user_social_accounts_pkey" PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE
);

CREATE INDEX "user_social_accounts_user_id_index" ON "user_social_accounts"("user_id");
CREATE INDEX "user_social_accounts_platform_index" ON "user_social_accounts"("platform");