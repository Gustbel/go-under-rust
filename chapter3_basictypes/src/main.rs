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
	let i_33 = 5.;		// Aunque 5 sea un entero si queremos definirlo como float debemos poner . o e0 para que Rust infiera que hablamos de un float y no un integer
	println!("\nFloats:\n\tDeclar1: {}\tDeclar2: {}\tDeclar3: {}", i_31, i_32, i_33);

	// Constants
	let i_34:f64 = std::f64::INFINITY;
	let i_35:f64 = std::f64::NEG_INFINITY;
	let i_36:f32 = std::f32::NAN;
	let i_37:f32 = std::f32::EPSILON;
	let i_38 = 5f64/i_35;
	println!("\n\tInfinito: {}\tInfinitoNeg: {}\tValorNulo: {}\tEpsilon: {}\t5/infinito: {}", i_34, i_35, i_36, i_37, i_38);

    // Methods
    let i_39 = (25.65_f32.floor()).sqrt();
    println!("\n\tRaiz Cuadrada de floor(25.65)=25 -> {}", i_39);

// Bools
	// Declaration
	let is_enable:bool = true; // Se define un booleano

	// Common uses
	  // Comparators like == , != or > return bools
	println!("\nBools:\n\t2 == (4-2)  ->  {}\t2 > 8  ->  {}\tand the value is_enable is {}", 2 == (4-2), 2 > 8, is_enable);

// Characters
	// Declaration
	let c_1:char = 'A'; // Caracter literal UNICODE, ocupa 32 bits
	let c_2 = 'Ñ'; // Caracter literal UNICODE, ocupa 32 bits
	let c_3:char = '\x2A'; // Escrito en HEXA pero usando Tabla ASCII
	let c_4:char = '\u{CA0}'; // Escrito en HEXA pero usando Tabla UNICODE
	println!("\nCharacters:\n\tChar1: {}\tChar2: {}\tChar3(hexaASCII): {}\tChar4(hexaUnicode): {}", c_1, c_2, c_3, c_4);

	//Methods
	println!("\t'*'.is_alphabetic() = {}\t'ß'.is_alphabetic() = {}", '*'.is_alphabetic(), 'ß'.is_alphabetic());

//Tuples
	// Declaration
	let t_1 = ("Argentina", 1978, 1986); // (&str, i32, i32)
	println!("\nTuple:\n\tValor1:{}\tValor2:{}\tValor3:{}", t_1.0, t_1.1, t_1.2);

	// Rust devuelve tuplas a ciertas funciones por ejemplo la siguiente
	// recibe un string literal y lo corta en 2 creando unta tupla (&str, &str)

	let string = "Hola Juan. Que cuentas?";
	let t_2 = string.split_at(11);	// Esta función divide en la posición indicada
	println!("\tOriginal: '{}'\t\tTuple-> Element1:'{}'\tElement2:'{}'", string, t_2.0, t_2.1);

//Pointer Types
println!("\nPointer Types:");
  // References:
	// Declaration
	let str_1 = "Esto es un string literal";
	println!("  References:\n\tValor de str_1 = '{}' -- y la referencia de esta variable es &str_1", str_1);
  // Boxes:
	// Declaration
	let t = (12, "eggs");
	let boxes_1 = Box::new(t); // El tipo es Box<(i32, &str)>
	println!("  Boxes:\n\tBoxes es la forma más simple de guardar valores en heap. Por ejemplo la boxes_1 recien creada es una tupla y está en el heap: Box<({}:i32,{}:&str)>", boxes_1.0, boxes_1.1);
  // Raw Pointes:
	// Declaration
	println!("  Raw Pointers:\n\tExactamente iguales a los punteros de C/C++, no los explicaremos en este código");

//Arrays
println!("\nArrays:");
	// Declaration 
	let mut chaos : [u32;6] = [2,14,1,29,3,5];
	let taxonomy = ["Animalia", "Arthropoda","Insecta"];
	let sieve = [true;1000]; // Crea un Array de 1000 elementos bool ya inicializados en true.. esto puede hacerse con cualquier tipo
	println!("\t- De tamaño FIJO, son siempre del mismo tipo y se guardan en el Stack (no en heap)\n\
			\t   Valor de arrays: chaos = {:?}, taxonomy = {:?}, cant de element de sieve: {}", chaos, taxonomy, sieve.len());

	//Methods
	chaos.sort();
	println!("\tchaos.sort() = {:?}  // Metodo que ordena un array", chaos);

//Vector
	println!("\nVector:");
	println!("\t- De tamaño VARIABLE, son siempre del mismo tipo y se guardan en heap, lo que obtenemos en la variable es un puntero al dato en heap\n");
	// Declaration 
		let mut v = vec![2,3,5,7];
		println!("\tv = {:?}  -> (Vect<i32>)", v);
		v.push(11);
		v.push(24);
		//v.push("Hola"); Da error porque está esperando un Integer
		println!("\tv.push(11);\n\tv.push(24);\n\tv = {:?}  -> (Vect<i32>)", v);
		v.pop();
		println!("\tv.pop();\n\tv = {:?}  -> (Vect<i32>)", v);

		// Otra forma de definir Vectors pero acá no sabe la cantidad de elemenos iniciales
		let mut v_2 = Vec::new();
		v_2.push("dato1"); v_2.push("dato2");
		// Si sabemos la cantidad máxima aproximada de elementos del Vector es mejor definir de esta forma:
		let mut v_3 = Vec::with_capacity(2);
		v_3.push("dato1"); v_3.push("dato2");

	//Methods
		v.insert(3,100);
		println!("\tv.insert(3,100);\n\tv = {:?}  -> (Vect<i32>)", v);
		v.remove(0);
		println!("\tv.remove(0));\n\tv = {:?}  -> (Vect<i32>)", v);
	
	// in a for block
	println!("\n\tCiclo FOR de v");
		for i in v {
			println!("\t\tElemento impreso en for: {}", i);
		}
	
//Slice
	println!("\nSlice:");
	println!("\t- Los Slices siempre guardan Referencias (no owning) de un array o un Vector\n\
			\tEn el ejemplo anterior del vector v, un slice de v sería:\n\t\tlet sv:&[i32] = &v;");



// String Types
	// Declaration
	// String Literal   -> Son inmutables y de tamaño fijo
	let string_1 = "Este es un String Literal";
	let string_2 = r"Este es un RAW String Literal"; //Esta forma de declaracion no permite escapes \ en la declaración, \ será reconocido como un caracter más
	let string_3 = b"GET"; //Este es un string byte, cada carácter es ASCII y ocupa 8bits y ES UN ARREGLO
	
	// Strings		-> Son análogos a un Vector y pueden agrandarse o achicarse tiene su propio buffer en heap
	let string_4 = "Este es un String Vector".to_string();

	println!("\nString Types:\n\tstr1:'{}'\tstr2:'{}'\tstr3:'{:?} (String Byte)'\tstr4:'{:?}'", string_1, string_2, string_3, string_4);

	// Methods   --> NO pueden usarse en los strings literales, solo en los Strings vectores
	let string_5 = "ONE".to_string();
	println!("\t\"ONE\".to_lowercase() = {}", string_5.to_lowercase());



}

