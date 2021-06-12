
use crate::point::Point;
use crate::actor::ActorId;
use crate::world::{ViewportMode};
use crate::item::ItemId;
use crate::MainInputMode;

#[derive(Debug)]
pub enum Action {
    MoveFollow { actor_id: ActorId, pos: Point, mode: ViewportMode },
    Move { actor_id: ActorId, pos: Point },
    PickUp { actor_id: ActorId, items: Vec<ItemId> },
    Ouch,
    EndTurn,
    Quit,
    UseItem { item_id: ItemId, target: ActorId },
    MoveViewport { dx: i32, dy: i32 },
    CenterViewport,
    GUI(GuiAction)
}

#[derive(Debug)]
pub enum GuiAction {
    TestBW,
    HideShowInventory,
    HideShowHelp,
    HideShowStatus,
    HideShowFOV,
    SwitchMode(MainInputMode)
}
