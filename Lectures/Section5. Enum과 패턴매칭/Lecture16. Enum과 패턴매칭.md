
## match 구문
- Pattern Matching, 패턴매칭, 패턴부합 이라고 함
- 구분 가능한 값에 따라 처리하는 case/switch문과 유사
- 매칭시 해당 값을 꺼낼 수 있다는 점이 특이
- 가능한 값 모두를 처리해야한다는 제약사항이 있음 (enum에 대해서 모든 아이템에 대한 경우의 처리로직이 있어야.)

```rust
enum Color {
	Red,
	Green,
	Blue
}

struct RGB(u8, u8, u8)

// enum Color를 struct RGB로 변경하는 함수
fn color_to_rgb(color: Color) -> RGB {
	match Color {
		Color::Red => RGB(255, 0, 0),
		Color::Green => RGB(0, 255, 0),
		Color::Blue => RGB(0, 0, 255)
	};
	// Color enum의 모든 경우인 Red, Green, Blue에 대해 처리가 있어야 함.
}
```


## match와 함께 값 꺼내기
```rust
enum Message {
	StartGame,
	WinPoint { who: String }, // struct처럼 이름(key)와 값을 가지는 구조 가ㅇ
	ChangePlayerName(String, bool), // tuple처럼 여러 값을 넣을 수 있음
}

fn handle_message(message: Message) {
	match message {
		Message::StartGame => println!("게임시작"),
		Message::WinPoint{who} => println!("{}의 득점", who),
		Message::ChangePlayerName(name, isActive) => println!("{name}으로 이름이 변경됨")
	}
}

fn main() {
	let myMsg = Message::ChangePlayerName(String::from("조예은"), false);
	handle_message(myMsg); // 해당 메세지에 따른 로직 처리
}
```

match문을 사용하면 enum값 내에 담겨있는 (struct, tuple구조) 값들을 바로 가지고와서 사용 가능함


## Option\<T\>  = None | Some(T)
Option 이넘 타입에서 증감 연산하기 => match 구문 활용

```rust
fn increment(x: Option<i32>) -> Option<i32> {
	match x {
		Some(i) => Some(i+1),
		None => None
	}
}

fn main() {
	let x = Some(2);
	println!("{:?}", increment(x));
	println!("{:?}", increment(None))
	
}
```


## match 구문의 와일드카드 _
match 구문은 모든 경우를 다루어야 한다.
이럴 때, 그 중 몇가지만 구현하고, 나머지 경우는 다루고 싶지 않을때 와일드카드로 \_를 사용한다.

```rust
enum Message {
	StartGame,
	WinPoint { who: String }, // struct처럼 이름(key)와 값을 가지는 구조 가ㅇ
	ChangePlayerName(String, bool), // tuple처럼 여러 값을 넣을 수 있음
	Message1,
	Message2,
	Message3(i32), // ...경우의 수 추가
}

fn handle_message(message: Message) {
	match message {
		Message::StartGame => println!("게임시작"),
		Message::WinPoint{who} => println!("{}의 득점", who),
		Message::ChangePlayerName(name, isActive) => println!("{name}으로 이름이 변경됨")
		// wildcard!
		Message::Message3(_) => println!("인자로 받는 값 안쓸때 걍 _로!"),
		_ => println!("아직 구현하지 않은 메세지") // 와일드카드 _
	}
}		
```


# Enum & Pattern Matching 요약
- 한정된 다양한 값들을 표현하는 타입 선언
- 패턴 매칭으로 각각의 경우에 따른 로직구현
- Option\<T\>로 null 없는 세상!
- 와일드카드 _