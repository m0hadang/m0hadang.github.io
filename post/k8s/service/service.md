포드 실행

```shell
$ kubectl apply -f .\deployment-hostname.yaml
deployment.apps/hostname-deployment created

$ kubectl get pods -o wide
NAME                                   READY   STATUS    RESTARTS   AGE   IP          NODE             NOMINATED NODE   READINESS GATES
hostname-deployment-7b57c676b9-7v72n   1/1     Running   0          13s   10.1.0.57   docker-desktop   <none>           <none>
hostname-deployment-7b57c676b9-l5nzd   1/1     Running   0          13s   10.1.0.56   docker-desktop   <none>           <none>
hostname-deployment-7b57c676b9-z8g5h   1/1     Running   0          13s   10.1.0.55   docker-desktop   <none>           <none>
```

클러스터 IP 타입 서비스 실행
```shell
$ kubectl apply -f .\hostname-svc-clusterip.yaml
service/hostname-svc-clusterip created

$ kubectl get services
NAME                     TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)    AGE
hostname-svc-clusterip   ClusterIP   10.101.34.137   <none>        8080/TCP   27s
kubernetes               ClusterIP   10.96.0.1       <none>        443/TCP    165m
```
- hostname-svc-clusterip 라는 서비스 생성됨
  - 쿠버네티스 클러스터 내부에서만 사용 가능한 내부 IP 할당 받음

임의 포드를 하나 생성하여 접속

```shell
$ kubectl run -i --tty --rm debug --image=alicek106/ubuntu:curl --restart=Never -- bash
If you don't see a command prompt, try pressing enter.
```

hostname-svc-clusterip 서비스를 사용해 포드에 접근. 응답 호스트 이름이 다음. 요청이 로드 밸런싱 되고 있음.

```shell
root@debug:/# curl 10.101.34.137:8080
<div class="form-layout">
        <blockquote>
        <p>Hello,  hostname-deployment-7b57c676b9-7v72n</p>     </blockquote>
</div>

root@debug:/# curl 10.101.34.137:8080
<div class="form-layout">
        <blockquote>
        <p>Hello,  hostname-deployment-7b57c676b9-l5nzd</p>     </blockquote>
</div>
root@debug:/#
```


서비스의 라벨 셀렉터(selector)와 포드의 라벨이 매칭돼 연결되면 쿠버네티스는 자동으로 엔드포인트(endpoint)라고 부르는 오브젝트를 별도로 생성

엔드포인트 오브젝트 출력

```shell
$ kubectl get endpoints
NAME                     ENDPOINTS                                AGE
hostname-svc-clusterip   10.1.0.55:80,10.1.0.56:80,10.1.0.57:80   10m
kubernetes               192.168.65.3:6443                        175m
```
- 엔드포인트 오브젝트는 서비스가 가리키고 있는 도착점(endpoint)을 나타냄
- 서비스를 이용해 포드를 연결한다면 엔드포인트는 자동으로 생성됨
- 엔드포인트 자체도 독립된 쿠버네티스의 리소스이기 때문에 이론상으로는 서비스와 엔드포인트를 따로 만드는 것이 가능

서비스 삭제

```shell
$ kubectl delete svc hostname-svc-clusterip
service "hostname-svc-clusterip" deleted
```

노드 타입 서비스 서비스 실행
```shell
$ kubectl apply -f .\hostname-svc-nodeport.yaml
service/hostname-svc-nodeport created

$ kubectl get services
NAME                    TYPE        CLUSTER-IP       EXTERNAL-IP   PORT(S)          AGE
hostname-svc-nodeport   NodePort    10.110.194.163   <none>        8080:31615/TCP   49s
kubernetes              ClusterIP   10.96.0.1        <none>        443/TCP          3h3m
```
- hostname-svc-nodeport 서비스 생성됨
- 31615 : 모든 노드에서 동일하게 접근할 수 있는 포트.
- CLUSTER-IP 할당된 이유는 NodePort 타입의 서비스가 ClusterIP의 기능을 포함하고 있음.
  - NodePort 타입의 서비스를 생성하면 자동으로 ClusterIP의 기능을 사욯라 수 있음
  - 쿠버네티스 클러스터에서 서비스의 내부 IP와 DNS 이름을 사용해 접근할 수 있음


실제 운영 환경에서 NodePort로 서비스를 외부에 제공하는 경우는 많지 않음. 
- NodePort에서 포트 번호를 80 또는 443 으로 설정하기에는 적절하지 않음
- SSL 인증서 적용, 라우팅 등과 같은 복잡한 설정을 서비스에 적용학 ㅣ어려움
- NodePort 서비스 그 자체를 통해 서비스를 외부로 제공하기보다는 인그레스(Ingress)라고 부르는 쿠버네티스의 오브젝트에서 간접적으로 사용되는 경우 많음




노드에서 접근

```shell
curl -v 127.0.0.1:31615
```


ExternalName 서비스
- 쿠버네티스를 외부 시스템과 연동해야 할 때는 ExternalName 타입의 서비스를 사용할 수도 있음
- 서비스가 외부 도메인을 가리키도록 설정

```yaml
apiVersion: v1
kind: Service
metadata:
  name: externalname-svc # 서비스 이름
spec:
  type: ExternalName # ExternalName 타입의 서비스 
  externalName: my.database.com # 요청을 전달할 외부 도메인 
```
- 쿠버네티스 내부의 포드들이 externalname-svc 라는 이름으로 요청을 보낼 경우 쿠버네티스의 DNS 는 my.database.com 으로 접근할 수 있도록 CNAME 레코드를 반환
