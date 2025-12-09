- 유저 : 실제 사용자. 사용자(사람을) 나타내는 개념. 
- 그룹 : 여러 유저들을 모아놓은 집합.
- 서비스 어카운트 : 개념상으로는 유저의 한 종류지만 유저와 다름.
  - 쿠버네티스에서는 유저나 그룹이라는 오브젝트가 없음

```shell
$ kubectl get services --as system:servicesaccount:default:alicek106
Error from server (Forbidden): services is forbidden: User "system:servicesaccount:default:alicek106" cannot list resource "services" in API group "" in the namespace "default"
```

- User 에 권한이 없다는 에러 메시지
- `system:servicesaccount:default:alicek106` 가 사실은 User 이다. 

그