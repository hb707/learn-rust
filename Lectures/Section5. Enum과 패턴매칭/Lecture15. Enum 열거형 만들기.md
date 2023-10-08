# Enum
- 여러 정해진 가짓수 중 택일 값을 다루는 타입
- 가위 | 바위 | 보
- 홀 | 짝
- 참 | 거짓
- 주사위 1, 2, 3, 4, 5, 6


## RGB Enum
```rust
#[derive(Debug)]
enum Color {
	Red,
	Green,
	Blue,
}

fn main() {
	let red: Color = Color::Red;
	let green = Color::Green;

	println!("red = {:?}", red)
}
```


**PartialEq** 
비교연산자 구현해주는 지시
```rust
#[derive(Debug, PartialEq)] // PartialEq지시자를 넣으면 자동으로 비교연산자 구현됨
enum Color {
	Red,
	Green,
	Blue,
}

fn main() {
	let red: Color = Color::Red;
	let green = Color::Green;

	println!("red = {:?}", red);
	println!("red == green : {}", red == green); // partialEq
	println!("red == red : {}", red == Color::Red);
	
}
```


##  Enum 개별값 안에 다른 값들 담기

위처럼 단순데이터만 담는것 뿐만 아니라 struct, tuple과 같은 구조를 이름을 붙여 저장하는 것도 가능하다!

```rust
enum Message {
	StartGame,
	WinPoint { who: String }, // struct처럼 이름(key)와 값을 가지는 구조 가ㅇ
	ChangePlayerName(String, bool), // tuple처럼 여러 값을 넣을 수 있음
}

fn main() {
	let m1 = Message::StartGame;
	let m2 = Messsage::WinPoint{
		who: String::from("한빈")
	};
	let m3 = Message::ChangePlayerName(
		String::from("한빈"),
		true
	)
}
```


## enum Option\<T\>
이미 존재하는 enum (별도로 선언할 필요 없음)

❗️어떤 값이 있나 없나를 확인할 수 있음. : Null을 확인하기 위한 수단으로 사용
```rust
enum Option<T> {
	None,
	Some(T)
} // 이미 있음! 언어 내장

fn main() {
	let some_num = Some(2);
	let absent_num: Option<i32> = None;
	// some_num과 absent_num이 같은 타입이 되어 유무를 비교할 수 있음.
	// rust에는 null이 없기 때문에 이렇게 처리를 함.

	// some_num이 None인지 아닌지 확인하려면
	println!("some_num == absent_num : {}", some_num == None)
}
```

하지만 Some을 사용하여 Option\<i32\> 타입의 변수를 선언했으면 +, - 같은 연산이 불가능함
=> 이 문제의 해결은 다음 강에서 *^^*

