fn main() {
    let mut nom;//Con mut se puedee hacer que una variable sea mutable, es decir, se puede modificar
    let mut ape;//Variable inmutable es decir, no se puede modificar
    nom = " Valentín";
    ape = " Cassino";
    println!("Hello, world! here remplace with the first argument Nombre:{} and the second argument Apellido:{}",nom,ape);
    //los corchetes sirven como parametros para tomar las variables
    let number: u32 = 5;
    let number2: u32 = 10;
    println!("La multiplicacion de 5x10 es: {}", number * number2);
    //todo!("Display this message println!('Hello, world!')");    
    /*los numeros tipo i32 o i64 es para la arquitectura de sistema y la i significa que pueden ser negativoso
    positivos, los numeros tipo u32 o u64 la u significa que son solo numeros positivos, para puntos flotantes se tiene
    f32 y f64   insize o unsize es dependiendo de la arquitectura de donde trabajamos*/ 
    //Booleanos
    let mut tipoBool = 2 > 10;
    let mut checkEmote  = "❌";//En rust es posible usar emoticones
    println!("El resultado de 2 > 10 es: {} {}", tipoBool, checkEmote);
    checkEmote = "✅";
    tipoBool = 2 < 10;
    println!("El resultado de 2 < 10 es: {} {}", tipoBool, checkEmote);
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    println!("Yo digo: {} en español",hello);
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    println!("Yo digo: {} en español",hello)
  
    

}
