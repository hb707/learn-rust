
## 제너릭, 트레이트, 수명

**Generics**
여러 타입에 대해 공통 코드를 작성

**Trait**
다양한 타입 중 제약조건을 만족하는 타입으로 한정

**Lifetime**
참조 Reference 값의 수명 정보를 제공

## Generic
### 1. 함수 선언시 활용


타입별로 나눠진 함수
```rust
fn smallest_i32(list: &[i32]) -> &i32 {
	let mut smallest = &list[0];
	for item in list {
		if item < smallest {
			smallest = item
		}
	}
	smallest
}

fn smallest_char(list: &[char]) -> &char {
	let mut smallest = &list[0];
	for item in list {
		if item < smallest {
			smallest = item
		}
	}
	smallest
}

fn main() {
	let numbers = vec![3, 4, 1, 6, 7, 10];

	let result = smallest_i32(&numbers);
	println!("가장 작은 수는 {}", result);

	let chars = vec!['홍', '길', '동'];

	let result = smallest_char(&chars);
	println!("가장 작은 글자는 {}", result);
}
```

함수 선언에 제너릭 활용
```rust
fn smallest<T>(list: &[T]) -> &T {
	let mut smallest = &list[0];
	for item in list {
		if item < smallest { // 임의의 T 타입에서는 비교연산 불가
			smallest = item
		}
	}
	smallest
}
```
임의의 T 타입에 대해서는 비교연산이 불가함.
비교가 되지 않는 타입들이 있기 때문에
=> 트레이트를 활용하여 비교가 가능한 타입으로 한정해주면 에러 없이 동작함
```rust
fn smallest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
	let mut smallest = &list[0];
	for item in list {
		if item < smallest { // 임의의 T 타입에서는 비교연산 불가
			smallest = item
		}
	}
	smallest
}
```
T: std::cmp::PartialOrd
PartialOrd로 비교가 가능한 특성을 가진 타입으로 한정해서 T를 받겠다!

PartialOrd 트레이트를 적용하면 아래와 같이 String의 배열도 받을 수 있음.
```rust
fn main() {
	// 이것도 가능
	let result = smallest(&["홍길동", "둘리", "도우너"]);
	println!("가장 앞선 이름은 {}", result);
}
```



### 2. Struct 선언시 활용
```rust
struct Point<T> {
	x: T,
	y: T
}

fn main() {
	let p1: Point<i32> = Point{x:2, y: 3};
	let p2: Point<f64> = Point{x: 2.44, y: 6.25};
	println!("p1 = {:?}, p2 = {:?}", p1, p2);
}
```

만약 여러개의 제너릭 활용하고 싶다면
```rust
struct Point<T, U> {
	x: T,
	y: U
}

fn main() {
	let p1: Point<i32, i32> = Point{x:2, y: 3};
	let p2: Point<f64, f64> = Point{x: 2.44, y: 6.25};
	println!("p1 = {:?}, p2 = {:?}", p1, p2);

	let p3: Point<i32, f64> = Point{x:2, y: 4.555};
	// 이렇게 여러 타입을 활용하도록 할수도 있다
}
```


### 3. Enum 선언시 활용
```rust
enum Option<T> {
	Some(T), // 어떤 타입이든 받을 수 있음
	None
}

enum Result<T, E> {
	Ok(T),
	Err(E)
}
```
=> 이것도 제너릭을 활용한 방법!

### 4. 메소드 선언시 활용
```rust
#[derive(Debug)]
struct Point<T> {
	x: T,
	y: T
}

impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}

	fn y(&self) -> &T {
		&self.y
	}
}

fn main() {
	let p1 = Point{x: 1, y: 4};
	println!("p1.x = {}", p1.x());
}
```