# Main-Safe

중요한 역할을 맡고 있는 메인스레드를 블락하지 않는다는 것이다.
막힘 없는 앱 환경을 제공하기 위해서는 Main-Safe하게 개발해야하고, 이를 위해 여러 스레드를 활용하여 동시성 코드를 짜야한다.

# 콜백 방식

```kotlin
// 순차적인 네트워크 호출을 나타내는 코드 

fun getGingerBrave(api: CookieService): gingerBrave {
    api.makeDough{ dough -> 
        api.addMagicPowder(dough){ -> magicDough
            api.escapeOven(magicDough) { -> cookie
                api.fetchGingerBrave(cookie) { -> gingerBrave
                    Log.d("You can't catch me! I'm the Gingerbre.. I'm Ginger Brave!")
                    return gingerBrave
                }
            }
        }
    }
}
```

# 코루틴 방식

```kotlin
suspend fun getGingerBrave(api: CookieService): gingerBrave {
    val dough = api.makeDough()
    val magicDough = api.addMagicPowder(dough)
    val cookie = api.escapeOven(magicDough)
    val gingerBrave = api.fetchGingerBrave(cookie)
    Log.d("You can't catch me! I'm the Gingerbre.. I'm Ginger Brave!")
    return gingerBrave
}
```

코루틴은 콜백 기반 코드를 sequential code로 바꾸어주기 때문에 비동기 코드를 단순화할 수 있습니다.

코틀린의 suspend 키워드는 내부적으로 콜백을 생성한다.
즉, suspend 키워드를 만난 Kotlin 컴파일러는 suspend-resume을 위한 최적화된 콜백 코드를 생성.

# CPS 패러다임

코루틴에는 CPS(Continuation Passing Style) 패러다임이 적용되어 있다.

CPS란 호출되는 함수에 Continuation을 전달하고, 각 함수의 작업이 완료되는 대로 전달받은 코루틴을 호출하는 패러다임을 의미.

Continuation는 콜백과도 비슷하다.


```kotlin
/**
 * Interface representing a continuation after a suspension point that returns a value of type `T`.
 */
@SinceKotlin("1.3")
public interface Continuation<in T> {
    /**
     * The context of the coroutine that corresponds to this continuation.
     */
    public val context: CoroutineContext

    /**
     * Resumes the execution of the corresponding coroutine passing a successful or failed [result] as the
     * return value of the last suspension point.
     */
    public fun resumeWith(result: Result<T>)
}
```

Continuation는 다음에 무슨 일을 해야 할지 담고 있는 확장된 콜백이다.

resumeWith : 특정 함수 A가 suspend 되어야 할 때, 현재 함수에서 a의 결과 값을 T로 받게 해주는 함수.

context : Continuation이 특정 스레드 혹은 스레드 풀에서 실행되는 것을 허용.

Continuation은 호출 함수간의 suspend-resume을 위한 communicator이고, CPS는 함수 호출 시에 이 Continuation을 전달하는 패러다임. Kotlin Coroutine 역시 CPS로 구현되어 있다.






# Link

https://tech.devsisters.com/posts/crunchy-concurrency-kotlin/