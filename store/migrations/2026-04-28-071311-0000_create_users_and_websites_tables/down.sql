-- This file should undo anything in `up.sql`

-- Drop tables in reverse order of dependencies

DROP TABLE IF EXISTS "website_tick";

DROP TABLE IF EXISTS "region";

DROP TABLE IF EXISTS "websites";

DROP TABLE IF EXISTS "users";

-- Drop enum (must be last)
DROP TYPE IF EXISTS "website_status";