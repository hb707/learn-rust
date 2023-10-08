# 구조체 기초


## 구조체란

Struct(구조체) : Tuple과 마찬가지로 서로 다른 타입의 여러 값을 하나로 묶어서 타입을 관리할 수 있는 방법

- Tuple은 값들에 대한 이름 없이 순서만으로 관리한다면, 구조체는 이름을 구체적으로 짓고 타입을 묶어서 관리

## 선언 및 이용
```rust
struct User {
	name: String,
	email: String,
	active: bool,
}
```

```rust
fn main() {
	let user = User {
		name: String::from("한빈"),
		email: String::from("hanbin@example.com"),
		active: true
	};

	println!("유저의 이름은 {}", user.name);
}
```

소유권의 개념과 함께 보자면 "한빈"이라는 String의 소유권은 name이 가지고 있고,
name의 소유권은 user가 가지고 있음
user의 범위가 끝나면 내부 데이터도 함께 해제됨

boolean인 active의 상태는 소유권이 아닌 복사로 관리됨!


## 구조체의 필드값 변경

```rust
fn main() {
	let user = User { // let mut user로 선언해야함!
		name: String::from("한빈"),
		email: String::from("hanbin@example.com"),
		active: true
	};

	user.email = String::from("another@example.com") // ❌ 변수 선언 시 처럼 mut 키워드가 있어야 내부 필드값 변경 가능
}
```

유저 구조체를 만드는 유틸리티 함수를 만들면 더 쉽게 생성가능
```rust
fn build_user(name: String, email: String) -> User {
	User{
		name: name, // js처럼 name 만으로 대입 가능
		email: email,
		active: true
	}
}
```


## 기존에 구조체 인스턴스가 있을 때 새로 만들기
방법1
```rust
fn main() {
	let user1 = User { 
		name: String::from("한빈"),
		email: String::from("hanbin@example.com"),
		active: true
	};
	
	let user2 = User { 
		name: user1.name,
		email: user1.email,
		active: true
	};

	println!("유저1의 이름 = {}", user1.name); // ❌
}
```
기존 인스턴스의 값을 하나씩 접근해서 생성 가능
단, String 같은 타입은 위처럼 넘기게 되면 기존 user1.email의 소유권이 user2로 넘어가게 되며, user1.email은 사용할 수 없게 됨.
=> clone() 함수 등으로 해결 가능...


방법2 : ..키워드
```rust
fn main() {
	let user1 = User { // let mut user로 선언해야함!
		name: String::from("한빈"),
		email: String::from("hanbin@example.com"),
		active: true
	};
	
	let user2 = User { 
		active: false,
		..user1 // js의 스프레드와 유사!
	};

	println!("유저1의 이름 = {}", user1.name); // ❌ 여전히 소유권 이전이 발생
}
```
그런데 이렇게 만들어도 소유권 이전이 발생함. user1.name에 접근 불가능



# 튜플 구조체
```rust
struct Color(i32, i32, i32); // RGB
struct Point(i32, i32, i32); // 3차원 좌표계

fn main() {
	let color = Color(1, 2, 3);
	let point = Point(1, 2, 3);

	color.0; // 튜플과 동일한 방식으로 접근
}
```

튜플과의 차이점은?
Color, Point와 같이 이름을 따로 명시함으로서,
각각을 구분 (Color를 넣을 자리에 Point를 넣는 것 방지)
-> 단순히 튜픒만 이용시 서로 혼동해서 사용해야하지만, 이름을 구분해 타입을 엄격히 관리할 수 있다!