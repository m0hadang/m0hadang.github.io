# 균형 분기점

두 알고리즘이 있다. 

- A : a(n^2)
- B : b(n) + m

a, b, m 이 어떤 값이든 B 알고리즘이 A 알고리즘보다 바른 조건이 되는 n이 존재할 것이다.

이러한 n의 값을 균형 분기점(break-even point)이라고 한다.

단순히 차수가 적은 B 알고리즘을 선택 하기 보다는 환경에 따라 균형 분기점을 파악 후 적절한 알고리즘을 선택하는 것이 중요하다.

# 빅오

정의 : 모든 n, n >= n0 에 대해 f(n) <= c*g(n) 이 성립하는 두 양의 상수 c, n0 가 존재하면 f(n) = O(g(n)) 이라고 한다.(f(n) 의 빅오는 g(n)이다)

위 정의가 너무 어려워 개인적으로 재설명 해본다.

f(n)의 빅오가 g(n)이 되기 위한 조건
1. f(n) <= c*g(n)이 성립 되어야 한다.
2. n, n >= n0 이며 n0, c 는 양의 상수 이다.
   - n, n >= n0 : n 은 다른 양의 상수 보다 크거나 같기에 1 보다는 크다.
3. 위 조건들 을 만족 하는 여러 g(n) 중에서 가장 작은 g(n)이 빅오 이다.

ex)
- 3n + 2
  - n >= 2 일때 3n + 2 <= 4n 성립
  - O(n)
- 100n + 6
  - n >= 3 일때 100n + 6 <= 101n 성립
  - O(n)
- 10n^2 + 4n + 2
  - n >= 5 일때 10n^2 + 4n + 2 <= 11n^2 성립 
  - O(n^2) 
   
빅오는 알고리즘에 대한 상한값을 의미한다. 최악의 경우에 이 상한값을 넘지 않을 것이다.

수학적 정리

f(n) = a_m(n^m) + ... a_1(n) + a_0 이면 O(n^m) 이다.

$a$

"$a$"

$$
frac{1}{2}
$$

$$
d_{ij} = \sum_{k=0}^{n-1}a_{ik}b_{kj}
$$

```markdown
|a|
|-|
|b|
```