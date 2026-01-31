pub const UP: &str = r#"
CREATE TABLE IF NOT EXISTS races (
    id TEXT PRIMARY KEY,
    description TEXT NOT NULL,
    animal_species TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS addresses (
    id TEXT PRIMARY KEY,
    country TEXT NOT NULL,
    postal_code TEXT NOT NULL,
    city TEXT NOT NULL,
    street TEXT NOT NULL,
    street_number TEXT NOT NULL,
    extra TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS animals (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    birthdate TEXT NOT NULL,
    animal_species TEXT NOT NULL,
    description TEXT NOT NULL,
    version INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS clients (
    id TEXT PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    birthdate TEXT NOT NULL,
    email TEXT NOT NULL,
    mobile_number TEXT NOT NULL,
    customer_number INTEGER NOT NULL UNIQUE,
    address_id TEXT NOT NULL,
    consent BOOLEAN NOT NULL,
    version INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (address_id) REFERENCES addresses(id)
);

CREATE TABLE IF NOT EXISTS client_animals (
    client_id TEXT NOT NULL,
    animal_id TEXT NOT NULL,
    PRIMARY KEY (client_id, animal_id),
    FOREIGN KEY (client_id) REFERENCES clients(id),
    FOREIGN KEY (animal_id) REFERENCES animals(id)
);

CREATE TABLE IF NOT EXISTS seminars (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    price TEXT NOT NULL,
    duration TEXT
);

CREATE TABLE IF NOT EXISTS events (
    id TEXT PRIMARY KEY,
    date TEXT NOT NULL,
    location_id TEXT NOT NULL,
    location_name TEXT,
    max_participants INTEGER,
    seminar_id TEXT NOT NULL,
    FOREIGN KEY (location_id) REFERENCES addresses(id),
    FOREIGN KEY (seminar_id) REFERENCES seminars(id)
);
"#;

pub const DOWN: &str = r#"
DROP TABLE IF EXISTS events;
DROP TABLE IF EXISTS seminars;
DROP TABLE IF EXISTS client_animals;
DROP TABLE IF EXISTS clients;
DROP TABLE IF EXISTS animals;
DROP TABLE IF EXISTS addresses;
DROP TABLE IF EXISTS races;
"#;
