Body Count
===

# Migrate

```
CREATE TABLE deads (
  id SERIAL PRIMARY KEY,
  register_date VARCHAR NOT NULL,
  deceased_date VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  gender VARCHAR,
  age VARCHAR,
  address VARCHAR,
  place VARCHAR,
  cause_death VARCHAR,
  city VARCHAR
);
```
