CREATE TABLE IF NOT EXISTS multicall (
    "tx_hash" VARCHAR(64),
    "from" VARCHAR(40),
    "index" INT,
    "data" BYTEA,
    PRIMARY KEY(tx_hash,evt_index)
);