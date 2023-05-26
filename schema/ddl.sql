CREATE KEYSPACE IF NOT EXISTS my_keyspace WITH REPLICATION = 
        {'class' : 'SimpleStrategy', 'replication_factor' : 1};
CREATE TABLE IF NOT EXISTS my_keyspace.events (
    transaction_id UUID,
    subscription_id UUID,
    client_id UUID,
    time_stamp_epoch BIGINT,
    properties MAP<TEXT,TEXT>,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (transaction_id, client_id, time_stamp_epoch),
);
CREATE TABLE IF NOT EXISTS my_keyspace.clients (
    client_id UUID PRIMARY KEY,
    time_stamp BIGINT,
);
CREATE TABLE IF NOT EXISTS my_keyspace.subscriptions (
    subscription_id UUID PRIMARY KEY,
    client_id UUID,
    time_stamp BIGINT,
);