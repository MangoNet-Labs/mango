-- Your SQL goes here
CREATE TABLE tx_recipients (
    tx_sequence_number          BIGINT       NOT NULL,
    -- MgoAddress in bytes.
    recipient                   BYTEA        NOT NULL,
    PRIMARY KEY(recipient, tx_sequence_number)
);
CREATE INDEX tx_recipients_tx_sequence_number_index ON tx_recipients (tx_sequence_number ASC);
