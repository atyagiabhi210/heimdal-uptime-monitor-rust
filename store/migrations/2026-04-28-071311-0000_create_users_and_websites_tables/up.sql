-- Enum
CREATE TYPE "website_status" AS ENUM ('Up', 'Down', 'Unknown');

-- Users (create first)
CREATE TABLE "users" (
    "id" TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "user_name" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

-- Websites
CREATE TABLE "websites" (
    "id" TEXT NOT NULL,
    "url" TEXT NOT NULL,
    "time_added" TIMESTAMP(3) NOT NULL,
    "user_id" TEXT NOT NULL,

    CONSTRAINT "website_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "website_user_id_fkey"
        FOREIGN KEY ("user_id") REFERENCES "users"("id")
        ON DELETE RESTRICT ON UPDATE CASCADE
);

-- Region
CREATE TABLE "region" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,

    CONSTRAINT "region_pkey" PRIMARY KEY ("id")
);

-- Website Tick
CREATE TABLE "website_tick" (
    "id" TEXT NOT NULL,
    "response_time_ms" INTEGER NOT NULL,
    "status" "website_status" NOT NULL,
    "website_id" TEXT NOT NULL,
    "region_id" TEXT NOT NULL,

    CONSTRAINT "website_tick_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "website_tick_website_id_fkey"
        FOREIGN KEY ("website_id") REFERENCES "websites"("id")
        ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "website_tick_region_id_fkey"
        FOREIGN KEY ("region_id") REFERENCES "region"("id")
        ON DELETE RESTRICT ON UPDATE CASCADE
);