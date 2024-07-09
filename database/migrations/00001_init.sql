CREATE TABLE "user" (
    "id" VARCHAR(40) NOT NULL,
    "registered_at" TIMESTAMPTZ NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE "api_token" (
    "id" VARCHAR(20) NOT NULL,
    "user_id" VARCHAR(40) NOT NULL,
    "hash" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL,

    PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id")
        REFERENCES "user" ("id")
        ON DELETE CASCADE
)