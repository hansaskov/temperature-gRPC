CREATE TABLE conditions (
   time TIMESTAMPTZ NOT NULL,
   temperature FLOAT NULL
);

SELECT
   create_hypertable('  ', by_range('time'));