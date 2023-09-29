# 1. immutable variable

```rust
fn main() {
	let x = 3;
	println!("x의 값은 {x}입니다")
	x = 7; // ❌ 에러발생
	println!("x의 값은 {x}입니다")
}
```

단순히 let 으로 선언한 변수는 불변하는 상수가 되어 다른 값으로 변경할 수 없다.

# 2. mutable variable

```rust
fn main() {
	let mut x = 3;
	println!("x의 값은 {x}입니다")
	x = 7; // ⭕️ 변경 가능
	println!("x의 값은 {x}입니다")
}
```

mut 키워드를 넣음으로서 값을 수정할 수 있음

# 3. 상수 선언 const

```rust
fn main() {
	const PI = 3.141592;
	println!("상수 PI의 값은 {PI}입니다")
}
```

상수 const는 mut 키워드를 붙일 수 없다.

# 4. 변수가리기 shadowing

```rust
fn main() {
	let x = 3;
	println!("x의 값은 {x}입니다"); // 3

	let x = x + 1;
	// shadowing : 기존에 선언된 x 변수를 가려버림 (새로운 x 라는 변수가 선언되는 것과 동일)
	println!("x의 값은 {x}입니다"); // 4

	{ // 새로운 스코프를 열 수 있음
		let x = x * 2;
		println!("x의 값은 {x}입니다"); // 8
	}

	println!("x의 값은 {x}입니다"); // 4
	// 새로운 스코프에서 만들어진 값은 끝나고 기존의 스코프에서 가지고 있던 값이 다시 나옴
}
```

- 같은 스코프 내에서 동일한 변수명을 다시 선언할 수 있고 새로 선언된 변수가 기존의 변수를 shadowing 하면서 가리게 됨
- 스코프를 생성해서 상위 스코프에서 선언된 변수값을 가져올 수 있고, shadowing 역시 가능
- 하위 스코프가 종료되면 아랫줄에서는 원래 스코프에서 할당된 값을 그대로 가져오게 됨.
