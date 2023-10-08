## Traits
```rust
trait Greet {
	fn greeting(&self) -> String;
}
```
Greet 트레이트를 적용하면, 해당 타입은 반드시 greeting 함수를 가지고 있어야 함.

## 특정 타입에 트레이트 구현하기
impl [트레이트명] for [타입: enum or struct] {
	여기에 트레이트에서 지정한 인터페이스 구현...
}

Enum
```rust
enum Pet {
	Dog,
	Cat,
	Hamster
}

impl Greet for Pet {
	fn greeting(&self) -> String {
		match (self) {
			Pet::Dog => String::from("멍멍"),
			Pet::Cat => String::from("야옹"),
			Pet::Hamster => String::from("찍찍"),
		}
	}
}
```

Struct
```rust
struct Person {
	name: String,
	active: bool,
}

impl Greet for Person {
	fn greeting(&self) -> String {
		String::from("안녕")
	}
}
```


## 파라미터로 트레이트 전달하기 1
```rust
fn meet(one: &impl Greet, another: &impl Greet) { // greet 트레이트를 구현한 어떤 타입이든!
	println!("첫번째가 인사합니다 : {}", one.greeting());
	println!("두번째가 인사합니다 : {}", another.greeting());
	// 파라미터의 타입 지정 시 트레이트를 전달했으므로 두가지 파라미터 모두 greeting이라는 메소드를 가지고 있는것을 알 수 있음.
}

fn main() {
	let cat = Pet::Cat;
	let gildong = Person {
		name: String::from("홍길동"),
		active: true,
	};
	meet(&cat, &gildong);
}
```


## 파라미터로 트레이트 전달하기2 : Trait Bound
위와같이 Greet 트레이트를 구현한 두가지 인자값을 전달하고자 할 때, 두 인자값의 타입까지 동일하기를 원하는 경우 : 제너릭과 함께 사용하기

```rust
fn meet<T: Greet> (one:&T, another: &T) { // greet 트레이트를 구현한 같은 타입을 가지는 one과 another
	println!("첫번째가 인사합니다 : {}", one.greeting());
	println!("두번째가 인사합니다 : {}", another.greeting());
	// 파라미터의 타입 지정 시 트레이트를 전달했으므로 두가지 파라미터 모두 greeting이라는 메소드를 가지고 있는것을 알 수 있음.
}

fn main() {
	let cat = Pet::Cat;
	let gildong = Person {
		name: String::from("홍길동"),
		active: true,
	};
	meet(&cat, &gildong); // ❌ 타입이 다르므로 불가

	let ham = Pet::Hamster;
	meet(&cat, &ham); // ⭕️ 타입까지 같으므로 가능
}
```


## 파라미터로 트레이트 전달하기3 : 여러개의 트레이트 전달
+로 추가 가능함

```rust
use std::fmt::Debug;

fn meet<T: Greet + Debug> (one:&T, another: &T) { // greet 트레이트와 Debug트레이트를 모두 구현한 같은 타입을 가지는 one과 another
	println!("첫번째가 인사합니다 : {}", one.greeting());
	println!("두번째가 인사합니다 : {}", another.greeting());
	// 파라미터의 타입 지정 시 트레이트를 전달했으므로 두가지 파라미터 모두 greeting이라는 메소드를 가지고 있는것을 알 수 있음.
}
```

두가지 인자값에 다른 트레이트를 적용하고자 할 때는
```rust
use std::fmt::Debug;

fn meet<T: Greet + Debug, U: Greet + Display> (one:&T, another: &U) { 
	// 두 인자값에 각각 다른 트레이트를 여러개 적용
	println!("첫번째가 인사합니다 : {}", one.greeting());
	println!("두번째가 인사합니다 : {}", another.greeting());
}

use std::fmt::Display;
// 만들어둔 struct에 Display 트레이트 적용하기
impl Display for Person {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.name.as_str()) // 스트링 슬라이스로 전달해야 해서 as_str() 메소드 사용
	}
}

fn main() {
	let cat = Pet::Cat;
	let gildong = Person {
		name: String::from("Gildong"),
		active: true
	}
	meet(cat, gildong)
;}
```
위와같이 제너릭에 여러 트레이트를 +를 사용하여 추가할 수 있다.
** 참고로 위의 Display 트레이트는 타입에 적용하기 위한 시그니처가 좀 복잡하므로 개발도구가 있어서 걔가 써줌! 💡 확인해보면 있음


그런데 저렇게 표기하면 여러 트레이트를 적용할수록 함수 선언부의 코드가 지저분해짐.
이럴때는 where를 사용해서 별도의 라인으로 트레이트 전달을 할 수 있음.


## 파라미터로 트레이트 전달하기4 : where

```rust
use std::fmt::Debug;

fn meet<T, U> (one:&T, another: &U) 
where // 트레이트를 조금 더 깔끔하게 전달 가능
	T: Greet + Debut,
	U: Greet + Display,
{ 
	println!("첫번째가 인사합니다 : {}", one.greeting());
	println!("두번째가 인사합니다 : {}", another.greeting());
}

use std::fmt::Display;
// 만들어둔 struct에 Display 트레이트 적용하기
impl Display for Person {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.name.as_str()) // 스트링 슬라이스로 전달해야 해서 as_str() 메소드 사용
	}
}

fn main() {
	let cat = Pet::Cat;
	let gildong = Person {
		name: String::from("Gildong"),
		active: true
	}
	meet(cat, gildong)
;}
```