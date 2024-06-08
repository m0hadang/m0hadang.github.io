# 고차 함수

함수를 마치 클래스에서 만든 인스턴스처럼 취급하는 방법.
- 함수를 파라미터로 넘겨줄 수 있음.
- 함수를 결과 값으로 반환 받을 수 있음.

코틀린에서는 모든 함수를 고차 함수로 사용 가능
- 람다 함수는 일반 함수와 달리 그 자체가 고차 함수이다. 

```kotlin
fun a(str: String) { println("function $str") }
fun b(function: (String) -> Unit) { function("kotlin") }

fun main() {
    b(::a) // 일반 함수

    val c: (String) -> Unit = { str -> println("lambda $str") } // 람다 함수
    b(c)
    b { str -> println("inner $str") } // 람다 축약
//  b({ str -> println("inner $str") })    
}
```

# 스코프 함수

인스턴스의 속성, 함수를 사용하기 쉽게 보조하는 함수.

이 보조 함수들은 임시적인 스코프를 생성하여 인스턴스를 사용하기 쉽게 보조한다.

apply
```kotlin
fun main() {
    var a = Book("Kotlin", 10000).apply {
        name = "book name is $name"
        discount()
    }
    println(a.name)
}
```
- 인스턴스의 변수와 함수 조작 가능.
- apply 는 인스턴스 자신을 반환.

run
```html
fun main() {
    var a = Book("Kotlin", 10000).run {
        name = "book name is $name"
        discount()
    } 
    println(a)
}
```
- 자신이 아닌 새로운 결과값 반환. 

with
- run 과 파라미터로 넘겨 받는 차이가 있을 뿐 그 이외 차이가 없다.

also/let

- also : apply와 같은 기능 가지고 있음. 대신 it을 통해 파라미터로 전달 받아 사용 가능. 처리가 끝나면 인스턴스를 반환.
- let  : run과 같은 기능 가지고 있음. 대신 it을 통해 파라미터로 전달 받아 사용 가능. 처리가 끝나면 최종값을 반환.
- it을 사용하는 이유는 같은 이름의 외부 변수와 혼란을 막기 위해.
```kotlin
fun main() {
    val price = 20000
    var a = Book("Kotlin", 10000).apply {
        name = "book name is $name"
        discount()
    }
    a.let {
        println("${it.name}, ${it.price}") // 충돌 방지
    }
}
```

# Object(Singleton), Companion Object(Static Member)

코틀린은 언어차원에서 싱글턴을 지원한다.

오브젝트는 그 자체로 인스턴스이다. 따라서 별도의 인스턴스를 생성하지 않으며 생성자도 없다.

**오브젝트는 최초 사용시 자동으로 생성 된다.**

```kotlin
class Car(val horsePower: Int) {
    companion object { fun create() = Car(100) }
}

object CarFactory {
    val cars = mutableListOf<Car>()
    fun build(horsePower: Int): Car {
        val car = Car(horsePower)
        cars.add(car)
        return car
    }
}

fun main() {
    val car = Car.create()
    println(car.horsePower)

    val car2 = CarFactory.build(200)
    println(car2.horsePower)
    println(CarFactory.cars.size)
}
```

# 익명 객체

object 키워드 사용하여 클래스 정의와 인스턴스 생성을 동시에 처리 가능.

```kotlin
interface EventListener {
    fun onEvent(count: Int)
}

class Counter(private var listener: EventListener) {
    fun count() {
        for (i in 1..100) {
            if (i % 5 == 0) listener.onEvent(i);
        }
    }
}

fun main() {
    val counter = Counter(
        object : EventListener { // EventListener를 상속 하는 익명 클래스
            override fun onEvent(count: Int) { // EventListener onEvent 구현
                println("Count: $count")
            }
        }
    )
    counter.count()
}
```

# null check

- `?.` : null safe operator
- `?:` : elvis operator
- `!!` : non-null assertion operator

# 동일성

내용의 동일성
- `a == b`
- ex) 문자열 내용이 같은가 ? 


객체의 동일성
- `a === b`
- ex) 같은 객체를 참조 하는가 ? 


내용의 동일성이 사용될때 모든 클래스가 상속 받는 Any 클래스(최상위)의 equals() 호출한다.

경우에 따라 사용자 정의 함수에서 equals()를 재정의 해야 한다.

```kotlin
class Product(val name: String, val price: Int) {
    override fun equals(other: Any?): Boolean {
        if (other is Product) {
            return other.name == name && other.price == price
        }
        return false
    }
}

fun main() {
    val a = Product("Kotlin", 100)
    val b = Product("Kotlin", 100)
    val c = a
    println(a == b) // true
    println(a === b) // false
    println(a === c) // true
}
```

# 중첩 클래스, 내부 클래스

```kotlin
class Outer {
    class Nested {              // 중첩 클래스
        fun hello() = "Hello"   // 외부 클래스와 별개의 클래스. 이름이 단지 Outer.Nested 이다.
    }
    inner class Inner {         // 내부 클래스
        fun hello() = "Hello"
    }
}
```

중첩 클래스
- 외부 클래스의 내용을 공유할 수 없음.    
- 하나의 클래스가 다른 클래스의 기능과 강하게 연관되어 있다는 의미를 전달하기 위해 사용.

내부 클래스 
- 외부 클래스의 속성과 함수의 사용이 가능
- 혼자서 객체 생성 불가하고 외부 클래스 객체가 있어야만 생성/사용 가능.

Ex)

```kotlin
class Outer(
    val name: String = "Outer"
) {
    class Nested(private val name: String = "Nested") {
        fun hello() { println(name) }
    }
    inner class Inner {
        fun hello() { println(name) }
    }
}

fun main() {
    val outer = Outer()
    val nested = Outer.Nested()
    val inner = outer.Inner()
    
    nested.hello() // Nested
    inner.hello() // Outer
    outer.Inner().hello() // Outer
    Outer.Nested().hello() // Nested
}
```

