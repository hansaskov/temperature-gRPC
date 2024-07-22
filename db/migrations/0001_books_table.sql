CREATE TABLE conditions (
   time TIMESTAMPTZ NOT NULL,
   temperature REAL NULL
);

SELECT create_hypertable('conditions', by_range('time'));