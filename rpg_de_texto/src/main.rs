mod personagens;
mod itens;
mod combate;

use std::ops::Index;
use crate::{itens::Item, personagens::Personagem, combate::combate};

fn main() 
{

    //So testando
    println!("Hello, world!");

    let mut barbaro = Personagem::gerar_barbaro();
    let mut arqueiro = Personagem::gerar_arqueiro();
    let mut mago = Personagem::gerar_mago();
    let dragao = Personagem::gerar_dragao();
    let mut goblin = Personagem::gerar_goblin();
    let orc = Personagem::gerar_orc();

    let vida = Item::gerar_poc_vida();
    let defesa = Item::gerar_poc_defesa();
    let ataque = Item::gerar_poc_ataque();
    let vida2 = Item::gerar_poc_vida();

    println!("o nome do barbaro é {}// sua vida é {}// seu ataque é {}// sua defesa é {}...", barbaro.nome, barbaro.vida, barbaro.ataque, barbaro.defesa);

    println!("\n------------------------\n");

    barbaro.add_no_inventario(vida);
    barbaro.add_no_inventario(ataque);
    barbaro.add_no_inventario(vida2);
    

    println!("\n------------------------\n");

    barbaro.mostrar_inventario();

    println!("\n------------------------\n");

    barbaro.usar_item(2);

    println!("\n------------------------\n");

    println!("vida do arqueiro = {}", barbaro.vida);

    println!("\n------------------------\n");

    barbaro.add_no_inventario(defesa);

    println!("\n------------------------\n");
    
    barbaro.mostrar_inventario();

    println!("\n------------------------\n");

    barbaro.usar_item(0);
    barbaro.usar_item(1);
    barbaro.usar_item(0);

    println!("\n------------------------\n");

    barbaro.mostrar_inventario();

    //TESTE DE 
    let vidateste = Item::gerar_poc_vida();
    mago.add_no_inventario(vidateste);

    combate(&mut mago, &mut goblin);
}
