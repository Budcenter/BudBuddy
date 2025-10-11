-- Drop indexes first
DROP INDEX IF EXISTS idx_strains_subspecies;


DROP INDEX IF EXISTS idx_strains_name;


-- Drop many-to-many relationship tables
DROP TABLE IF EXISTS cannabis.strain_ailments;


DROP TABLE IF EXISTS cannabis.strain_flavors;


DROP TABLE IF EXISTS cannabis.strain_effects;


-- Drop lookup tables
DROP TABLE IF EXISTS cannabis.ailments;


DROP TABLE IF EXISTS cannabis.flavors;


DROP TABLE IF EXISTS cannabis.effects;


-- Drop main strains table
DROP TABLE IF EXISTS cannabis.strains;


-- Drop enum type
DROP TYPE IF EXISTS cannabis.subspecies;


-- Drop schema last
DROP SCHEMA IF EXISTS cannabis;