enum IpAddrKind {
	V4,
	V6,
}

fn route(ip_kind: IpAddrKind) {}

fn values() {
	let four = IpAddrKind::V4;
	let six = IpAddrKind::V6;
}

struct IpAddr {
	kind: IpAddrKind,
	address: String,
}

fn main() {
	let home = IpAddr {
		kind: IpAddrKind::V4,
		address: String::from("127.0.0.1"),
	};
	
	let loopback = IpAddr {
		kind: IpAddrKind::V6,
		address: String::from("::1"),
	};
}


// We can represent the same concept in a more concise way
// using just an enum, rather than an enum inside a struct,
// by putting data directly into each enum variant

enum IpAddr2 {
	V4(u8, u8, u8, u8),
	V6(String),
}

fn consice() {
	let home = IpAddr2::V4(127, 0, 0, 1);
	let loopback = IpAddr2::V6(String::from("::1"));
}
