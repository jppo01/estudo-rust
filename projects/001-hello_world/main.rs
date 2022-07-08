// PRIMEIRO PROGRAMA - HELLO, WORLD!

fn main (){
    println!("hello, world!");
}








/***********************************************************************

 ########################## RESUMO/EXPLICAÇÃO ##########################

==> fn define uma função. 
==> fn main(){
, 
    }
    main é uma função especial, é a primeira função a ser executada no RUST. Nessa linha estou 
    declarando uma função chamada main que não retorna nada. Caso tenha algum parametro para passar para a
    função esses paramretros devem ser passads dentro dos parenteses. 
==> Em rust todo corpo de função tem que ser delimitado por {}
==> se você quiser pode usar a ferramenta rustfmt para formatar o seu código. Por padrão ela é instalada 
    junto com o RUST. Então caso queira, já pode começar usar. Dê uma olhada na documentação.
==> A identação recomenada para seguir o padrão de estilo RUST é feita usando 4 “espaços” e não tab.
==> println! chama uma macro em RUST. Aqui talvez seja novidade para você que já teve contato com outras linguagens. 
    Caso você tivesse digitado o código sem o ponto e esclamação no final de println você estaria chamando uma função normal. 
    Para o propósito desse post é importante que você saiba que uma macro é chamada usando (!) e que macros nem sempre segue 
    as mesmas regras de uma função.
==> “hello, world!” é uma string que passamos como parâmetro entre parêntese para a macro println!
==> o ponto e virgula( ; ) no final da linha indica para o RUST que a expressão terminou e que pode começar a expressão seguinte


**********************************************************************/
