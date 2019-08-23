struct Color (u8, u8, u8);
struct Kilometers(i32);

enum _Day {
	Sunday,
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday
}

enum FlashMessage {
	Success,
	Warning { category: i32, message: String },
	Error(String)
}

fn main() {
	let black = Color(0, 0, 0);

	let Color(r, g, b) = black;

	println!("Black = rgb({}, {}, {})", r, g, b);

	let distance = Kilometers(20);

	let Kilometers(distance_in_km) = distance;
	println!("The distance: {} km", distance_in_km);


	let mut form_status = FlashMessage::Success;
	print_flash_message(form_status);

	form_status = FlashMessage::Warning { category: 2, message: String::from("Field X is required") };
	print_flash_message(form_status);

	form_status = FlashMessage::Error(String::from("Connection Error"));
	print_flash_message(form_status);

}

fn print_flash_message(m: FlashMessage) {
	match m {
	    FlashMessage::Success => 
	    	println!("Form Submitted correctly"),
	    FlashMessage::Warning { category, message} =>
	    	println!("Warning : {} - {}", category, message),
	    FlashMessage::Error(msg) =>
	    	println!("Error : {}", msg)
	}
}