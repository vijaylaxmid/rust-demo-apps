# Mysql Change Data Capture (CDC) - on Fluvio Streaming Platform

**Change Data Capture (CDC)** converts database changes in real-time events that can be used to improve replication, keep audit trails, as well as feed downstream business intelligence tools.

This demo focuses on **database replication**. We'll use a **CDC producer** to publish database events to a **Fluvio stream** and **CDC consumers** to create identical databases replicas.

## Requirements

Access to a Fluvio installation (local or cloud)

* [Setup Fluvio Cloud](https://app.fluvio.io/login)


## Standalone Deployment

For a standalone deployment using your own databases, use instructions in [cdc-consumer](./cdc-consumer/README.MD) and [cdc-producer](./cdc-producer/README.MD) sections.


## Laptop Demo Deployment

The laptop demo environment with 2+ mysql instances requires Docker:

* [Setup MYSQL Docker Environment](./docker/README.MD)

Please complete the Docker setup, before moving to the next section.


## Build Producer/Consumer

In **mysql-cdc** directory, build producer/consumer

```
$ cargo build
...
Compiling cdc-producer v0.1.0 (/projects/github/rust-demo-apps/mysql-cdc/cdc-producer)
Compiling cdc-consumer v0.1.0 (/projects/github/rust-demo-apps/mysql-cdc/cdc-consumer)
Finished dev [unoptimized + debuginfo] target(s) in 1m 21s
```

## Start CDC Producer

**None**: Producer must be started before Consumer, as the producer creates the topic in Fluvio.

Producer **profile** is configured to connect to the mysql database configured in [Docker setup](./docker/README.MD).

Start producer:

```
./target/debug/cdc-producer ./cdc-producer/producer_profile.toml
```

## Start CDC Consumer

Consumer **profile** is configured to connect to the mysql database configured in [Docker setup](./docker/README.MD). 

Start consumer:

```
./target/debug/cdc-consumer cdc-consumer/consumer_profile.toml 
```

## Connect to Mysql

In two separate terminal windows, connect to the mysql databases

### Connect to Producer (master db)

Producer database is mapped port **3080**.

```
$ mysql -h 0.0.0.0 -P 3080 -ufluvio -pfluvio4cdc!
...
mysql >
```

Producer profile has a filter that registers only changes applied to the "flvDB" database. Let's create the database:

```
mysql> CREATE DATABASE flvDb;
Query OK, 1 row affected (0.01 sec)

mysql> use flvDb;
Database changed
```

### Connect to Consumer (slave db)

Consumer database is mapped port **3090**.

```
$ mysql -h 0.0.0.0 -P 3090 -ufluvio -pfluvio4cdc!
...
mysql >
```

**NOTE**: Database was crated the by the cdc-producer (check the log).

Let's take a look:
```
mysql> SHOW DATABASES;
+--------------------+
| Database           |
+--------------------+
| flvDb              |
| information_schema |
+--------------------+
2 rows in set (0.00 sec)

mysql> use flvDb;
Database changed
```

## MYSQL Test Commands

In the producer terminal, generate mysql commands and see then propagated to the consumer database;

### Producer

```
mysql> CREATE TABLE pet (name VARCHAR(20), owner VARCHAR(20), species VARCHAR(20), sex CHAR(1), birth DATE);
Query OK, 0 rows affected (0.03 sec)

mysql> show tables;
+-----------------+
| Tables_in_flbDb |
+-----------------+
| pet             |
+-----------------+
1 row in set (0.00 sec)
```

### Consumer

Create table has been propagated to the consumer:

```
mysql> show tables;
+-----------------+
| Tables_in_flbDb |
+-----------------+
| pet             |
+-----------------+
1 row in set (0.00 sec)
```

## Sample MYSQL Commands

For additional mysql commands, checkout [MYSQL-COMMANDS](./MYSQL-COMMANDS.MD)


## Fluvio Events

CDC producer generates the events on the **Fluvio topic** as defined in the producer profile. By default the topic name is **rust-mysql-cdc**. 

To view the events generated by the CDC producer, start run **fluvio consumer** command:

```
$ ./target/debug/fluvio consume rust-mysql-cdc -B
...
{"uri":"flv://mysql-srv1/flvDb/year","sequence":30,"bn_file":{"fileName":"binlog.000003","offset":10650},"columns":["y"],"operation":{"Add":{"rows":[{"cols":[{"Year":1998}]}]}}}
{"uri":"flv://mysql-srv1/flvDb/year","sequence":31,"bn_file":{"fileName":"binlog.000003","offset":10921},"columns":["y"],"operation":{"Add":{"rows":[{"cols":[{"Year":1999}]}]}}}
{"uri":"flv://mysql-srv1/flvDb/year","sequence":32,"bn_file":{"fileName":"binlog.000003","offset":11192},"columns":["y"],"operation":{"Delete":{"rows":[{"cols":[{"Year":1998}]},{"cols":[{"Year":1999}]}]}}}
```