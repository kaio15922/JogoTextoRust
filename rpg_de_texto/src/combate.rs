use std::io;

use crate::{personagens::Personagem};

pub fn combate (player: &mut Personagem, inimigo: &mut Personagem) {

    println!("Um {} apareceu!", inimigo.nome );

    while (player.vida > 0 && inimigo.vida > 0) {

        println!("O que você irá fazer?\n");

        println!("1. Atacar");
        println!("2. Golpe Especial {}/5", player.ataque_especial.unwrap_or(0));
        println!("3. Usar Item\n");

        let mut escolha = String::new();

        io::stdin().read_line(&mut escolha).unwrap();
        
        let escolha: u32 = escolha.trim().parse().unwrap();

        match escolha {

            1 => { player.ataque(inimigo, false);
                    println!("Você atacou!\n");
                },

            2 => { player.ataque(inimigo, true);
                    println!("Você usou o golpe especial!\n");
                 },

            3 => { 

                    println!("Qual item deseja utilizar?\n");
                    player.mostrar_inventario();

                        let mut item_escolhido = String::new();

                        io::stdin().read_line(&mut item_escolhido).unwrap();
        
                        let item_escolhido: i32 = item_escolhido.trim().parse().unwrap();

                    player.usar_item( (item_escolhido-1) );
                    
                 
                },
            
            _ => { println!("aoba\n"); },

        }

        inimigo.ataque(player, false);

        println!("Sua vida: {}\n", player.vida);
        println!("Vida do Inimigo {}\n", inimigo.vida);

    }

    println!("Combate encerrado\n");

}