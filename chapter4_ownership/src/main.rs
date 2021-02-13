use std::rc::Rc;

fn print_padovan(){
    let mut padovan = vec![1,1,1];  // padovan es OWNER del vector en heap
    padovan.push(1);
    println!("Ownership\n\tpadovan vector: {:?}", padovan);
}

fn main() {

    // Simple Example Owner
    print_padovan();
    //padovan.push(1);  // La variable no puede usarse de nuevo primero porque fue declarada en la función y segundo porque ya fue dropped cuando finalizó la función.


    //MOVES -- Cuando hacemos un move lo que estamos haciendo es cambiar el ownership de una variable 
    // a otra que queda no inicializada. Sin embargo esto solo pasa en vectores, strings y en 
    // otros tipos mas complejos, no en integers, y/o otros tipos simples.
    let string_1 = "Este es un String Vector".to_string();
    let string_2 = string_1;
    println!("Moves\n\tstring_2 ahora contiene el ownership de la cadena -> '{:?}' y string_1 queda no inicializado", string_2);
    // println!("Moves\n\tstring_1 YA NO contiene el ownership de la cadena -> '{:?}'", string_1);  //bad: string_1 ya no tiene el ownership de la cadena porque se lo pasó a string_2


    // Rc and Arc -- Cuando necesitamos varios punteros hacia un valor podemos usar Rc
    // sin embargo hay que aclarar que no se puede modificar el valor, solo leer, el tipo Rc<T> es INMUTABLE.
    // Esto se hace CLONANDO el Rc<String>
    let s:Rc<String> = Rc::new("Shirataki".to_string());
    let t:Rc<String> = s.clone();
    let u = s.clone();    // Rust infiere el tipo
    println!("Rc and Arc\n\ts -> {:?}, s -> {:?}, s -> {:?} -- Vemos que todos apuntan a la misma cadena", s,t,u);

}



