fn main() {

// Integers
		//Declaration
	let i_11 = -2; 		// Se declara sin tipo en especial, Rust infiere el tipo de dato
	let i_12:u8 = 2; 	// Se declara con tipo u8
	let i_13 = 2u8;		// Otraforma de declarar con tipo u8
	let i_14:u16 = 0x3A; // Metemos hexa
	let i_15:u8 = 0b_010_110_110; // Metemos binario de 1 byte, podemos agregar tantos _ como quisieramos al numero ya que Rust los ignora, sirve para ver código más claramente
	let i_16:usize = 0o2341237; // Metemos octal de tipo de dato tamaño de la arquitectura utilizada
	println!("\nIntegers:\n\tDeclar1: {}\tDeclar2: {}\tDeclar3: {}\tDeclar4: {}\tDeclar5: {}\tDeclar6: {}", i_11, i_12, i_13, i_14, i_15, i_16);

		//OVERFLOW
	//let a:u8 = 256;      overflow

	//let a:u8 = 255;
	//a = a +1;
    //println!("Esto no se muestra ya que es un overflow, {}!", a); SI SE PUEDE VER EN build --release
	let i_21:u8 = 255;
	let i_22 = i_21.wrapping_add(1);	// acá evita el overflow
    println!("\n\tEsto pasa cuando hacemos 255 + 1 = {} en una variable u8 utilizando wrapping_add(1) (NO OVERFLOW)", i_22);

// Floating Points
	// Declaration
	let i_31 = 31415.926e-4f64; // Forma mas detallada de declarar un float literal con punto y exponencial
	let i_32 = -1.56; // Tipo inferido por Rust
	let i_33 = 2.;		// Aunque 2 sea un entero si queremos definirlo como float debemos poner . o e0 para que Rust infiera que hablamos de un float y no un integer
	println!("\nFloats:\n\tDeclar1: {}\tDeclar2: {}\tDeclar3: {}", i_31, i_32, i_33);

	// Constants
	let i_34:f64 = std::f64::INFINITY;
	let i_35:f64 = std::f64::NEG_INFINITY;
	let i_36:f32 = std::f32::NAN;
	let i_37:f32 = std::f32::EPSILON;
	let i_38 = 5f64/i_35;
	println!("\n\tInfinito: {}\tInfinitoNeg: {}\tValorNulo: {}\tEpsilon: {}\t5/infinito: {}", i_34, i_35, i_36, i_37, i_38);

}

