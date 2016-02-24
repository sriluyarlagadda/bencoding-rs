use std::collections::HashMap;


mod decoder;



#[derive(PartialEq , Debug)]
pub enum BencodingResult {
	Str(String),
	Int(i64),
	List(Vec<BencodingResult>),
	Dict(HashMap<String, BencodingResult>)
}


#[cfg(test)]
mod tests {
	use ::BencodingResult;
	use decoder::{decode};
	use std::collections::HashMap;

	#[test]
	fn test_string() {
		assert_eq!(decode("4:spam"), Ok(BencodingResult::Str(String::from("spam"))));
		assert_eq!(decode("3:ifer"), Ok(BencodingResult::Str(String::from("ife"))));

		assert_eq!(decode("3:s"), Err("Decoding Error: not enough characters"));
	}

	#[test]
	fn test_int() {
		assert_eq!(decode("i24e"), Ok(BencodingResult::Int(24)));
		assert_eq!(decode("i0e"), Ok(BencodingResult::Int(0)));
		assert_eq!(decode("i-3e"), Ok(BencodingResult::Int(-3)));
		assert_eq!(decode("i-42e"), Ok(BencodingResult::Int(-42)));

		assert_eq!(decode("i2n4e"), Err("parse error: not a number"));
		assert_eq!(decode("i-e"), Err("parse error: not a number"));
		assert_eq!(decode("ie"), Err("Empty number"));
		assert_eq!(decode("i03e"), Err("Number starts with 0"));
		assert_eq!(decode("i003e"), Err("Number starts with 0"));
		assert_eq!(decode("i-0e"), Err("Number -0 not valid"));
		assert_eq!(decode("i23"), Err("integer decoding error:did not find 'e'"));
	}

	#[test]
	fn test_list() {
		assert_eq!(decode("le"), Ok(BencodingResult::List(vec![])));

		let bencode_str:BencodingResult = BencodingResult::Str(String::from("spam"));
		assert_eq!(decode("l4:spame"), Ok(BencodingResult::List(vec![bencode_str])));

		let bencode_str_spam:BencodingResult = BencodingResult::Str(String::from("spam"));
		let bencode_int_24:BencodingResult = BencodingResult::Int(24);
		let bencode_int_35:BencodingResult = BencodingResult::Int(35);
		let bencode_str_wat = BencodingResult::Str(String::from("wat"));
		assert_eq!(decode("l4:spami24e3:wati35ee"), Ok(BencodingResult::List(vec![bencode_str_spam, 
														bencode_int_24, bencode_str_wat, bencode_int_35])));
	}

	#[test]
	fn test_dict() {
		assert_eq!(decode("de"), Ok(BencodingResult::Dict(HashMap::new())));

		let bencode_int_24 = BencodingResult::Int(24);
		let mut map: HashMap<String, BencodingResult> = HashMap::new();
		map.insert(String::from("spam"), bencode_int_24);
		assert_eq!(decode("d4:spami24ee"), Ok(BencodingResult::Dict(map)));

		assert_eq!(decode("d4:spami24e"), Err("end of input"));

		let bencode_str_a:BencodingResult = BencodingResult::Str(String::from("a"));
		let becode_str_bee:BencodingResult = BencodingResult::Str(String::from("b"));

		let mut map: HashMap<String, BencodingResult> = HashMap::new();
		map.insert(String::from("spam"), BencodingResult::List(vec![bencode_str_a , becode_str_bee]));
		assert_eq!(decode("d4:spaml1:a1:bee"), Ok(BencodingResult::Dict(map)));
	}

}


