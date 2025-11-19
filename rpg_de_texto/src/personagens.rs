use rand::Rng;

struct Personagem {

    nome: String,
    vida: i32,
    ataque: u32,
    defesa: u32,
    //inventario;// Implementar depois

}

impl Personagem {

    fn ataque (&mut self, alvo: &mut Personagem, especial: bool) {

        let mut margem: u32 = 0;

        match &self.nome {

            "Arqueiro" => { margem = 31; },
            "Bárbaro"  => { margem = 26; },
            "Mago"     => { margem = 36; },
            _ => {}
        }

        if (especial == true) {

            margem -=15;
        }

        let mut dano = rand::thread_rng().gen_range(1..=margem);

        if (especial ==true){

            dano = dano*2;
        }

        if (dano > alvo.defesa) {

            alvo.vida -= dano as i32;
        }

    }
}

//impl personagem {add item ao inventario}//

fn gerar_arqueiro () -> Personagem {

    let arqueiro = Personagem {

            nome: "Arqueiro".to_string(),
            vida: 250,
            ataque: 0,
            defesa: 7,

    };

    return arqueiro;

}

fn gerar_barbaro () -> Personagem {

    let barbaro = Personagem {

            nome: "Bárbaro".to_string(),
            vida: 300,
            ataque: 0,
            defesa: 12,

    };

    return barbaro;

}

fn gerar_mago () -> Personagem {

    let mago = Personagem {

            nome: "Mago".to_string(),
            vida: 260,
            ataque: 0,
            defesa: 5,

    };

    return mago;

}