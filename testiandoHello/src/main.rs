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
    println!("Yo digo: {} en español",hello);
    //Generate a tuple una tupla es un conjunto de elemento como un array
    let generacionOfTubpla = ("Hola","Mi nombre es", "Valentin", "y tengo", 18i32, "años", "this is ",true);
    println!("{} {} {} {} {} {} {} {}",generacionOfTubpla.0,generacionOfTubpla.1,generacionOfTubpla.2,generacionOfTubpla.3,generacionOfTubpla.4,generacionOfTubpla.5,generacionOfTubpla.6,generacionOfTubpla.7);
    //El struct clasico es como un tipo de keyMap por cada campo le corresponde un tipo de campo deseado
    struct enemigo {name: String, level: u8, strong: bool}
    //Tuple Struct es similar al de arriba pero la diferencia es que este no posee nombre de campo se indexa
    struct typeOfEnemies (String, String, String, f32);
    //Unic struct se usa como marcadores en el tutorial de microsoft aun no explica como funciona
    struct unicStruct;
    //Para instanciar los campos de un struct clasic usamos el nombre del struct dos puntos y insertamos el tipo de dato solicitado
    //todo esto lo almacenamos en una variable let ejeplo a continuacion.
    let termineitor = enemigo {name: String::from("TermineitorR14"), level: 27, strong: true};
    //Ahora crearemos el struct tuple que contiene solo los datos sin un campo key
    let malo = typeOfEnemies(String::from("Es malo"), String::from("Tiene fuerza bruta"), String::from("Ojo bionico"), 1.75);
    //Ahora pasare a imprimir los datos
    println!("------------DATA------------");
    println!("DATOS DEL ENEMIGO: ");
    println!("NOMBRE: {} NIVEL: {} ES FUERTE?: {}", termineitor.name, termineitor.level, termineitor.strong);
    println!();
    println!("CARACTERISTICAS DEL ENEMIGO:");
    println!("{}: {} {}: {} {}: {} {} mide: {}",termineitor.name,malo.0,termineitor.name,malo.1,termineitor.name,malo.2,termineitor.name,malo.3);
   //Como ves el String::from se usa para convertir el str a Typo String
}
