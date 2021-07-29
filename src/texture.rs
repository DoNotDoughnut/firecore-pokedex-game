use engine::{
    graphics::TextureManager,
    tetra::{graphics::Texture, Context, Result},
};
use hashbrown::HashMap;

use pokedex::{item::ItemId, pokemon::PokemonId, trainer::TrainerId};

use crate::serialize::SerializedPokemon;

pub type TrainerTextures = TextureManager<TrainerId>;
pub type ItemTextures = TextureManager<ItemId>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PokemonTexture {
    Front,
    Back,
    Icon,
}

pub struct PokemonTextures {
    pub front: HashMap<PokemonId, Texture>,
    pub back: HashMap<PokemonId, Texture>,
    pub icon: HashMap<PokemonId, Texture>,
}

impl PokemonTextures {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            front: HashMap::with_capacity(capacity),
            back: HashMap::with_capacity(capacity),
            icon: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, ctx: &mut Context, pokemon: &SerializedPokemon) -> Result {
        self.front.insert(
            pokemon.pokemon.id,
            Texture::from_file_data(ctx, &pokemon.front)?,
        );
        self.back.insert(
            pokemon.pokemon.id,
            Texture::from_file_data(ctx, &pokemon.back)?,
        );
        self.icon.insert(
            pokemon.pokemon.id,
            Texture::from_file_data(ctx, &pokemon.icon)?,
        );
        Ok(())
    }

    pub fn get(&self, id: &PokemonId, side: PokemonTexture) -> &Texture {
        match side {
            PokemonTexture::Front => self.front.get(id),
            PokemonTexture::Back => self.back.get(id),
            PokemonTexture::Icon => self.icon.get(id),
        }
        .unwrap_or_else(|| panic!("Could not get texture for pokemon with ID {}", id))
    }
}
