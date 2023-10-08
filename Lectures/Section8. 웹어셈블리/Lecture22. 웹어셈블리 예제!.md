
[참고 링크](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm)

# 웹어셈블리 기초

wasm-pack 설치
```shell
cargo install wasm-pack

wasm-pack --version
```
웹어셈블리에 필요한 빌드툴 : 컴파일, 에셋묶기 등..  (webpack 같은 느낌) : 웹어셈블리 프로젝트에 필요한 툴 사용할 수 있게함.



```
wasm-pack new [프로젝트명]
```
프로젝트 초기화
- cargo.toml
- lib.rs <- 빌드하려는 wasm 바이너리의 기본 파일

1. wasm_bindgen
자바스크립트랑 rust 소통 도와주는 패키지 
wasm-pack으로 초기화하면 기본적으로 디펜던시 추가되어있음
```rust
#[wasm_bindgen]
```
으로 디렉티브 처리 -> 컴파일러에게 아래의 코드가 rust-js 간의 소통을 위해 필요한 코드임을 알려줌


lib.rs
```rust
#[wasm_bindgen] // 다음의 항목은 rust와 wasm 소통을 위한 코드임
extern "C" { // 외부함수를 쓰겠다.
	fn alert(s: &str); // 외부함수 : rust - 난모르는데아무튼어디에있음
	// -> 브라우저에 있는 alert 함수라고 생각하면 됨!
}

#[wasm_bindgen]
pub fn greet() { // pub이 붙으면 외부에서(js로) 러스트 함수 호출이 가능
	alert("Hello, my-wasm!");
}
```
이 코드는 JS로 rust의 greet 함수 호출 -> rust로 JS의 alert함수 호출 하는...
굳이 rust로 왔다간 그런 코드임. 아무튼 상호간에 이렇게 서로의 함수를 사용할 수 있음.

JS로 노출할 필요 없는 함수는 일반 러스트 코드로 작성하고, JS에서 호출할 함수에만 wasm_bindgen을 붙여서 구성하면 된다.

```
wasm-pack build --target web // 웹에서 사용할 wasm 빌드
wasm-pack build --target nodejs // nodejs환경에서 사용할 wasm 빌드
```
위와 같이 사용할 환경에 따라 target을 지정하여 wasm 바이너리를 빌드할 수 있다.
빌드의 결과물로 생성되는 pkg 디렉토리를 확인해보면 여러가지 파일이 있는데
그 중
- [프로젝트명].wasm
- [프로젝트명].js
파일이 생성된 것을 확인할 수 있다.


=> 일반적으로는 npm package로 생성을 해서 js에서 쉽게 불러와 활용하도록 wasm을 이용하게 된다.