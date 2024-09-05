CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE orders (
    order_uid UUID PRIMARY KEY,
    track_number VARCHAR,
    entry VARCHAR,
    payment VARCHAR,
    locale VARCHAR,
    internal_signature VARCHAR,
    customer_id VARCHAR,
    delivery_service VARCHAR,
    shardkey VARCHAR,
    sm_id integer,
    date_created VARCHAR,
    oof_shard VARCHAR
);

CREATE TABLE deliveries (
    delivery_uid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_uid UUID REFERENCES orders(order_uid),
    name VARCHAR,
    phone VARCHAR,
    zip VARCHAR,
    city VARCHAR,
    address VARCHAR,
    region VARCHAR,
    email VARCHAR
);

CREATE TABLE transactions (
     transaction_uid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
     order_uid UUID REFERENCES orders(order_uid),
     request_id VARCHAR,
     currency VARCHAR,
     provider VARCHAR,
     amount integer,
     payment_dt integer,
     bank VARCHAR,
     delivery_cost integer,
     goods_total integer,
     custom_fee integer
);

CREATE TABLE items (
    item_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_uid UUID REFERENCES orders(order_uid),
    chrt_id integer,
    track_number VARCHAR,
    price integer,
    rid VARCHAR,
    name VARCHAR,
    sale integer,
    size VARCHAR,
    total_price integer,
    nm_id integer,
    brand VARCHAR,
    status integer
);

