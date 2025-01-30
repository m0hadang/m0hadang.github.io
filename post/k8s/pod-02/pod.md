기존 실행된 같은 이름의 파드 중지 

```shell
$ kubectl delete pods my-nginx-pod
pod "my-nginx-pod" deleted
```

포드 실행

```shell
$ kubectl apply -f .\nginx-pod-with-ubuntu.yaml
pod/my-nginx-pod created
```

2 개의 컨테이너 실행 중

```shell
$ kubectl get pods
NAME           READY   STATUS    RESTARTS   AGE
my-nginx-pod   2/2     Running   0          13s

$ docker ps
CONTAINER ID   IMAGE               COMMAND                   ...
f6a8c88338cd   alicek106/rr-test   "tail -f /dev/null"       ...
bed75afa1f62   nginx               "/docker-entrypoint.…"    ...    
```
- nginx 1개 ubuntu 서버 1개 실행 중

kubectl 사용하여 우분투 서버 접속

```shell
$ kubectl exec -it my-nginx-pod -c ubuntu-sidecar-container bash
```

우분투 서버에서 nginx 서버 접근

```shell
$ curl localhost
```

우분투 컨테이너의 로컬호스트에서 Nginx 서버로 접근이 가능한 것은 포드 내의 컨테이너들이 네트워크 네임스페이스 등과 같은 리눅스 네임스페이스를 공유해 사용하기 때문이다.