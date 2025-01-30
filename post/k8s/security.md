서비스 account 확인

```shell
$ kubectl get serviceaccount
NAME      SECRETS   AGE
default   0         98s
```

account 생성

```shell
$ kubectl create sa alicek106
serviceaccount/alicek106 created
$ kubectl get serviceaccount
NAME        SECRETS   AGE
alicek106   0         1s
default     0         2m50s
```


특정 Account 사용하여 명령어 실행

```shell
$ kubectl get services --as system:serviceaccount:default:alicek106
Error from server (Forbidden): services is forbidden: User "system:serviceaccount:default:alicek106" cannot list

- 권한 부족으로 실패
```
- --as system:serviceaccount : 인증을 위해 서비스 account를 사용
- default:alicek106 : default 네임스페이스의 alicek106 서비스 account 사용

