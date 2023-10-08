# 구조체 메소드

## 메소드
- 함수 fn과 비슷. 파라미터를 받고 반환값을 줌
- 구조체 Struct, 열거 Enum, 트레이트 Trait 문맥 안에서 정의
- 열거와 트레이트는 추후 설명


```rust
struct Rectangle {
	width: u32,
	height: u32
}

impl Rectangle { // impl 키워드로 struct 문맥 안에서 정의
	// &self 파라미터로 구조체 자신이 파라미터로 들어감 (메소드 호출 시에는 인자값 불필요)
	fn area(&self) -> u32 {
		self.width * self.height
	}
}

fn main() {
	let rect = Rectangle {
		width: 20,
		height: 30
	};

	println!("rect의 면적은 : {}", rect.area());
}
```
- impl
- &self


### 메소드의 self
```rust
impl Rectangle { 
	fn area(&self) -> u32 {
		self.width * self.height
	}
}
```
위와 같이 메소드를 정의했을 때
인자값의 자리에는 다음과 같이 적을 수 있음

1.
```rust
rectangle : &Rectangle
```

2. 
```rust
self : &Self
```

3.
```rust
&self
```
위의 self: &Self를 축약한 표현 => 임대!

❗️self는 소유권을 가져갈수도 있고, (불변/가변)임대도 가능!
하지만 &를 붙여 임대형식이 아닌 소유권을 주는 식으로 메소드를 작성하면, 그 이후에는 해당 struct를 아예 사용할 수 없게되므로... 항상 임대형식으로 &self를 받는 메소드를 구현한다

만약에 self를 받아서 값을 변경하는 메소드를 짜고 싶을때는 mut을 사용하여 인자값을 받는다.
```rust
impl Rectangle { 
	fn double_width(&mut self) {
		self.width = self.width * 2;
	}
}
```



## 연관 associated 함수
```rust
struct Rectangle {
	width: u32,
	height: u32,
}
  
impl Rectangle {
	fn square(size: u32) -> Rectangle {
		Rectangle {
			width: size,
			height: size
		}
	}
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
}
  
fn main() {
	let mut rect = Rectangle { // mut 키워드 있어야 mut 메소드 사용가능
		width: 20,
		height: 30,
	};

	  
	println!("rect의 면적은 : {}", rect.area()); //  600
	
	let sq = Rectangle::square(20);
	println!("sq의 면적은 : {}", sq.area()); //  600
	
}
```

Rectangle::square(20)
형태로 사용 가능!



# 구조체 총정리
- 관련있는 값들을 이름을 붙여 모아 구성하는 타입
- impl 블록으로 메소드나 연관함수 만들어 사용가능
- 메소드의 첫번째 파라미터는 self, &self, &mut self 모두 가능 : 함수의 소유권 이전과 동일