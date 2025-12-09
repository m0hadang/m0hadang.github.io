포드 실행

```shell
$ kubectl apply -f nginx-pod.yaml
pod/my-nginx-pod created
```

특정 오브젝트의 목록 확인

```shell
$ kubectl get pods
NAME           READY   STATUS    RESTARTS   AGE
my-nginx-pod   1/1     Running   0          9m15s
```
- 1/1 : 1개의 컨테이너가 정의되어 있음.
  - 실제로 대부분 쿠버네티스의 컨테이너 애플리케이션은 이처럼 1개의 컨테이너로 포드를 구성해 사용함. 하지만 반드시 필수는 아님.

노드 정보도 같이 출력

```shell
$ kubectl get pods -o wide
NAME           READY   STATUS    RESTARTS   AGE   IP          NODE             NOMINATED NODE   READINESS GATES
my-nginx-pod   2/2     Running   0          23m   10.1.0.37   docker-desktop   <none>           <none>
```

생성된 리소스의 정보 상세 확인

```shell
$ kubectl describe pods my-nginx-pod
  ...
Status:           Running
IP:               10.1.0.35   <-- 외부에서 접근할 수 없는 IP
IPs:
  IP:  10.1.0.35         
  ...
```

생성된 리소스를 도커에서 확인

```shell
$ docker ps
CONTAINER ID   IMAGE     COMMAND                  ...     
751b0d75c479   nginx     "/docker-entrypoint.…"   ...
```

쿠버네티스 외부 또는 내부에서 포드에 접근하려면 서비스(service)라고 하는 쿠버네티스 오브젝트를 따로 생성해야 함.
