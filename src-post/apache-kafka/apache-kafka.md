# test environment
- [docker compose](../docker-compose/kafka.html) 
- [kafka producer](./python-kafka.html)

# 컨슈머 그룹 별로 메시지 오프셋 관리

컨슈머들이 두 그룹(1 과 2, 3)으로 나누어져 있는 상태에서 프로듀서로 메시지 전송하면 컨슈머 1은 메시지를 받고 컨슈머 2, 3은 둘 중 하나만 메시지를 받는다.

```powershell
# consumer 1
kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic
```

```powershell
# consumer 2(group.id=group1)
kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic --consumer-property group.id=group1

# consumer 3(group.id=group1) 
kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic --consumer-property group.id=group1
```


producer

```powershell
kafka-console-producer --broker-list localhost:9092 --topic my-topic
```

# 키 기반으로 파티션 분배

consumer 1(partition 0)
```powershell
>> kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic --partition 0
result0
result10
result20
```

consumer 2(partition 1)
```powershell
>> kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic --partition 1
result2
result4
...
result24
result26
result28
```

consumer 3(partition 2)
```powershell
>> kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic --partition 2
result1
result3
...
result25
result27
result29
```

# 키 기반으로 컨슈머 그룹의 메시지 분배

consumer 1(group.id=group1)
```powershell
>> kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic --consumer-property group.id=group1
result0
result10
result20
```

consumer 2(group.id=group1)
```powershell
>> kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic --consumer-property group.id=group1
result2
result4
...
result24
result26
result28
```

consumer 3(group.id=group1)
```powershell
>> kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic --consumer-property group.id=group1
result1
result3
...
result25
result27
result29
```