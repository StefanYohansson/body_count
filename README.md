Body Count
===

### Usage

```
Body Count: a list or total of casualties.

Usage:
  body_count (-w | --http)
  body_count (-s | --crawler)
  body_count (-h | --help)
  body_count --version

Options:
  -w --http       Start API.
  -d --deamon     Start API as daemon.
  -s --crawler    Get data from ITEP.
  -h --help       Show this screen.
  --version       Show version.
```

### Migrate

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
  city VARCHAR,
  source VARCHAR NOT NULL
);

CREATE TABLE sources (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL
);

INSERT INTO sources (name) VALUES ('itep-rn');
```
