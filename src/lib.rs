use std::collections::HashMap;


mod decoder;

#[derive(PartialEq , Debug, Clone)]
pub enum BencodingResult {
	Str(String),
	Int(i64),
	List(Vec<BencodingResult>),
	Dict(HashMap<String, BencodingResult>),
	Bytes(Vec<u8>)
}


#[cfg(test)]
mod tests {
	use ::BencodingResult;
	use decoder::{decode, decode_bytes};
	use std::collections::HashMap;

	#[test]
	fn test_string() {
		assert_eq!(decode("4:spam"), Ok(BencodingResult::Str(String::from("spam"))));
		assert_eq!(decode("3:ifer"), Ok(BencodingResult::Str(String::from("ife"))));

		assert_eq!(decode("3:s"), Err("Decoding Error: not enough characters"));

		assert_eq!(decode_bytes(convert_str_to_vec_u8("4:spam")), Ok(BencodingResult::Bytes(convert_str_to_vec_u8("spam"))));
		assert_eq!(decode_bytes(convert_str_to_vec_u8("3:ifer")), Ok(BencodingResult::Bytes(convert_str_to_vec_u8("ife"))));

	}

	fn convert_str_to_vec_u8(string: &str) -> Vec<u8> {
		string.bytes().collect::<Vec<u8>>()
	}

	#[test]
	fn test_int() {
		assert_eq!(decode_bytes(convert_str_to_vec_u8("i24e")), Ok(BencodingResult::Int(24)));
		assert_eq!(decode_bytes(convert_str_to_vec_u8("i0e")), Ok(BencodingResult::Int(0)));
		assert_eq!(decode_bytes(convert_str_to_vec_u8("i-3e")), Ok(BencodingResult::Int(-3)));
		assert_eq!(decode_bytes(convert_str_to_vec_u8("i-42e")), Ok(BencodingResult::Int(-42)));

		assert_eq!(decode_bytes(convert_str_to_vec_u8("i2n4e")), Err("parse error: not a number"));
		assert_eq!(decode_bytes(convert_str_to_vec_u8("i-e")), Err("parse error: not a number"));
		assert_eq!(decode_bytes(convert_str_to_vec_u8("ie")), Err("Empty number"));
		assert_eq!(decode_bytes(convert_str_to_vec_u8("i03e")), Err("Number starts with 0"));
		assert_eq!(decode_bytes(convert_str_to_vec_u8("i003e")), Err("Number starts with 0"));
		assert_eq!(decode_bytes(convert_str_to_vec_u8("i-0e")), Err("Number -0 not valid"));
		assert_eq!(decode_bytes(convert_str_to_vec_u8("i23")), Err("integer decoding error:did not find 'e'"));
	}

	#[test]
	fn test_list() {
		assert_eq!(decode_bytes(convert_str_to_vec_u8("le")), Ok(BencodingResult::List(vec![])));

		let bencode_str_bytes:BencodingResult = BencodingResult::Bytes("spam".bytes().collect::<Vec<u8>>());
		assert_eq!(decode_bytes(String::from("l4:spame").into_bytes()), Ok(BencodingResult::List(vec![bencode_str_bytes])));

		let bencode_byte_spam:BencodingResult = BencodingResult::Bytes(convert_str_to_vec_u8("spam"));
		let bencode_int_24:BencodingResult = BencodingResult::Int(24);
		let bencode_int_35:BencodingResult = BencodingResult::Int(35);
		let bencode_byte_wat = BencodingResult::Bytes(convert_str_to_vec_u8("wat"));
		assert_eq!(decode_bytes(convert_str_to_vec_u8("l4:spami24e3:wati35ee")), Ok(BencodingResult::List(vec![bencode_byte_spam, 
														bencode_int_24, bencode_byte_wat, bencode_int_35])));
	}

	#[test]
	fn test_dict() {
		assert_eq!(decode_bytes(convert_str_to_vec_u8("de")), Ok(BencodingResult::Dict(HashMap::new())));

		let bencode_int_24 = BencodingResult::Int(24);

		let mut map_bytes:HashMap<String, BencodingResult> = HashMap::new();
		map_bytes.insert(String::from("spam"), bencode_int_24);
		assert_eq!(decode_bytes(String::from("d4:spami24ee").into_bytes()), Ok(BencodingResult::Dict(map_bytes)));

		assert_eq!(decode("d4:spami24e"), Err("end of input"));

		let bencode_str_a:BencodingResult = BencodingResult::Bytes(convert_str_to_vec_u8("a"));
		let becode_str_bee:BencodingResult = BencodingResult::Bytes(convert_str_to_vec_u8("b"));

		let mut map: HashMap<String, BencodingResult> = HashMap::new();
		map.insert(String::from("spam"), BencodingResult::List(vec![bencode_str_a , becode_str_bee]));
		assert_eq!(decode_bytes(convert_str_to_vec_u8("d4:spaml1:a1:bee")), Ok(BencodingResult::Dict(map)));
	}

}


