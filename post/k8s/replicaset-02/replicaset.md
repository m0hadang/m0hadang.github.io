같은 라벨을 가지는 1 개의 포드 생성

```shell
$ kubectl apply -f .\pod-without-replicaset.yaml
pod/my-nginx-pod created


```

라벨과 같이 출력

```shell
$ kubectl get pods --show-labels
NAME           READY   STATUS    RESTARTS   AGE   LABELS
my-nginx-pod   1/1     Running   0          72s   app=my-nginx-pods-label
```

라벨에 app 있는 리소스만 출력

```shell
$ kubectl get pods -l app
NAME           READY   STATUS    RESTARTS   AGE
my-nginx-pod   1/1     Running   0          2m16s

$ kubectl get pods -l app=my-nginx-pods-label
NAME           READY   STATUS    RESTARTS   AGE
my-nginx-pod   1/1     Running   0          2m16s
```


같은 라벨을 가지는 3 개의 포드 생성

```shell
$ kubectl apply -f .\replicaset-nginx.yaml
replicaset.apps/replicaset-nginx created

$ kubectl get pods
NAME                     READY   STATUS    RESTARTS   AGE
my-nginx-pod             1/1     Running   0          7m15s
replicaset-nginx-hhgvr   1/1     Running   0          29s
replicaset-nginx-w5cvf   1/1     Running   0          29s
```
- replicaset-nginx 2개만 생성됨
- 레플리카셋에 의해 같은 라벨을 가진 포드 3개만 만들어 지도록 관리되고 있음

레블리카셋이 생성해 놓은 포드의 라벨을 삭제

편진기 실행 후 라벨 삭제 & 저장하면 자동으로 설정 반영됨

```shell
$ kubectl edit pods replicaset-nginx-hhgvr
pod/replicaset-nginx-hhgvr edited

labels:                      <-- 제거
  app: my-nginx-pods-label   <-- 제거
```

레플리카셋에 의해 관리되는 라벨에서 1개가 제거 되었음으로 1개를 다시 추가하여 3대로 맞춤

```shell
kubectl get pods --show-labels
NAME                     READY   STATUS    RESTARTS   AGE    LABELS
my-nginx-pod             1/1     Running   0          10m    app=my-nginx-pods-label
replicaset-nginx-hhgvr   1/1     Running   0          4m4s   <none>
replicaset-nginx-lhdpn   1/1     Running   0          85s    app=my-nginx-pods-label
replicaset-nginx-w5cvf   1/1     Running   0          4m4s   app=my-nginx-pods-label
```