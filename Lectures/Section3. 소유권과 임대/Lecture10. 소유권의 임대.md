# 소유권의 임대
##  소유권의 임대
```rust
fn main() {
	let s = String::from("헬로");
	let (len, s) = string_length(s); // s의 소유권을 넘긴 뒤 또다시 받아야 함 -> 번거로움
	println!("문자열 s = {} 의 길이는 = {}", s, len);
}

fn string_length(s: String) -> (usize, String) {
	println!("문자열의 길이는: {}", s.len());
	(s.len(), s) // 리턴값 여러개를 튜플로 반환할 수 있다
}
```
지난 강에서 소유권을 함수로 주고 받는 번거로운 과정이 있었던 것을 확인했다.
이를 해결할 수 있는 방법이 바로 소유권의 임대이다. => & 기호를 이용!

임대를 이용한 코드
```rust
fn main() {
	let s = String::from("헬로");
	let len = string_length(&s); // s의 소유권을 임대해줌 (참조값을 넘김)
	println!("문자열 s = {} 의 길이는 = {}", s, len);
}

fn string_length(s: &String) -> usize { // 인자의 타입에 &기호를 붙여줌
	println!("문자열의 길이는: {}", s.len());
	s.len() // 필요한 리턴값만 반환. 소유권 반환 불필요
}
```

## 참조(Reference)
- 특정 데이터가 위치한, 접근할 수 있는 주소
- 해당 데이터는 다른 소유자가 소유하고 

**참조는 immutable함**
```rust
fn main() {
	let s = String::from("Hello");
	append_word(&s);
	println!("s = {}", s);
}

fn append_word(s : &String) {
	s.push_arr(", World!"); // ❌ 에러발생
}
```
append_word 함수의 인자값으로 받은 것은 String의 참조값 (포인터)
이 값은 기본적으로 immutable 하기 때문에 이 값을 받아서 원 데이터를 변경할 수 없다.

**변경가능한 참조 : mutable reference**
```rust
fn main() {
	let s = String::from("Hello");
	append_word(&mut s); // 인자값 입력 시점에서도 &mut 기호를 넣어주어야함
	println!("s = {}", s);
}

fn append_word(s : &mut String) { // &mut으로 참조값으로 변경이 가능함을 표현
	s.push_arr(", World!"); // ⭕️
}
```
❗️ 변경가능한 참조 mutable reference의 제약조건
- mutable ref 만들면, 추가 참조를 만들 수 없다 (mut, immutable 모두 불가)
```rust
fn main() {
	let mut s = String::from("Hello");
	
	let r1 = &mut s;
	let r2 = &mut s; // 이 라인까지는 에러가 나지 않음

	println!("r1 = {}, r2 = {}", r1, r2); // 복수의 mut ref 사용하는 시점에서 에러 발생
}
```
이는 데이터 경쟁조건 (data race) 때문.

### 데이터 경쟁조건 data race
- 둘 이상의 포인터가 같은 데이터를 참조
- 한 개 이상의 포인터가 데이터를 쓰려고ㅗ 접근
- 해당 데이터 접근을 동기화할 방법이 없음
- Rust는 아예 컴파일 타임에 데이터 경쟁조건을 방지!
=> DB에서 쓰기를 할 때 lock 걸어야하는 느낌으로,,,

**다른 블록에서 여러번의 &mut : ⭕️**
블록이 다르다면 블록을 빠져나오면서 해당 &mut 참조는 어차피 사라지게 되므로 여러개의 &mut 참조 가능

**여러번의 일반 불변 참조와 &mut : ❌**
```rust
fn main() {
	let mut s = String::from("Hello");

	let r1 = &s; // 일반 불변 참조 1
	let r2 = &s; // 일반 불변 참조 2
	println!("r1 = {}, r2 = {}", &r1, &r2);

	let r3 = &mut s;
	println!("r1 = {}, r2 = {}, r3 = {}", &r1, &r2, &r3); // ❌ 여기서 에러
}
```

**여러번의 일반 불변 참조와 &mut 인데 범위가 겹치지 않음 : ⭕️**
```rust
fn main() {
	let mut s = String::from("Hello");

	let r1 = &s; // 일반 불변 참조 1
	let r2 = &s; // 일반 불변 참조 2
	println!("r1 = {}, r2 = {}", &r1, &r2);

	let r3 = &mut s;
	println!("r3 = {}", &r3); // ⭕️ : 일반참조 없이 mut 참조만 사용하면? OK
}
```
참조의 범위는 단순히 {}로 묶인 코드블럭 뿐만 아니라 실제 사용되는 범위도 체크하기 때문

그래서 또 아래 코드는 안됨
```rust
fn main() {
	let mut s = String::from("Hello");

	let r1 = &s; // 일반 불변 참조 1
	let r2 = &s; // 일반 불변 참조 2
	println!("r1 = {}, r2 = {}", &r1, &r2);

	let r3 = &mut s;
	println!("r3 = {}", &r3); // ❌ : 일반참조 없이 mut 참조만 사용하면? OK
	println!("r1 = {}, r2 = {}", &r1, &r2);	
}
```

# 참조 Reference 정리
- 참조를 이용해 소유권을 넘기지 않고 데이터에 접근 가능
- 딱 하나의 변경 가능 참조가 있거나, 불변 참조를 여럿 활용 가능