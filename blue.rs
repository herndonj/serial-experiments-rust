fn main () {


let mut bv: Vec<(&str, &str, &str, f64, &str)> = Vec::new();

	bv.push(("2023-12-03", "Starbucks", "Food:Take Away", -4.34, ""));

	println!("This is from the vector: {}", &bv[1].1);
	
}		