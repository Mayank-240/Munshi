CREATE KEYSPACE IF NOT EXISTS my_keyspace WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1};
CREATE TABLE IF NOT EXISTS my_keyspace.user (
    transaction_id UUID PRIMARY KEY,
    subscription_id UUID,
    client_id UUID,
    time_stamp TEXT,
    properties MAP<TEXT,TEXT>,
);