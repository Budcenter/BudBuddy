-- Add up migration script here
-- Make sure the cannabis schema exists
CREATE SCHEMA IF NOT EXISTS cannabis;


-- Since types don't support IF NOT EXISTS, we throw a custom error
DO $$ BEGIN CREATE TYPE cannabis.subspecies AS ENUM (
    'sativa',
    'indica',
    'ruderalis',
    'hybrid'
);


EXCEPTION
WHEN duplicate_object THEN RAISE NOTICE 'Type cannabis.subspecies already exists';


END $$;


-- Create strains table with all necessary columns
CREATE TABLE IF NOT EXISTS cannabis.strains (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    subspecies cannabis.subspecies,
    image_url TEXT
);


-- This manages the value of each unique effect. Effects can either be positive or negative
-- To check what strains have a certain effect, refer to cannabis.strain_effects
CREATE TABLE IF NOT EXISTS cannabis.effects (
    id SERIAL PRIMARY KEY,
    name text NOT NULL,
    is_positive boolean DEFAULT TRUE NOT NULL,
    created_at timestamp WITH time zone DEFAULT (NOW() AT TIME ZONE 'utc'::text) NOT NULL,
    CONSTRAINT unique_effects_created_at_check CHECK (
        (
            created_at = (created_at AT TIME ZONE 'UTC'::text)
        )
    ),
    CONSTRAINT unique_effects_name_length_check CHECK ((length(name) < 20))
);


-- This manages the value of each unique flavor. 
-- To check what strains have a certain flavor, refer to cannabis.strain_flavors
CREATE TABLE IF NOT EXISTS cannabis.flavors (
    id SERIAL PRIMARY KEY,
    name text NOT NULL,
    created_at timestamp WITH time zone DEFAULT (NOW() AT TIME ZONE 'utc'::text) NOT NULL,
    CONSTRAINT unique_flavors_created_at_check CHECK (
        (
            created_at = (created_at AT TIME ZONE 'UTC'::text)
        )
    ),
    CONSTRAINT unique_flavor_name_length_check CHECK ((length(name) < 20))
);


-- This manages the value of each unique ailment. 
-- To check what strains have a certain ailment, refer to cannabis.strain_ailments
CREATE TABLE IF NOT EXISTS cannabis.ailments (
    id SERIAL PRIMARY KEY,
    name text NOT NULL,
    created_at timestamp WITH time zone DEFAULT (NOW() AT TIME ZONE 'utc'::text) NOT NULL,
    CONSTRAINT unique_ailments_created_at_check CHECK (
        (
            created_at = (created_at AT TIME ZONE 'UTC'::text)
        )
    ),
    CONSTRAINT unique_ailment_name_length_check CHECK ((length(name) < 20))
);


-- Create table to manage strain effects (Many to many)
CREATE TABLE IF NOT EXISTS cannabis.strain_effects (
    id SERIAL PRIMARY KEY,
    strain_id INT REFERENCES cannabis.strains(id),
    effect_id INT REFERENCES cannabis.effects(id)
);


-- Create table to manage strain flavors (Many to many)
CREATE TABLE IF NOT EXISTS cannabis.strain_flavors (
    id SERIAL PRIMARY KEY,
    strain_id INT REFERENCES cannabis.strains(id),
    flavor_id INT REFERENCES cannabis.flavors(id)
);


-- Create table to manage strain ailments (Many to many)
CREATE TABLE IF NOT EXISTS cannabis.strain_ailments (
    id SERIAL PRIMARY KEY,
    strain_id INT REFERENCES cannabis.strains(id),
    ailment_id INT REFERENCES cannabis.ailments(id)
);


-- Add indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_strains_name ON cannabis.strains(name);


CREATE INDEX IF NOT EXISTS idx_strains_subspecies ON cannabis.strains(subspecies);