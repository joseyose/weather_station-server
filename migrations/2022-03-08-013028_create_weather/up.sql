CREATE TABLE weather (
    id SERIAL PRIMARY KEY,
    temperature_c FLOAT NOT NULL DEFAULT 0.0,
    temperature_f FLOAT NOT NULL DEFAULT 0.0,
    humidity FLOAT NOT NULL,
    pressure FLOAT NOT NULL,
    date TIMESTAMP WITH TIME ZONE NOT NULL
)