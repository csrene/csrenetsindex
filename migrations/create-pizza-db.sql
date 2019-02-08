CREATE TABLE customers(
    id SERIAL NOT NULL,
    name VARCHAR NOT NULL,
    email VARCHAR
);

CREATE TABLE orders(
    id SERIAL NOT NULL,
    item VARCHAR,
    quantity INT,
    customer VARCHAR
);