# 간단한 문자열 검색

target : aaaaabb
 
search : aaab

```
step 1
aaaaabb
aaab

step 2
aaaaabb
 aaab

step 3
aaaaabb
  aaab
... 
```

# KMP 문자열 검색

찾을 문자열의 패턴 정보를 이용하여 검색 효율성 높임.

search : abcabcacab

search 문자열에 대한 pattern table

| character     | a | b | c | a | b | c | a | c | a | b |
|---------------|---|---|---|---|---|---|---|---|---|---|
| pattern count | 0 | 0 | 0 | 1 | 2 | 3 | 4 | 0 | 1 | 2 |

pattern count
- 문자열 시작 부분과 일치하는 패턴의 길이.
- 현재 위치가 패턴의 마지막 위치로 가정. 

### pattern table 을 이용하여 문자열 검색

```
answer  : abc"abcabcacab"

target  : abcabca|bcacab  
search  : abcabca|cab 
```

pattern 에서 | 시점까지 일치 하였다. 여기서 패턴 정보를 활용하여 불필요한 검색을 줄인다. 

| character     | a | b | c | a | b | c | a   | (c) | a | b |
|---------------|---|---|---|---|---|---|-----|-----|---|---|
| pattern count | 0 | 0 | 0 | 1 | 2 | 3 | {4} | 0   | 1 | 2 |


일치하지 않은 지점(c)의 바로 뒤 match count from begin 값{4}을 참조하여 검색 위치를 조정한다.

pattern 의 4위치(b)부터 다시 검색을 시작하면 된다.

```
target  : abcabca|bcacab  
search  :    abca|bcacab 
```

패턴 정보를 이용 하여 앞의 "abca" 는 검색하지 않고 일치하는 문자열을 찾았다.

만약 위 예시와 같이 일치하지 않는다면 이전 방법과 동일하게 패턴 정보를 이용하여 재검색을 진행 하면 된다.

# pattern count 의미

- 패턴 시작부터 현재 위치까지 4개의 문자들이 문자열 시작 부분과 일치.
  현재 위치 {a} 에서 pattern count {4} 는 많의 의미를 내포하고 있다.
- 현재 위치 (c)를 패턴의 마직막 위치로 가정하고 패턴의 시작 위치는 6 - (4 - 1) = 3, 즉 &a 위치.
  - 패턴 시작 &a 부터 패턴 마지막인 현재 위치인 {a} 까지가 패턴
    - abc(abca)bcacab
  - 시작 부분 4 개의 문자열
    - (abca)bcabcacab

| character     | a | b | c | &a | b | c | {a} | (c) | a | b |
|---------------|---|---|---|----|---|---|-----|-----|---|---|
| pattern count | 0 | 0 | 0 | 1  | 2 | 3 | {4} | 0   | 1 | 2 |

즉현재 위치 {a} 패턴 abca 가 존재한다.

위 패턴 정보를 이용하여 다음과 같이 불필요한 검색을 제거한다.
- 검색 과정에서 현재 문자 위치까지 일치 되었다면 패턴 역시 계속 일치 되었으며 그 길이가 4가 되었다. 
- **문자열 시작 부분은 패턴과 동일한 중복 문자열이기에 비교시 생략 가능하다.** 즉 여태까지 abca 패턴이 나왔으며 문자열 시작 부분이 abca 와 같기에 시작 부분 abca 비교 안하여도 됨. 

### pattern table 생성

step 1

아직까지는 패턴이 시작되지 않았기에 0 으로 초기화 한다.

| character     | a | b | c | a | b | c | a | c | a | b |
|---------------|---|---|---|---|---|---|---|---|---|---|
| pattern count | 0 | 0 | 0 |   |   |   |   |   |   |   |

aaaa

step 2

패턴이 발생 하였다. 첫 문자 a 와 1개 일치함으로 1 초기화.

| character     | a | b | c | a | b | c | a | c | a | b |
|---------------|---|---|---|---|---|---|---|---|---|---|
| pattern count | 0 | 0 | 0 | 1 |   |   |   |   |   |   |

step 3

첫번째 문자 일치 이후 2번째 문자도 일치하였다. 2로 초기화.

| character     | a | b | c | a | b | c | a | c | a | b |
|---------------|---|---|---|---|---|---|---|---|---|---|
| pattern count | 0 | 0 | 0 | 1 | 2 |   |   |   |   |   |

step 4

같은 원리로 패턴이 4 개 길이까지 일치하였다.

| character     | a | b | c | a | b | c | a | c | a | b |
|---------------|---|---|---|---|---|---|---|---|---|---|
| pattern count | 0 | 0 | 0 | 1 | 2 | 3 | 4 |   |   |   |

step 5

불일치 발생하여 패턴이 깨졌다. 0 으로 다시 초기화.

| character     | a | b | c | a | b | c | a | c | a | b |
|---------------|---|---|---|---|---|---|---|---|---|---|
| pattern count | 0 | 0 | 0 | 1 | 2 | 3 | 4 | 0 |   |   |

step 6

이후 다시 패턴이 생성되면서 패턴 테지블이 완성 됨.

| character     | a | b | c | a | b | c | a | c | a | b |
|---------------|---|---|---|---|---|---|---|---|---|---|
| pattern count | 0 | 0 | 0 | 1 | 2 | 3 | 4 | 0 | 1 | 2 |

### pattern count 계산 공식

![kmp.png](kmp.png)

### 코드

```cpp
#include <iostream>
#include <stdio.h>
#include <string.h>

const char* target  = "abcabcabcacab";
const char* pattern = "abcabcacab";

const int target_len = strlen(target);
const int pattern_len = strlen(pattern);
	
int pattern_table[100];

void init_pattern_table() {	
	pattern_table[0] = 0;
		
	int begin = 0;
	for (int i = 1; i < pattern_len; ++i) {
		int count = pattern_table[i - 1];
		if (pattern[begin] == pattern[i]) {
			pattern_table[i] = count + 1;
			++begin;
		} else {			
			pattern_table[i] = 0;
			begin = 0;
		}
	}
	
	std::cout << "pattern count=>" << std::endl; 
	for (int i = 0; i < pattern_len; ++i) {
		std::cout << pattern_table[i];
	}
	std::cout << std::endl;
}

int pmatch() {	
	int i = 0;
	int j = 0;
	while (i < target_len && j < pattern_len) {
		if (target[i] == pattern[j]) {
			i++;
			j++;
		} else if(j == 0) {
			i++;
		} else {
			j = pattern_table[j - 1];
		}		
	}
	return j == pattern_len ? (i - pattern_len) : -1;
}

int main() {
	init_pattern_table();	
	int answer = pmatch();
	std::cout << answer << std::endl;
}
```