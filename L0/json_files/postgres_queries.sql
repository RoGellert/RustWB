-- Файл с запросами SQL, собранными в одном месте
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
    date_created TIMESTAMP,
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

CREATE TABLE payments (
     payment_uid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
     order_uid UUID REFERENCES orders(order_uid),
     transaction VARCHAR,
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

SELECT json_agg(result)::text
FROM (
    SELECT
        orders.order_uid,
        orders.track_number,
        orders.entry,
        orders.payment,
        orders.locale,
        orders.internal_signature,
        orders.customer_id,
        orders.delivery_service,
        orders.shardkey,
        orders.sm_id,
        orders.date_created,
        orders.oof_shard,
        json_build_object(
            'transaction_uid', payments.transaction_uid,
            'request_id', payments.request_id,
            'currency', payments.currency,
            'provider', payments.provider,
            'amount', payments.amount,
            'payment_dt', payments.payment_dt,
            'bank', payments.bank,
            'delivery_cost', payments.delivery_cost,
            'goods_total', payments.goods_total,
            'custom_fee', payments.custom_fee
        ) AS payment,
        json_build_object(
            'name', deliveries.name,
            'phone', deliveries.phone,
            'zip', deliveries.zip,
            'city', deliveries.city,
            'address', deliveries.address,
            'region', deliveries.region,
            'email', deliveries.email
        ) as delivery,
        json_agg(
            json_build_object(
                'chrt_id', items.chrt_id,
                'track_number', items.track_number,
                'price', items.price,
                'rid', items.rid,
                'name', items.name,
                'sale', items.sale,
                'size', items.size,
                'total_price', items.total_price,
                'nm_id', items.nm_id,
                'brand', items.brand,
                'status', items.status
            )
        ) AS items
    FROM
        orders
    INNER JOIN
        payments ON orders.order_uid = payments.order_uid
    INNER JOIN
        deliveries ON orders.order_uid = deliveries.order_uid
    INNER JOIN
        items ON orders.order_uid = items.order_uid
    GROUP BY
        orders.order_uid, payments.payment_uid, deliveries.delivery_uid
) result;

