서비스
- 포드에 접근하기 위한 규칙을 정의. 쿠버네티스에서 애플리케이션을 배포하기 위해서는 반드시 알아야 할 오브젝트.
- 여러 개의 포드에 쉽게 접근할 수 있도록 고유한 도메인 이름을 부여
- 여러 개의 포드에 접근할 때, 요청을 분산하는 **로드 밸런서** 기능을 수행
- 클라우드 플랫폼의 로드 밸런서, 클러스터 노드의 포트 등을 통해 포드를 외부로 노출

쿠버네티스를 설치할 때 기본적으로 calico, flannel 등의 네트워크 플러그인을 사용하도록 설정되기 때문에 자동으로 오버레이 네트워크를 통해 각 포드끼리 통신할 수 있음
- 단, 어떤 네트워크 플러그인을 사용하느냐에 따라서 네트워킹 기능 및 성능에 차이가 있을 수 있음

도커 컨테이너와 마찬가지로 포드의 IP는 영속적이지 않아 항상 변할 수 있음.

여러 개의 디플로이먼트를 하나의 완벽한 애플리케이션으로 연동하려면 포드 IP 가 아닌, 서로를 발견(Discorvery)할 수 있는 방법 필요

도커와 달리 쿠버네티스는 디플로이먼트를 생헐할 때 포드를 외부로 노출하지 않으며 디플로이먼트의 yaml 파일에는 단지 포드의 애플리케이션이 사용할 내부 포트만 정의
- `- containerPort: 80`

포트를 외부로 노출해 사용자들이 접근하거느, 다른 디플로이먼트의 포드들이 내부적으로 접근하려면 서비스(service)라고 부르는 별도의 쿠버네티스 오브젝트를 생성해야함





포드의 스케일 변경 명령어
```shell
$ kubectl scale --replicas=1 deployment hostname-deployment
deployment.apps/hostname-deployment scaled

$ kubectl get deploy
NAME                  READY   UP-TO-DATE   AVAILABLE   AGE
hostname-deployment   1/1     1            1           65m
```

요청 던잘 메커니즘 결정 속성(externalTrafficPolicy)

```shell
$ kubectl get svc hostname-svc-nodeport -o yaml
...
  externalTrafficPolicy: Cluster
...
```
- Cluster
  - 클러스터의 모든 노드에 랜덤한 포트를 개방. NodePort와 LoadBalancer 타입의 기본 동작.
  - 모든 워커 노드에서 동일한 랜덤 포트가 개방. 클라우드 플랫폼의 로드 밸런서는 노드 중 하나로 요청을 전달하고, 노드는 다시 포드 중 하나로 요청을 전달
    - 노드 간에 요청이 리다이렉트 되어 NAT 발생함으로 클라이언트의 IP를 보존할 수 없음 
- Local
   - 포드가 생성된 노드에서만 접근 가능하며, 로콜 노드에 위치한 포드 중 하나로 요청.
    - 클라이언트가 포드에 접근하려면 포드를 생성한 노드로만 접근 가능
  - 추가적인 네트워크 홉이 발생하지 않으며 전다로디는 요청의 클라이언트 IP 또한 보존
  - 모든 워커 노드에서 동일한 랜덤 포트가 개발 되지만 로드밸런서는 포드가 위치한 노드로만 요청을 전달하며 해당 노드 내의 포드에서만 요청이 분산.
    -네트워크 홉이 한 단계 적으며 클라이언트의 IP 또한 포드의 소스코드 내에서 정상적으로 확인 가능.