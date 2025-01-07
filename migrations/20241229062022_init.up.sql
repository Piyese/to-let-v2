CREATE TYPE listing_type AS ENUM (
    'Single',
    'Bedsitter',
    'SelfContained'
);

CREATE TYPE listing AS (
    type_of_listing listing_type,
    price INTEGER ,
    number_of_bedrooms SMALLINT,
    available_units INTEGER,
    images TEXT[],
    additional_fees TEXT[]
);

-- Create the Collection table
CREATE TABLE collections (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    location VARCHAR(255) NOT NULL,
    contact_information TEXT NOT NULL,
    -- amenities TEXT[],
    listings listing[] NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    rules TEXT[]
);

-- istings listing[] NOT NULL CHECK(
--     listings::listing[]IS NOT NULL 
--     AND ARRAY_LENGTH(listings, 1) IS NOT NULL 
--     AND (ALL(x).type_of_listing IS NOT NULL 
--     AND x.price IS NOT NULL 
--     AND x.available_units IS NOT NULL 
--     FOR x IN listings)
-- ),