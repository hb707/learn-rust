trait Greet {
	fn greeting(&self) -> String;
}
enum Pet {
	Dog,
	Cat,
	Hamster
}

impl Greet for Pet {
	fn greeting(&self) -> String {
		match (self) {
			Pet::Dog => String::from("멍멍"),
			Pet::Cat => String::from("야옹"),
			Pet::Hamster => String::from("찍찍"),
		}
	}
}

struct Person {
	name: String,
	active: bool,
}

impl Greet for Person {
	fn greeting(&self) -> String {
		String::from("안녕")
	}
}

fn meet<T: Greet> (one:&T, another: &T) { // greet 트레이트를 구현한 같은 타입을 가지는 one과 another
	println!("첫번째가 인사합니다 : {}", one.greeting());
	println!("두번째가 인사합니다 : {}", another.greeting());
	// 파라미터의 타입 지정 시 트레이트를 전달했으므로 두가지 파라미터 모두 greeting이라는 메소드를 가지고 있는것을 알 수 있음.
}

fn main() {
	let cat = Pet::Cat;
	let gildong = Person {
		name: String::from("홍길동"),
		active: true,
	};
	// meet(&cat, &gildong); // ❌ 타입이 다르므로 불가

	let ham = Pet::Hamster;
	meet(&cat, &ham); // ⭕️ 타입까지 같으므로 가능
}