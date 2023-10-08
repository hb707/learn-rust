# 에러의 전파

함수1 -> 함수2호출 -> 함수3호출 에서 함수3에서 에러가 발생한 경우
내부 호출에서 발생한 에러는 단순히 리턴만 된 후, 함수1에서 발생한 에러에 따른 처리를 해주는 것이 일반적이다.
이렇게 상위 함수에 에러를 전파하는 방법

## Result<String, Error>을 리턴값 타입으로 사용
```rust
// 파일에서부터 string 값을 읽어오는 함수
fn read_username() -> Result<String, Error> {
	let file_result = File::open("users.txt");

	// 파일 열기에서 에러 발생시 전파
	let mut file = match file_result {
		Ok(file) => file,
		Err(e) => return Err(e)
	}

	let mut username = String::new();

	// 파일 읽기에서 에러 발생시 전파
	match file.read_to_string(&mut username) {
		Ok(_) => Ok(username),
		Err(e) => Err(e)
	}
}
```


## 에러전파 축약표현 : ?
일반적으로는 위 코드의 축약표현을 사용하게 된다.
```rust
// 파일에서부터 string 값을 읽어오는 함수
fn read_username() -> Result<String, Error> {
	// mut 키워드를 붙인뒤 뒤에 ?를 붙이기
	let mut file = File::open("users.txt")?;

	//
	let mut username = String::new();
	file.read_to_string(&mut username)?; // username변수에 읽어온 값 담기
	Ok(username) // 에러가 발생하지 않은 경우에만 이 라인으로 넘어오게됨. read_username 함수의 최종 리턴값 적어줌(;가 빠진!)
}
```
에러 핸들링까지 완료된 값을 가지고 오고 싶을 때는,
가져올 대상의 변수에 mut 키워드를 붙이고 + 메소드 호출 후 ?를 붙인다.
?를 붙이면 에러가 발생하면 에러를 리턴하고, 정상처리 시에는 해당 값을 변수에 담게 된다.

## 에러전파 축약표현2
위에서 파일 열기 - 파일 읽기를 따로 에러핸들링해줬다면,
둘을 연결해서 체이닝하여 사용할 수도 있다.
```rust
// 파일에서부터 string 값을 읽어오는 함수
fn read_username() -> Result<String, Error> {
	let mut username = String::new();
	File::open("users.txt")?.read_to_string(&mut username)?;
	Ok(username)
}
```

## 에러전파 축약표현3
위 내용은 사실 std::fs 패키지에서 제공하고 있음
```rust
use std::fs;

fn read_username() -> Result<String, Error> {
	fs::read_to_string("users.txt")
}
```


# 에러 다루기 정리
- 복구 불가능한 에러 -> panic!
- 복구 가능한 에러 Result<T, E>
- unwrap() 또는 expect()로 결과값 쓰거나 panic! 하기
- ? 문법으로 에러 전파 간결히 표현