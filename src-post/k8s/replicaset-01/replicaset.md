포드 실행

```shell
$ kubectl apply -f .\replicaset-nginx.yaml
replicaset.apps/replicaset-nginx created    <-- 생성됨

$ kubectl get pods
NAME                     READY   STATUS    RESTARTS   AGE
replicaset-nginx-d6zxp   1/1     Running   0          75s
replicaset-nginx-h5w5j   1/1     Running   0          75s 
replicaset-nginx-nfdtv   1/1     Running   0          75s
```
- 3 개의 포드가 생성됨

4 개로 생성할 수 있도록 yaml 수정

```yaml
replicas: 4
```

```shell
$ kubectl apply -f .\replicaset-nginx.yaml
replicaset.apps/replicaset-nginx configured    <-- 수정됨

$ kubectl get pods
NAME                     READY   STATUS    RESTARTS   AGE
replicaset-nginx-d6zxp   1/1     Running   0          3m43s
replicaset-nginx-h5w5j   1/1     Running   0          3m43s
replicaset-nginx-nfdtv   1/1     Running   0          3m43s
replicaset-nginx-zng6r   1/1     Running   0          27s
```

레플리카셋 종료

```shell
$ kubectl delete rs replicaset-nginx
replicaset.apps "replicaset-nginx" deleted
$ kubectl get pods
No resources found in default namespace.
```
- 레플리카셋을 종료하여 포드도 같이 종료 되었지만 서로 종속적인 리소스는 아니다.