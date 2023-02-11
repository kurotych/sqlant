-- Add second table. Add PK and FKS!

CREATE SCHEMA test_schema;

CREATE TABLE test_schema.customers (
  customer_id INT PRIMARY KEY,
  customer_name VARCHAR(50)
);

CREATE TABLE test_schema.orders (
  order_id INT PRIMARY KEY,
  order_description VARCHAR(50),
  customer_id INT,
  FOREIGN KEY (customer_id) REFERENCES test_schema.customers(customer_id)
);

