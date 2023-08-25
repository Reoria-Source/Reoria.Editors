use crate::layout::*;
use crate::widgets::*;
use araiseal_types::*;
use core::convert::Into;
use graphics::iced_wgpu::Renderer;
use graphics::iced_widget::{
    checkbox, column, container, row, text, text_input, Column, Container, Image, PickList, Row,
    Rule,
};
use graphics::iced_winit::{
    core::{
        alignment::{Alignment, Horizontal, Vertical},
        Color, Element, Length,
    },
    style::Theme,
};

use graphics::iced_aw::ColorPicker;
use std::path::Path;

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Default)]
pub struct ItemUiGeneric {
    #[educe(Default(expression = "Vec::with_capacity(23)"))]
    pub type_list: Vec<ItemTypes>,
    pub type_selected: Option<ItemTypes>,
    pub txt_value: String,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub level_input: NumInput<u16, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub sound_input: NumInput<u16, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub sprite_input: NumInput<u16, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub type2: NumInput<u16, Message>,
    pub breakable: bool,
    pub stackable: bool,
    pub repairable: bool,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub stack_limit_input: NumInput<u16, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub base_price_input: NumInput<u64, Message>,
    pub color: Color,
    pub show_color: bool,
}

impl ItemUiGeneric {
    pub fn layout(&self, _item_type: ItemTypes) -> Element<Message, Renderer<Theme>> {
        let row0 = row![
            text_input("Name", &self.txt_value)
                .on_input(Message::NameInput)
                .width(Length::Fixed(256.0))
                .padding(3),
            row![text("Item Type").horizontal_alignment(Horizontal::Center)].spacing(6),
            PickList::new(&self.type_list[..], self.type_selected, Message::TypeSelect,),
            row![text("Item Type 2").horizontal_alignment(Horizontal::Center)].spacing(6),
            self.type2.view(5, 0, 100, 1, Message::GenericInput),
        ]
        .spacing(12)
        .align_items(Alignment::Center);

        let sprite_value = self.sprite_input.value;

        let image_path = format!("./resources/items/{}.png", sprite_value);

        let row1 = row![column![
            "Item Sprite",
            row![
                self.sprite_input.view(4, 0, 1000, 1, Message::GenericInput),
                if Path::new(&image_path).exists() {
                    container(
                        Image::new(&image_path)
                            .width(Length::Fixed(32.0))
                            .height(Length::Fixed(32.0)),
                    )
                    .center_x()
                    .center_y()
                    .width(Length::Fixed(32.0))
                    .height(Length::Fixed(32.0))
                } else {
                    Container::new("")
                        .center_x()
                        .center_y()
                        .width(Length::Fixed(32.0))
                        .height(Length::Fixed(32.0))
                }
            ]
            .align_items(Alignment::Center)
            .spacing(6)
        ]
        .spacing(5),]
        .spacing(6);

        let row2 = row![
            column![
                "Level Req",
                self.level_input.view(1, 0, 200, 1, Message::GenericInput)
            ]
            .spacing(5),
            column![
                "Sound ID",
                self.sound_input.view(2, 0, 100, 1, Message::GenericInput)
            ]
            .spacing(5),
            column![
                "Stack Limit",
                self.stack_limit_input
                    .view(3, 1, 1000, 1, Message::GenericInput)
            ]
            .spacing(5),
            column![
                "Base Store Price",
                self.base_price_input
                    .view(0, 0, 999999999, 1, Message::BasePriceInput)
            ]
            .spacing(5)
        ]
        .spacing(6);

        let row4 = row![
            checkbox("Breakable", self.breakable, Message::Breakable,),
            checkbox("Repairable", self.repairable, Message::Repairable,),
            checkbox("Stackable", self.stackable, Message::Stackable,)
        ]
        .spacing(6);

        let colorpicker = ColorPicker::<Message, Theme>::new(
            self.show_color,
            self.color,
            araiseal_ui::button("Set Color").on_press(Message::ChooseColor),
            Message::CancelColor,
            Message::SubmitColor,
        );

        let row5 = Row::<Message, Renderer<Theme>>::with_children(vec![
            text(format!(
                "R: {:.0} G: {:.0} B: {:.0} A: {:.0}",
                self.color.r * 255.0,
                self.color.g * 255.0,
                self.color.b * 255.0,
                self.color.a * 255.0
            ))
            .into(),
            Container::new(Row::new())
                .height(Length::Fixed(32.0))
                .width(Length::Fixed(32.0))
                .into(),
        ])
        .spacing(10)
        .align_items(Alignment::Center);

        Column::<Message, Renderer<Theme>>::with_children(vec![
            text("Generic")
                .width(Length::Fill)
                .vertical_alignment(Vertical::Bottom)
                .horizontal_alignment(Horizontal::Center)
                .into(),
            Rule::horizontal(0).into(),
            row0.into(),
            row1.into(),
            row2.into(),
            row4.into(),
            row5.into(),
        ])
        .spacing(6)
        .align_items(Alignment::Center)
        .into()
    }
}