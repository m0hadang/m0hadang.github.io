# test environment
- [docker compose](../docker-compose/kafka.md)

# create topic

```powershell
kafka-topics --bootstrap-server localhost:9092 --topic my-topic --partitions 3 --replication-factor 1 --create
```

# delete topic

Kafka v2.2+:

```powershell
kafka-topics --bootstrap-server localhost:9092 --delete --topic my-topic
```

Kafka v2.1 or less:

```powershell
kafka-topics --zookeeper localhost:2181 --delete --topic my-topic
```

# consume

```powershell
kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic
```

```powershell
--consumer-property group.id=group1 
--from-beginning
--partition 0
```

can't use `--consumer-property group.id=group1 ` and `--partition 0` at the same time

# produce

```powershell
kafka-console-producer --broker-list localhost:9092 --topic my-topic
```

# list

```powershell
kafka-topics --bootstrap-server localhost:9092 --topic my-topic --describe
```