use crate::{
    world::World,
    actor::{Actor, ActorId, ActorKind, ActorAI},
    terrain::{TerrainKind, DoorState, Orientation, TerrainFeature},
    item::*,
    point::Point,
    terrain::read_from_file
};

use super::*;

use std::collections::HashMap;
use rand::Rng;
use maplit::hashmap;

pub fn populate_world(world: &mut World) {
    // read map from file
    let kind_map = hashmap! {
        '.' => TerrainKind::Grass,
        '*' => TerrainKind::Hedge,
        ':' => TerrainKind::StoneFloor,
        'P' => TerrainKind::Path,
        ';' => TerrainKind::ThickGrass,
        'W' => TerrainKind::Water,
        '#' => TerrainKind::Wall,
        '~' => TerrainKind::ShallowWater,
        'd' => TerrainKind::Door(DoorState::Locked),
        'D' => TerrainKind::Door(DoorState::Open),
        '+' => TerrainKind::Window,
        'B' => TerrainKind::Bridge(Orientation::Vertical),
        'b' => TerrainKind::Bridge(Orientation::Horizontal),
        'f' => TerrainKind::Grass,
    };

    let feature_map = hashmap! {
        'f' => TerrainFeature::Fountain,
    };
    
    world.terrain = read_from_file("assets/sample.layer", &kind_map, &feature_map).unwrap();

    let player_id = world.player_id();
    
    // add some items to player's inventory
    let magical_wand = Item::new(ItemKind::Wand)
        .with_owner(player_id);
    let some_money = Item::new(ItemKind::Money(42))
        .with_owner(player_id);
    let healing_potion = Item::new(ItemKind::Potion(Potion::Healing))
        .with_owner(player_id);
    let swimming_potion = Item::new(ItemKind::Potion(Potion::Swimming))
        .with_owner(player_id);
    let vision_potion = Item::new(ItemKind::Potion(Potion::Vision))
        .with_owner(player_id);
    let key = Item::new(ItemKind::Key)
        .with_owner(player_id);

    let player = world.actors.get_mut(&player_id).unwrap();
    player.inventory.push(world.items.add(magical_wand));
    player.inventory.push(world.items.add(some_money));
    player.inventory.push(world.items.add(healing_potion));
    player.inventory.push(world.items.add(swimming_potion));
    player.inventory.push(world.items.add(vision_potion));
    player.inventory.push(world.items.add(key));

    // spawn some more items on the map (just as an example)
    world.items.add(Item::new(ItemKind::Money(10)).with_pos((5, 6)));
    world.items.add(Item::new(ItemKind::Wand).with_pos((12, 10)));
    world.items.add(Item::new(ItemKind::Wand).with_pos((5, 6)));
    world.items.add(Item::new(ItemKind::Gold).with_pos(player.pos));
    world.items.add(Item::new(ItemKind::Ore).with_pos(player.pos));
    world.items.add(Item::new(ItemKind::Bread).with_pos(player.pos));
    world.items.add(Item::new(ItemKind::Money(20)).with_pos(player.pos));
    world.items.add(Item::new(ItemKind::Wand).with_pos(player.pos));

    // add shopkeeper next to the player, so that we can immediately go shopping
    let pos = player.pos + Point::from((1,0));
    let shopkeeper = Actor::new(ActorKind::Shopkeeper, pos, 4)
        .with_ai(ActorAI::DoNothing);
    
    world.actors.add(shopkeeper);

    // spawn some random NPCs
    
    // TODO: this is some sort of index which could be kept up-to-date
    let actor_positions = world.actors.iter()
        .map(|(id, actor)| (actor.pos.clone(), id.clone()))
        .collect::<HashMap<Point, ActorId>>();
    
    let max_npc: u32 = 5;
    
    let mut slots = world.terrain.iter()
        .filter(|(pos, tile)| tile.kind == TerrainKind::StoneFloor
                && !actor_positions.contains_key(*pos))
        .map(|(pos, _tile)| pos)
        .collect::<Vec<&Point>>();
    
    let mut rng = rand::thread_rng();
    for _ in 0..max_npc {
        let len = slots.len();
        if len == 0 {
            break;
        }
        let index = rng.gen_range(0..len);
        let actor_pos = slots[index];
        let new_actor = Actor::new(
            ActorKind::Townsfolk,
            actor_pos.clone(),
            rng.gen_range(5..8)
        );
        world.actors.add(new_actor);
        slots.remove(index);
    }
}
