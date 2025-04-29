CREATE TABLE noun_roots_table (
    id BIGSERIAL PRIMARY KEY,
    root VARCHAR(255) NOT NULL,
    is_regular BOOLEAN NOT NULL,
    conjugation_group VARCHAR(255) NOT NULL,
    gender BIGINT NOT NULL,
    metadata JSONB
);

