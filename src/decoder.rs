use std::iter::Peekable;
use std::num::ParseIntError;
use std::collections::HashMap;
use super::BencodingResult;
use std::vec::IntoIter;

pub fn decode(input:Vec<u8>) -> Result<BencodingResult, &'static str> {
	let mut peekable_input:Peekable<IntoIter<u8>> = input.into_iter().peekable();

	if peekable_input.peek().is_none()  {
		return Err("End of input")
	}

	return decode_peekable(&mut peekable_input)
}

fn decode_string(peekable_input: &mut Peekable<IntoIter<u8>>) -> Result<Option<Vec<u8>>, &'static str> {
	let next_char:char = *(peekable_input.peek().unwrap()) as char;
	if !next_char.is_numeric() {
		return Ok(None)
	}

	let count:usize = next_char.to_digit(10).unwrap() as usize;
	peekable_input.next();

	if peekable_input.peek().is_none() {
		return Err("unable to decode string invalid bencoded string")
	}

	let next_char:char = *(peekable_input.peek().unwrap()) as char;
	if next_char != ':' {
		println!("decode string next_char {}", next_char);
		return Err(": does not exist after number invalid bencoded string after count {}")
	}
	peekable_input.next();

	let result_string:Vec<u8> = (peekable_input.take(count)).collect();
	if result_string.len() != count {
		return Err("Decoding Error: not enough characters")
	}

	return Ok(Some(result_string));
}


fn decode_int(peekable_input: &mut Peekable<IntoIter<u8>>) -> Result<Option<i64>, &'static str> {
	let next_char:char = *(peekable_input.peek().unwrap()) as char;
	if next_char != 'i' {
		return Ok(None)
	}

	peekable_input.next();

	let mut result_string:String = String::new();

	loop {
		let new_value:char;
		{
			let next_value_option = peekable_input.peek();
			if let Some(next_value) = next_value_option {
				new_value = *next_value as char;
			} else {
				return Err("integer decoding error:did not find 'e'")
			}
		}			

		peekable_input.next();
		if new_value != 'e' {
			result_string.push(new_value);
		} else {
			break;
		}
	}

	let error:Option<&'static str> = is_valid_format(&result_string);

	if let Some(err) = error {
		return Err(err)	
	}

	let result:Result<i64, ParseIntError> =  result_string.parse::<i64>();
	if result.is_err() {
		return Err("parse error: not a number")
	}

	return Ok(Some(result.unwrap()))
}

fn is_valid_format(string: &str) -> Option<&'static str> {
	if string.is_empty() {
		return Some("Empty number")
	}

	if string.len() > 1 && string.starts_with("0") {
		return Some("Number starts with 0")
	}

	if string.len() >1 && string.starts_with("-") && string.chars().nth(1).unwrap() == '0' {
		return Some("Number -0 not valid")
	}

	return None
}

fn decode_list(peekable_input: &mut Peekable<IntoIter<u8>>) -> Result<Option<Vec<BencodingResult>>, &'static str> {
	if let Some(next_char) = peekable_input.peek() {
		if *next_char as char != 'l' {
			return Ok(None)
		}
	}
	peekable_input.next();

	let mut bencoded_list:Vec<BencodingResult> = Vec::new();

	loop {

		let mut is_e:bool = false;
		if let Some(next_char) = peekable_input.peek() {
			if *next_char as char == 'e' {
				is_e = true;
			}
		}

		if is_e {
			peekable_input.next();
			return Ok(Some(bencoded_list))
		}

		let next_value:Result<BencodingResult, &str>  = decode_peekable(peekable_input);
		if let Ok(result) = next_value {
			bencoded_list.push(result)
		} else {
			return Err(next_value.err().unwrap())
		}

	}
}


fn decode_peekable(peekable_input:& mut Peekable<IntoIter<u8>>) -> Result<BencodingResult, &'static str> {
	let result:Result<Option<Vec<u8>>, &str> = decode_string(peekable_input);

	if let Err(decode_string_error) = result {
		return Err(decode_string_error)
	}

	if let Ok(Some(decoded_string)) = result {
		return Ok(BencodingResult::ByteString(decoded_string))
	}

	let result:Result<Option<i64>, &str> = decode_int(peekable_input);

	if let Err(decode_int_error) = result {
		return Err(decode_int_error)
	}

	if let Ok(Some(int_result)) = result {
		return Ok(BencodingResult::Int(int_result))
	}

	let result:Result<Option<Vec<BencodingResult>>, &str> = decode_list(peekable_input);
	if let Err(decode_list_error) = result {
		return Err(decode_list_error)
	}

	if let Ok(Some(list_result)) = result {
		return Ok(BencodingResult::List(list_result))
	}

	let result:Result<Option<HashMap<String, BencodingResult>>, &str> = decode_map(peekable_input);
	if let Err(decode_map_error) = result {
		return Err(decode_map_error)
	}

	if let Ok(Some(map_result)) = result {
		return Ok(BencodingResult::Dict(map_result))
	}
	return Err("general error")
}


fn decode_map(peekable_input: &mut Peekable<IntoIter<u8>>) -> 
					Result<Option<HashMap<String, BencodingResult>>, &'static str> {
	if let Some(next_char) = peekable_input.peek() {
		if *next_char as char != 'd' {
			return Ok(None)
		}
	}

	peekable_input.next();
	let mut bencoded_dict:HashMap<String, BencodingResult> = HashMap::new();

	loop {
		let mut is_e:bool = false;
		if let Some(next_char) = peekable_input.peek() {
			if *next_char as char == 'e' {
				is_e = true;
			}
		} else {
			return Err("end of input")
		}
		if is_e {
			peekable_input.next();
			return Ok(Some(bencoded_dict))
		}

		let mut key:String = String::new();

		if let Ok(Some(decoded_string)) = decode_string(peekable_input) {
			if let Ok(bencoded_string) = String::from_utf8(decoded_string) {
				key = bencoded_string;
			} else {
				return Err("Key is not valid string");
			}
		}

		let result: Result<BencodingResult, &str> = decode_peekable(peekable_input);
		if let Ok(bencoded_result) = result {
			bencoded_dict.insert(key, bencoded_result);
		} else {
			return Err(result.err().unwrap())
		}

	}
}




