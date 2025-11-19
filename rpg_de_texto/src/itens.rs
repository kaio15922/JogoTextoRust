use crate::personagens::Personagem;

pub struct Item
{
    pub nome: String,
    pub descrição: String,
    pub efeito: u32,
}

impl Item 
{
    //recebe referencia do item, mas pensa comigo, se faz efeito tem q sumir, entao passar ownwer?
    pub fn fazer_efeito(&self ,personagem: &mut Personagem)
    {
        match self.nome.as_str()
        {
            "Poção_de_ataque"   => {personagem.ataque += self.efeito}
            "Poção de defesa"   => {personagem.defesa += self.efeito}
            "Poção de vida"   =>   {personagem.vida += self.efeito as i32}
            _                   => {println!("bateu aq")},
        }
    }

    pub fn gerar_poc_ataque() -> Self
    {
        Self 
        { 
            nome: "Poção_de_ataque".to_string(), 
            descrição: "Aumenta o ataque em 10".to_string(), 
            efeito: 10 
        }
    } 
    pub fn gerar_poc_vida() -> Self
    {
        Self 
        { 
            nome: "Poção de vida".to_string(), 
            descrição: "Recupera 30 de vida".to_string(), 
            efeito: 30 
        }
    } 
    pub fn gerar_poc_defesa() -> Self
    {
        Self 
        { 
            nome: "Poção de defesa".to_string(), 
            descrição: "Aumenta a defesa em 5".to_string(), 
            efeito: 5 
        }
    }   
}