# Data Class

데이터를 다루는데 최적화된 클래스로 5 함수를 자동으로 생성.
- equals() / hashCode() / toString() / copy() / componentN()

copy()의 경우 파라미터를 주면 일부 속성을 변경 가능.
```kotlin
val a = Product("Kotlin", 100)
val b = a.copy(price = 200)
```


componentN()을 사용하여 프로퍼티에 접근 가능.
```kotlin
data class SomeData(val a: Int, val b: String)

fun main() {
    val data = SomeData(42, "Hello")
    println(data.component1())
    println(data.component2())
}
```

- 사실 이 함수는 사용자가 직접 호출하기 위해서 구현된것이 아닌 배열에서 값을 꺼낼때 사용하기 위해 구현 되었다.
- `listOf(SomeData(1, "a"), SomeData(2, "b")).map { (a, b) -> "$a $b" }`

# Enum Class

클래스이기에 생성자와 함수를 가질 수 있다.

```kotlin
enum class Color(val numver: Int) {
    RED(1), BLUE(2), GREEN(3);
    fun isRed() = this == RED      // 함수는 마지막에 정의 해야함.
}
```


# Zip(join)

두 컬렉션의 각 요소를 Pair 로 묶어 반환. 결과의 갯수는 아이템의 갯수가 적은 컬렉션을 따라감.

```kotlin
fun main() {
    val a = listOf("a", "b", "c")
    val b = listOf(1, 2, 3)
    val result = a zip b
    println(result)
}
```

# const

const 는 string 포함 기본 자료형만 가능하며 객체같이 런타임에 변경 가능한 자료형은 변경 사용 불가.
```kotlin
const val a = 10  // 가능
const val a = "a" // 가능
const val a = User(name = "Alice",age = 20) // 불가능
```

const 는 클래스 멤버 변수, 함수 지역 변수로 사용 불가하며 반드시 companion object 안에서만 사용 가능.
- 객체의 생성과 관계 없이 사용 가능


# lateinit(늦은 초기화)

코틀린에서 변수를 초기화 하지 않을 경우 컴파일 에러가 발생 하는데 이를 우회하는 방법.
- `lateinit var a: Int`

lateinit 변수 초기화 하였는지 확인.
- `::a.isInitialized`

제한 사항
- 초기값 할당 전까지 변수를 사용할 수 없음(에러)
- 프리미티브 타입에는 사용 불가

# lazy

lateinit과 비슷. 하지만 나중에 초기화를 하지 않더라도 변수를 사용하게 되면 알아서 lazy에서 정의한 값으로 초기화 하고 사용.

```kotlin
fun main() {
    val a: Int by lazy { // 람다
        println("init")
        10
    }
    println("not init")
    println(a) // init 10
}

not init
init
10
```

# 코루틴(비동기처리)

Scope : 코루틴은 제어범위 및 실행범위 지정 가능.
- GlobalScope : 프로그램 어디서나 제어, 동작이 가능한 코루틴 범위.
- CoroutineScope : 특정한 목적의 Dispatcher를 지정하여 제어 및 동작이 가능한 새로운 코룬틴 범위 생성.

코루틴의 dispatcher.
- CoroutineScopef를 만들 때 적용 가능한 Dispatcher.
- Dispatchers.Default : 기본적인 백그라운드 동작.
- Dispatchers.IO : I/O 중심의 동작.
- Dispatchers.Main : 메인 스레드에서 동작.

**!!! Dispatcher 들은 모든 플랫폼에서 지원되지 않으니 지원 되는 플랫폼에서 사용 !!!**

코루틴은 launch, async 키워드를 사용하여 생성 가능.

```kotlin
suspend fun main() {
    val scope = CoroutineScope(Dispatchers.Default)
    val coroutineA = scope.launch {    // 반환값 없는 Job 객체.
        println("A")
    }
    val coroutineB = scope.async {     // 반환값 있는 Deferred 객체.
        println("B")
        "Result"
    }
    coroutineA.join()
    println(coroutineB.await())
}
```

안드로이드에서 runBlocking을 걸어준 후 일정 이상 응답이 없으면 ANR이 발생 하면서 앱이 자동 종료.

delay, async, launch, await, join, ... 이런 키워드는 루틴의 대기 가능한 블록(runBlocking) 안에서만 동작이 가능.

# cancel
코루틴에 cancel 사용시 발생할 수 있는 2가지 경우
1. 코루틴 내부의 delay() 함수 또는 yield() 함수가 사용된 위치까지 수행된 뒤 종료.
2. cancel() 로 인해 속성인 isActive 가 false 가 되고 이를 확인하여 수동으로 종료.


Ex)

```kotlin
fun main() {
    runBlocking {
        val job = launch { // cancel 이 없다면 프로그램 1000 번 모두 수행 하고 프로그램 종료 
            repeat(1000) { i ->
                println("job: I'm sleeping $i ...")
                delay(500L) // 취소 위치
            }
        }
        delay(1300L) // delay a bit
        println("main: I'm tired of waiting!")
        job.cancel() // cancels the job.  
        job.join() // waits for job's completion
        println("main: Now I can quit.")
    }
}
```

# withTimeoutOrNull

cancel 대신 withTimeoutOrNull 사용하여 일정 시간이 지나면 취소 할 수 있음.

```kotlin
fun main() {
    runBlocking {
        val result = withTimeoutOrNull(1300L) {
            repeat(1000) { i ->
                println("job: I'm sleeping $i ...")
                delay(500L)
            }
            "Done" // will get cancelled before it produces this result
        }
        println("Result is $result")
    }
}
```