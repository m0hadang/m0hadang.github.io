
디플로이먼트 실행

```shell
$ kubectl apply -f .\deployment-nginx.yaml
deployment.apps/my-nginx-deployment created
```

디플로이먼트 생성 확인

```shell
$ kubectl get deploy
NAME                  READY   UP-TO-DATE   AVAILABLE   AGE
my-nginx-deployment   3/3     3            3           15s
```


디플로이먼트가 알아서 레플리카셋을 생성하고 레플리카셋이 포드 3개 생성

```shell
$ kubectl get replicasets
NAME                             DESIRED   CURRENT   READY   AGE
my-nginx-deployment-549567945c   3         3         3       2m41s

$ kubectl get pods
NAME                                   READY   STATUS    RESTARTS   AGE
my-nginx-deployment-549567945c-6hnwn   1/1     Running   0          3m10s
my-nginx-deployment-549567945c-gdv24   1/1     Running   0          3m10s
my-nginx-deployment-549567945c-hbcm7   1/1     Running   0          3m10s
```

버전 관리를 위한 이비지 설정

```shell
$ kubectl apply -f .\deployment-nginx.yaml --record
Flag --record has been deprecated, --record will be removed in the future
deployment.apps/my-nginx-deployment configured

$ kubectl set image deployment my-nginx-deployment nginx=nginx:1.11 --record
Flag --record has been deprecated, --record will be removed in the future
deployment.apps/my-nginx-deployment image updated
```
- --record 옵션을 줌으로서 스냅샷 기록
- kubectl set image 명령어 대신 yaml 파일에서 직접 image 항목을 nginx:1.11 로 변경한 다음 kubectl apply -f 명령어로 적용해도 동일하게 변경

버전이 바뀌면 새로 실행 됨
```shell
$ kubectl get pods
NAME                                  READY   STATUS    RESTARTS   AGE
my-nginx-deployment-6b5f4f694-jjf2f   1/1     Running   0          2m9s
my-nginx-deployment-6b5f4f694-tg6wn   1/1     Running   0          2m11s
my-nginx-deployment-6b5f4f694-zxxt5   1/1     Running   0          2m8s
```
 
```shell
$ kubectl get replicasets
NAME                             DESIRED   CURRENT   READY   AGE
my-nginx-deployment-549567945c   0         0         0       9m37s    <-- 첫번째 생성된 레플리카셋
my-nginx-deployment-6b5f4f694    3         3         3       2m33s    <-- 버전이 바뀐 레플리카셋
```

디플로이먼트는 포드의 정보를 업데이트함으로써 새로운 레플리카셋과 포드를 생성했음에도 불구하고 이전 버전의 레플리카셋을 삭제하지 남겨둠
- **디플로이먼트는 포드의 정보가 변경되어 업데이트가 발생했을 때 이전 정보를 리비전으로서 보존**

히스토리 정보

```shell
$ kubectl rollout history deployment my-nginx-deployment
deployment.apps/my-nginx-deployment
REVISION  CHANGE-CAUSE
1         kubectl.exe apply --filename=.\deployment-nginx.yaml --record=true
2         kubectl.exe set image deployment my-nginx-deployment nginx=nginx:1.11 --record=tru
```
- --record=true 옵션으로 디플로이먼트를 변경하면 변경 사항의 위와 같이 디플로이먼트에 기록

롤백

```shell
$ kubectl rollout undo deployment my-nginx-deployment --to-revision=1
deployment.apps/my-nginx-deployment rolled back

$ kubectl get replicasets
NAME                             DESIRED   CURRENT   READY   AGE
my-nginx-deployment-549567945c   3         3         3       15m
my-nginx-deployment-6b5f4f694    0         0         0       8m53s
```

레플리카셋 리비전 정보와 활성화된 레플리카셋 이름 확인

```shell
$ kubectl describe deploy my-nginx-deployment
...
Annotations:            deployment.kubernetes.io/revision: 3
...
NewReplicaSet:   my-nginx-deployment-549567945c (3/3 replicas created)
...
```

