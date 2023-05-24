CREATE KEYSPACE IF NOT EXISTS my_keyspace WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1};
CREATE TABLE IF NOT EXISTS my_keyspace.user (
    transaction_id UUID,
    subscription_id UUID,
    client_id UUID,
    time_stamp_epoch BIGINT,
    time_stamp TIMESTAMP,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    properties MAP<TEXT,TEXT>,
    PRIMARY KEY (transaction_id,time_stamp_epoch),
);