
use crate::point::RvPoint;
use crate::actor::ActorId;
use crate::world::World;

#[derive(Debug)]
pub enum Action {
    Move { actor_id: ActorId, pos: RvPoint }
}

// pub fn move_by(world: &World, actor_id: &ActorId, dx: i32, dy: i32, follow: bool) ->
//     Option<Action>
// {
//     let actor = world.actors.get(actor_id).unwrap();
//     //match actor.pos.as_ref() {
//         //Some(pos) => {
        
//     let new_pos = actor..pos + (dx, dy).into();
//     if !World::is_blocking(&new_pos, &world.tiles, &world.actors) {
//         if follow {
//             let mode = match (dx, dy) {
//                 (0, -1) => ViewportMode::North,
//                 (0, 1) => ViewportMode::South,
//                 (1, 0) => ViewportMode::East,
//                 (-1,0) => ViewportMode::West,
//                 _ => ViewportMode::Center // TODO: ::Skip ?
//             };
            
//             return Some(GameAction::MoveFollow {actor_id: actor_id.clone(), pos: new_pos.clone(), mode });
//         } else {
//             return Some(GameAction::Move { actor_id: actor_id.clone(), pos: new_pos.clone() });
//         }
//     }
// },
//         _ => {}
//     }
//     None
// }

