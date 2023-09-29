# 1. Rust 설치하기

[rustup](https://rustup.rs/)

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env" # install 후 이거 실행하라고 안내해줌 : 안하면 rustup command not found로 뜸

# 설치확인
rustup --version # 컴파일러 및 패키지관리자 등 한번에 설치해주는 애
rustc --version # 컴파일러 설치 확인
```

# 2. Hello, world!

VSC 실행 후 확장프로그램 : rust 검색 후 설치

- rust-analyzer
- Rust syntax
  설치됨.

1. main.rs 작성하기

```rust
fn main() {
	println!("hello, world!");
}
```

2. main.rs 파일 생성 후 컴파일

```shell
rustc main.rs
```

main 이라는 컴파일된 파일이 생성되는 것 확인

3. 컴파일된 파일 실행

```shell
./main
```

Hello, world 가 찍히는 것 확인

# 3. Rust의 패키지 관리자 Cargo 사용하기

설치 확인

```shell
cargo --version
```

cargo에서 새 프로젝트 생성하기

```shell
cargo new hello_cargo
cd hello_cargo
```

cargo로 프로젝트 빌드하기

```shell
cargo build
```

cargo로 프로젝트 실행하기

```shell
cargo run # 빌드와 실행을 한번에!
```

> **❗️Cargo를 이용해 프로젝트 생성 시, git 연결 및 src폴더와 빌드 후 target 폴더 생성 등이 자동으로 됨**

전체 설치 제거

```shell
rustup self uninstall
# Rust 관련 모든 툴 삭제
```
