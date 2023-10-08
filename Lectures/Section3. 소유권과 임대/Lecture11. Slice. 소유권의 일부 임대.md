# Slice
- 어떤 모음에 있는 (일부) 연속된 요소들을 참조하는 방법
- 참조와 마찬가지로 소유권을 넘기지는 않음

```rust
fn main() {
	let s = String::from("Hello, World!");
	let word = first_word(&s);
	println!("word = {}", word);
}

fn first_word(s : &String) -> ??? {} // 무엇을 반환할 것인가?
```

## 1. 문자열의 특정 범위 인덱스값
1. 그 범위 값을 통해 문자열로 만드는 것도 번거로우며
2. 원래 문자열이 메모리에서 사라진다면?
3. 원래 문자열이 중간에 값이 바뀌는 경우가 생기면?
=> 인덱스 범위 값이 무효해지는 상황이 발생

## 2. 문자열 슬라이스 &str
```rust
fn main() {
	let s = String::from("Hello, World!");

	let word1: &str = &s[0..6]; // &s[..6]과 동일
	let word2: &str = &s[6..13]; // &s[6..]과 동일
	let all = &s[..]; // 모든 영역

	println!("word1 = {}, word2 = {}", word1, word2)
}
```
소유권을 임대받아 기존 String의 일부를 나타낼 수 있게됨



# 문자열 리터럴은 사실 슬라이스
문자열 리터럴은 어딘가에 있는 영역의 값!
String -> Heap 메모리 공간에 있는 (어쩌면) 변경할 수도 있는 데이터
문자열리터럴 -> 어딘가에 존재하는 값의 참조 (&str)


# 문자열 슬라이스를 파라미터 타입으로!
```rust
fn main() {
	let hello: &str = "Hello World!";
	let s: String = String::from("헬로 월드");
	// 두 타입의 차이

	let word = first_word(&s);
	println!("hello = {}, s = {}", hello, s);
}

fn first_word(s: String) -> &str { // 이 함수는 String 타입만 파라미터로 받을 수 있음
	&s[..5];
}

fn first_word(s: &str) -> &str { // 이 함수는 String 타입, 문자열 슬라이스 타입 모두 인자로 받을 수 있음
	&s[..5];
}
```

함수의 파라미터 타입을 String으로 하면 문자열 리터럴을 인자로 받을 수 없지만,
&str로 하면 문자열슬라이스와 String 타입 모두를 인자값으로 받을 수 있기 때문에
문자열을 받는 함수는 &str을 타입으로 하는 것이 좋음


# 문자열 외의 슬라이스
```rust
fn main() {
	let a = [1,2,3,4,5];
	let slice = &a[1..3];

	println!("a = {:?}, slice = {:?}", a, slice);
}
```
문자열 외의 콜렉션도 슬라이스 당연히 가능하다!

**:?**
화면 출력시의 트레이트 ... 추후 다룰 예정!



# 소유권과 임대 총정리!
- Rust가 안전하고 빠른 프로그램을 만들게 함
- 컴파일러가 엄격히 검사하는 메모리 관리 규칙
- 규칙을 잘 따르면 메모리 해제는 컴파일러가 자동으로
- 소유권 이전과 임대 개념
- 슬라이스로 일부영역 참조도 편리하게 가능!