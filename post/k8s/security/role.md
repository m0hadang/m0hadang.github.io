쿠버네티스 권한 부여 방법
- Role : 네임스페이스에 속함. 네임스페이스 단위로 권한 부여.
    - ex) 네임스페이스에 속하는 오브젝이여서 디플로이먼트나 서비스처럼 네임스페이스에 속하는 오브젝트들에 대한 권한을 정의할 때 사용
- Cluster Role : 네임스페이스에 속하지 않음. 클러스터 단위로 권한 부여.
    - ex) 네임스페이스에 속하지 않는 오브젝트뿐만 아니라 클러스터 전반에 걸친 기능을 사용하기 위해서도 사용
    - ex) 퍼시스턴트 볼륨의 목록 조회

롤 조회
```shell
현재 네임스페이스의 롤 목록만을 출력
$ kubectl get role
No resources found in default namespace.

클러스터 자체에 존재하는 모든 롤의 목록 출력
$ kubectl get clusterrole
NAME                                                                   CREATED AT
admin                                                                  2024-10-01T04:37:54Z
cluster-admin                                                          2024-10-01T04:37:54Z
edit                                                                   2024-10-01T04:37:54Z
kubeadm:get-nodes                                                      2024-10-01T04:37:55Z
storage-provisioner                                                    2024-10-01T04:38:02Z
system:aggregate-to-admin                                              2024-10-01T04:37:54Z
```

API 그룹은 쿠버네티스의 오브젝트가 가지는 목적에 따라 분류되는 일종의 카테고리.

오브젝트의 API 그룹 확인

yaml 설정 파일 이용하여 role 생성

```shell
$ kubectl apply -f .\service-reader-role.yaml
role.rbac.authorization.k8s.io/service-reader created
$ kubectl get roles
NAME             CREATED AT
service-reader   2024-10-01T05:06:41Z

```

role 을 account 에 바인딩

```shell
$ kubectl apply -f .\rolebinding-service-reader.yaml
rolebinding.rbac.authorization.k8s.io/service-reader-rolebinding created



```

role 이 부여되어 이전에는 실패한 명령어가 현재는 성공

```shell
$ kubectl get services --as system:serviceaccount:default:alicek106
NAME         TYPE        CLUSTER-IP   EXTERNAL-IP   PORT(S)   AGE
kubernetes   ClusterIP   10.96.0.1    <none>        443/TCP   33m
```


롤, 롤 바인딩, 서비스 어카운트는 모두 1:1 관계가 아님
- 하나의 롤은 여러 개의 롤 바인딩에 의해 참조될 수 있음
- 하나의 서비스 어카운트는 여러 개의 롤 바인딩에 의해 권한을 부여받을 수도 있음

롤은 권한을 부여하기 위한 일종의 템플릿과 같은 역할

롤 바인딩은 롤과 서비스 어카운트를 연결하기 위한 중간 다리 역할

### 클러스터 롤

노드, 퍼시스턴트 볼륨 등과 같이 네임스페이스에 종속되지 않는 오브젝트도 존재

클러스터 수준의 오브젝트들에 대한 접근 권한은 서비스 어카운트에 기본적으로 설정돼 있지 않는다

클러스터 롤이라는 이름이 나타내는 것처럼 클러스터 롤은 클러스터 단위의 리소스에 대한 권한을 정의하기 위해 사용

롤을 사용할때와 마찬가지로 클러스터 롤을 특정 대상에게 ㅇ녀결하려면 클러스터 롤 바인딩이라고 하는 쿠버네티스 오브젝트 사용해야함

