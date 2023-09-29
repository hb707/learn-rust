# 1. 프로젝트 세팅하기

```shell
cargo new rock-paper-scissors
cd rock-paper-scissors
```

# 2. 소스코드

```rust
use std::io; // 패키지 임포트

fn main() {
	println!("[가위, 바위, 보] 중 하나를 입력해주세요!");

	let mut decision = String::new(); // 변수 선언

	io::stdin().read_line(&mut decision).expect("입력실패"); // 문자열 받기

	println!("당신의 선택 : {decision}")
}
```

기본 패키지인 io 패키지를 사용해서 커맨드라인에서 입력받은 값을 변수에 저장할 수 있다
