use crate::widgets::NumInputMessage;
use araiseal_types::*;
use araiseal_ui::ListData;
use graphics::iced_winit::core::Color;

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    SaveButtonPress,
    SaveAllButtonPress,
    RevertButtonPress,
    ListSelect(ListData),
    TypeSelect(ItemTypes),
    DataInput((usize, NumInputMessage<i16>)),
    GenericInput((usize, NumInputMessage<u16>)),
    BasePriceInput((usize, NumInputMessage<u64>)),
    Repairable(bool),
    Stackable(bool),
    Breakable(bool),
    NameInput(String),
    ChooseColor,
    SubmitColor(Color),
    CancelColor,
}