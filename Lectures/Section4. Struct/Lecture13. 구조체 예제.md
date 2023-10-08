# 구조체 예제

## 구조체를 이용한 함수 구성
### 1. 단순 데이터 타입
```rust
fn main() {
	let width = 20;
	let height = 30;
	let area = calc_area(width, height);
	println!("사각형의 면적은 {}", area);
}

fn calc_area(width: u32, heigth: u32) -> u32 {
	width * heigth
}
```

### 2. tuple
```rust
fn main() {
	let rect = (20, 30);
	let area = calc_area(rect);
	println!("사각형의 면적은 {}", area);
}

fn calc_area(rect: (u32, u32)) -> u32 {
	rect.0 * rect.1
}
```

### 3. struct
```rust
struct Rect {
	width: u32,
	height: u32
}

fn main() {
	let rect = Rect {
		width: 20,
		height: 30,
	};
	let area = calc_area(&rect); // 임대로 인자값 전달
	println!("사각형의 면적은 {}", area);
}

fn calc_area(rect: &Rect) -> u32 { // 참조로 넘겨주기
	rect.width * rect.height
}
```
파라미터 값 전달 시에도 더 분명하고 엄격한 타입 관리가 가능

## 구조체 출력
구조체를 출력하려고 할 때,
```rust
struct Rect {
	width: u32,
	height: u32
}

fn main() {
	let rect = Rect {
		width: 20,
		height: 30,
	};
	
	println!("사각형 {}", rect); // ❌ 에러 발생
}
```
위와 같이 println으로 구조체를 출력하려고 하면 에러가 발생한다. 
> `Rect` cannot be formatted with the default formatter

Rect 구조체를 화면에 표현할 때의 트레이트(?)를 작성하지 않아서 위와 같은 에러 발생



### :? 트레이트
구조체의 내용을 디버깅 목적으로 확인하려고 할 때 쓰기 좋은 방법
```rust
#[derive(Debug)]
struct Rect {
	width: u32,
	height: u32
}

fn main() {
	let rect = Rect {
		width: 20,
		height: 30,
	};
	
	println!("사각형 {}", rect); // ❌ 에러 발생
}
```
구조체의 위에
> #[derive(Debug)] 

를 작성해줌.
이는 컴파일러에게 무언가를 지시하는 지시자가 된다. : 디버그를 위한 뭔가를 만들어줘!
```rust
println!("rect 구조체 : {:?}", rect); // 디버그 관련 메소드 호출!
// Rectangle {
//	  Width: 20,
//    height: 30
// }
```

println!에서 사용할 때
{} : 일반 디스플레이
{:?} : 디버그 메소드 => 구조체의 내용을 자세히 보여줌
 
 

### dbg! 매크로
```rust
dbg!(rect); 
```

=> 소스파일위치, 파일명, 코드라인, 파라미터로 전달한 값의 구체적인 내용이 콘솔에 표시
손쉬운 디버깅이 가능하다!