mod personagens;
mod itens;

use crate::{itens::Item, personagens::Personagem};

fn main() {

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

    println!("o nome do mago é {}// sua vida é {}// seu ataque é {}// sua defesa é {}...", mago.nome, mago.vida, mago.ataque, mago.defesa);

    println!("vida do arqueiro = {}", arqueiro.vida);

    barbaro.ataque(&mut arqueiro, false);

    println!("vida do arqueiro = {}", arqueiro.vida);

    println!("vida do arqueiro = {}", arqueiro.vida);

    barbaro.ataque(&mut arqueiro, false);

    println!("vida do arqueiro = {}", arqueiro.vida);

    println!("vida do arqueiro = {}", arqueiro.vida);

    barbaro.ataque(&mut arqueiro, false);

    println!("vida do arqueiro = {}", arqueiro.vida);

    vida.fazer_efeito(&mut arqueiro);
    vida.fazer_efeito(&mut arqueiro);
    vida.fazer_efeito(&mut arqueiro);

    println!("vida do arqueiro = {}", arqueiro.vida);

    barbaro.add_no_inventario(vida);
    barbaro.add_no_inventario(defesa);

    barbaro.mostrar_inventario();
    barbaro.mostrar_inventario();
    barbaro.mostrar_inventario();





}
