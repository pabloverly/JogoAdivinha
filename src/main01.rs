
//1 ESTRUTURA
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
//2 ESTRUTURA
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    //TOTAL DE CARACTERE
    //let spaces = "    ";
    //let spaces = spaces.len();
    //println!("{}", spaces);


    /*
    Tipos inteiros
    comprimento	Assinado	Sem sinal
    8 bits	    i8      	u8
    16 bits	    i16	        u16
    32 bits	    i32	        u32
    64 bits	    i64	        u64
    128 bits	i128	    u128
    arco	    isize	    usize
    */   
    
    
   // let x = 2.0; // f64
   // let y: f32 = 3.0; // f32
   // println!("{} {}", x, y)

    //let t = true;
   //  let f: bool = false;
   // println!("{} {}", t, f)

   // let c = 'z';
    //let z = 'ℤ';
    //let heart_eyed_cat = '😻';
    //println!("{}{}{}", c,z,heart_eyed_cat)

    //TRUPLA
   // let tup = (500, 6.4, 1);
    //let (x, y, z) = tup;
    //println!("The value of y is: {}", z)

    //MATRIZ
    //let a = [1, 2, 3, 4, 5];
    //let first = a[0];
    //let second = a[1];
    //println!("{}{}",first,second );
    


    //FUNCOES
   // another_function(5)


   //CORPO DE FUNCOES
   // let mut x = 5;
    // let y = {
    //    let x = 3;
    //    x + 1
    //};
    //println!("The value of y is: {}", y);

    //FUNCAO RETORNO SEM PARAMETRO
    //let x = funcaoRetornoSemParametro();    
    //println!("The value of x is: {}", x);    


    //FUNCAO RETORNO COM PARAMTRE
    //let x  = funcaoRetornoComParametro(10);
    //println!("{}", x );


    //CONDICOES
  /*  let number = 3;
    if number > 3 {
        println!("numero maior que 3");
    } else  if number == 3  {
        println!("numero igual a 3");
    }
    else {
        println!("fora condicao" );
    }
    */

   //CONDICAO  EM UMA DECLARACAO
    //let condition = true;
    //let number = if condition { 5 } else { 6 };
    //println!("{}",number );
    
    //loop infinito
    // loop {
    //     println!("again!");
    // }
    
    //LOOP COM BREACK
    /*let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("VALOR {}", counter );
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    */

    //while
    //let mut number = 3;
    //while number != 0 {
    //    println!("{}!", number);
    //    number -= 1;
    // }
    //println!("LIFTOFF!!!");


    //FOR
    //let a = [10, 20, 30, 40, 50];
    //for element in a.iter() {
    //    println!("the value is: {}", element);
    //}
    //FOR MAIS LIMPO
    //for number in (1..10).rev() {
    //    println!("{}!", number);
    //}
    //println!("LIFTOFF!!!");

        //TRATAMENTO STRING
   // let mut s = String::from("hello");
    //s.push_str(", world!"); // push_str() appends a literal to a String
    //println!("{}", s); // This will print `hello, world!`


    //COPIA DA VARIAVEL
    //let s1 = String::from("hello");
    //let s2 = s1.clone();
    //println!("s1 = {}, s2 = {}", s1, s2);


    //REFERENCIA
    /*let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);
*/


//1 ESTRUTURA
 /*   let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email );
*/



    //2 ESTRUTURA COM DEBUG
    let rect1 = Rectangle  {width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",    area(&rect1)
    );
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);

}


fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
//FUNCOES DE RETORNO
fn funcaoRetornoSemParametro() -> i32 {
    5
}
fn funcaoRetornoComParametro(x:i32) -> i32{
    let y = x * 2;
    y
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}