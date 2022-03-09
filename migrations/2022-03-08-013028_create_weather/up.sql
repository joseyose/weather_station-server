CREATE TABLE weather (
    id SERIAL PRIMARY KEY,
    temperature FLOAT NOT NULL,
    humidity FLOAT NOT NULL,
    pressure FLOAT NOT NULL,
    date TIMESTAMP WITH TIME ZONE NOT NULL
)