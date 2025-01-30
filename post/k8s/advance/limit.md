쿠버네티스와 같은 컨테이너 오케스트레이션 툴의 가장 큰 장점 중 하나는 여러 대의 서버를 묶어 리소스 풀로 사용할 수 있다

클러스터의 CPU나 메모리 등의 자원이 부족할 때, 필요한 용량만큼의 서버를 동적으로 추가함으로써 수평적으로 확장할 수 있기 때문이다.

하지만 서버를 수평적으로 늘리는 스케일 아웃 만큼 중요한 작업이 또 한가지 있다. 바로 클러스터 내부에서 컴퓨팅 자원 활용률을 늘리는 것이다


리소스 제한한 상태로 Pod 생성

```shell
$ kubectl apply -f .\resource-limit-pod.yaml
pod/resource-limit-pod created

$ kubectl get pods -o wide                  
NAME                 READY   STATUS              RESTARTS   AGE   IP       NODE             NOMINATED NODE   READINESS GATES
resource-limit-pod   0/1     ContainerCreating   0          1s    <none>   docker-desktop   <none>           <none>

$ kubectl describe node docker-desktop

...

Non-terminated Pods:          (10 in total)
  Namespace                   Name                                      CPU Requests  CPU Limits  Memory Requests  Memory Limits  Age
  ---------                   ----                                      ------------  ----------  ---------------  -------------  ---
  default                     resource-limit-pod                        1 (6%)        1 (6%)      256Mi (6%)       256Mi (6%)     40s

```


쿠버네티스에서는 CPU 를 압축 가능한 리소스라고 부름
- CPU 경합 발생하여도 컨테이너의 CPU 사용량을 스스토 통제 가능


메모리나 스토리지는 압축 불가능한 리소스라고 부름
- 메모리 사용의 경합이 발생하면 우선 순위가 낮은 컨테이너의 프로세스가 먼저 종료
  - 다른 프로세스가 사용하는 메모리를 강제로 가져올 수 없으니 당연한 결과이다

