
# 에러의 종류

## 1. 복구 가능 에러
- File not found -> 다른 경로를 입력받아 재시도
- 프로그래머가 에러 상황에 따른 처리를 지정해줌

> Result<T, E>

### 복구가능한 오류 Result<T, E>
#### 📍사용하기
```rust
enum Result<T, E> {
	Ok(T),
	Err(E)
} // 기본 내장 라이브러리에 포함되어 직접 선언 불필요
```
OK와 Err의 경우에 대한 타입을 담을 수 있는 enum 타입

```rust
use std::fs::File;

fn main() {
	let file: Result<File, std::io::Error> = File::open("hello.txt");
	// File에 있는 open메소드 사용 시 Result 타입을 반환 : 정상/에러 각각의 경우 처리 가능
	match file {
		Ok(f) => println!("파일을 잘 열었음"),
		Err(e) => panic!("파일 열기 실패")
	}
}
```

#### 📍에러 종류에 따른 처리도 가능 : kind() 함수
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
	let file: Result<File, std::io::Error> = File::open("hello.txt");
	// File에 있는 open메소드 사용 시 Result 타입을 반환 : 정상/에러 각각의 경우 처리 가능
	match file {
		Ok(f) => println!("파일을 잘 열었음"),
		Err(e) => match e.kind() {
			// kind() : 에러의 종류에 대한 이넘을 반환해주는 함수
			// 이값을 이용해서 에러의 종류에 따른 처리를 해줄 수 있음
			// ex) NotFound에러라면 해당 파일명을 가지는 파일을 생성, 나머지 경우는 panic 일으키기
			ErrorKind::NotFound => File::create("hello.txt"),
			_ => panic!("파일 접근 실패")
		}
	}
}
```


#### 📍unwrap(), expect()
매번 match 구문으로 파일을 추출하기 번거로움.
손쉽게 원하는 파일만 추출해 결과값으로 받고 싶을 때 사용
- unwrap() : 파일이 있으면 리턴, 없으면 panic! -> 단순 테스트코드에 쓰기 편함
- expect() : 파일이 있으면 리턴, 없으면 panic!과 메시지 출력 -> 디버깅 편리!

```rust
fn main() {
	let file: File = File::open("hello.txt").unwrap();
	let file2: File = File::open("hello.txt").expect("파일을 열 수 없음")
}
```


## 2. 복구 불가능 에러
- 배열의 범위 밖 접근 (명백한 프로그래머의 실수)
- 프로그램이 더이상 진행되지 않고 종료됨

>panic! 매크로
>- 배열의 범위 밖을 접근하는 등의 상황
>- 의도적으로 특정 경우에 panic! 매크로를 사용한 경우 (절대 일어나서는 안되는 상황임을 명시)


### panic!으로 강제종료

```rust
panic!("~~에러")
```
=> 이렇게 하면 프로그램 종료되며 인자값으로 넣은 string 출력

**디버깅을 위한 자세한 에러 내역 출력**
```shell
$ RUST_BACKTRACE=1
```
위와 같이 환경변수 설정 후 다시 panic! 매크로로 강제종료 시 panic 발생까지의 호출 스택을 전부 보여줌
