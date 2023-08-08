use crate::npc::*;
use araiseal_types::*;
use araiseal_ui::*;
use iced::pure::{
    widget::{Column, Container, Row, Scrollable},
    Element,
};
use iced::Length;
use std::convert::TryFrom;

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Default)]
pub struct NpcUI {
    #[educe(Default(expression = "Vec::with_capacity(MAX_NPCS)"))]
    pub data: Vec<(NpcData, bool)>,
    menu: NpcUIMenu,
    generic: NpcUIGenerics,
    enemies: NpcUIEnemies,
    settings: NpcUISettings,
    currentid: usize,
}

impl UiRenderer for NpcUI {
    type Message = Message;

    fn update(&mut self, msg: Message) -> Option<Box<dyn UiRenderer<Message = Message>>> {
        match msg {
            Message::SaveAllButtonPress => {
                self.save_all();
                return None;
            }
            Message::SaveButtonPress => {
                self.data[self.currentid]
                    .0
                    .save_file(self.currentid)
                    .unwrap();
                return None;
            }
            Message::RevertButtonPress => {
                let item = NpcData::load_file(self.currentid).unwrap();
                self.data[self.currentid].0 = item.0;
                self.data[self.currentid].1 = false;
                self.set_object_to_layout(self.currentid);
                return None;
            }
            Message::ListSelect(data) => {
                self.currentid = data.id;
                self.menu.list_selected = Some(data);
                self.set_object_to_layout(self.currentid);
                return None;
            }
            Message::EnemyDataListSelect(data) => {
                self.enemies.currentid = data.id;
                self.enemies.data_selected = Some(data);

                if self.enemies.currentid == 0 {
                    return None;
                }

                if let Some(enemy) = self.enemies.data.get(self.enemies.currentid) {
                    if let Some(npcid) = self.data[self.currentid].0.enemies.get(enemy.id - 1) {
                        self.enemies.npc_selected =
                            Some(self.enemies.npc_data[*npcid as usize].clone())
                    }
                } else {
                    return None;
                }
            }
            Message::EnemyListSelect(data) => {
                if self.enemies.currentid == 0 {
                    return None;
                }

                if let Some(enemy) = self.enemies.data.get(self.enemies.currentid) {
                    if let Some(npcid) = self.data[self.currentid].0.enemies.get_mut(enemy.id - 1) {
                        *npcid = data.id as u64;
                        self.enemies.npc_selected = Some(data);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            Message::BehaviourTypeSelect(data) => {
                self.generic.behaviour_selected = Some(data);
                self.data[self.currentid].0.behaviour = data;
            }
            Message::GenericBoolInput((id, data)) => match id {
                14 => {
                    self.settings.target_auto_switch = data.get_data();
                    self.data[self.currentid].0.target_auto_switch = data.get_data();
                }
                15 => {
                    self.settings.target_attacked_switch = data.get_data();
                    self.data[self.currentid].0.target_attacked_switch = data.get_data();
                }
                16 => {
                    self.settings.target_range_dropout = data.get_data();
                    self.data[self.currentid].0.target_range_dropout = data.get_data();
                }
                17 => {
                    self.settings.can_target = data.get_data();
                    self.data[self.currentid].0.can_target = data.get_data();
                }
                18 => {
                    self.settings.can_move = data.get_data();
                    self.data[self.currentid].0.can_move = data.get_data();
                }
                19 => {
                    self.settings.can_attack_player = data.get_data();
                    self.data[self.currentid].0.can_attack_player = data.get_data();
                }
                24 => {
                    self.settings.has_enemies = data.get_data();
                    self.data[self.currentid].0.has_enemies = data.get_data();
                }
                25 => {
                    self.settings.has_allys = data.get_data();
                    self.data[self.currentid].0.has_allys = data.get_data();
                }
                26 => {
                    self.settings.can_attack = data.get_data();
                    self.data[self.currentid].0.can_attack = data.get_data();
                }
                28 => {
                    self.settings.canpassthru = data.get_data();
                    self.data[self.currentid].0.canpassthru = data.get_data();
                }
                _ => return None,
            },
            Message::GenericU8Input((id, data)) => match id {
                0 => {
                    self.generic.sizex_input.value = data.get_data();
                    self.data[self.currentid].0.size.x = data.get_data();
                }
                1 => {
                    self.generic.sizey_input.value = data.get_data();
                    self.data[self.currentid].0.size.y = data.get_data();
                }
                2 => {
                    self.generic.sizeh_input.value = data.get_data();
                    self.data[self.currentid].0.size.height = data.get_data();
                }
                3 => {
                    self.generic.sizew_input.value = data.get_data();
                    self.data[self.currentid].0.size.width = data.get_data();
                }
                _ => return None,
            },
            Message::GenericI32Input((id, data)) => match id {
                0 => {
                    self.generic.sprite_input.value = data.get_data();
                    self.data[self.currentid].0.sprite = data.get_data();
                }
                1 => {
                    self.generic.level_input.value = data.get_data();
                    self.data[self.currentid].0.level = data.get_data();
                }
                2 => {
                    self.generic.sight_input.value = data.get_data();
                    self.data[self.currentid].0.sight = data.get_data();
                }
                3 => {
                    self.generic.follow_sight_input.value = data.get_data();
                    self.data[self.currentid].0.follow_sight = data.get_data();
                }
                _ => return None,
            },
            Message::GenericU32Input((id, data)) => match id {
                3 => {
                    self.generic.maxhp_input.value = data.get_data();
                    self.data[self.currentid].0.maxhp = data.get_data();
                }
                4 => {
                    self.generic.maxsp_input.value = data.get_data();
                    self.data[self.currentid].0.maxsp = data.get_data();
                }
                5 => {
                    self.generic.maxmp_input.value = data.get_data();
                    self.data[self.currentid].0.maxmp = data.get_data();
                }
                6 => {
                    self.generic.walkdistance_input.value = data.get_data();
                    self.data[self.currentid].0.walkdistance = data.get_data();
                }
                7 => {
                    self.generic.pdamage_input.value = data.get_data();
                    self.data[self.currentid].0.pdamage = data.get_data();
                }
                9 => {
                    self.generic.pdef_input.value = data.get_data();
                    self.data[self.currentid].0.pdefense = data.get_data();
                }
                11 => {
                    self.generic.maxdamage_input.value = data.get_data();
                    self.data[self.currentid].0.maxdamage = data.get_data();
                }
                12 => {
                    self.generic.mindamage_input.value = data.get_data();
                    self.data[self.currentid].0.mindamage = data.get_data();
                }
                13 => {
                    self.generic.max_drops.value = data.get_data();
                    self.data[self.currentid].0.drops_max = data.get_data() as u16;
                }
                _ => return None,
            },
            Message::GenericI64Input((id, data)) => match id {
                0 => {
                    self.settings.target_switch_chance.value = data.get_data();
                    self.data[self.currentid].0.target_auto_switch_chance = data.get_data();
                }
                1 => {
                    self.generic.respawn_wait_input.value = data.get_data();
                    self.data[self.currentid].0.respawn_wait = data.get_data();
                }
                2 => {
                    self.generic.movement_wait_input.value = data.get_data();
                    self.data[self.currentid].0.movement_wait = data.get_data();
                }
                3 => {
                    self.generic.attack_wait_input.value = data.get_data();
                    self.data[self.currentid].0.attack_wait = data.get_data();
                }
                4 => {
                    self.generic.intervaled_wait_input.value = data.get_data();
                    self.data[self.currentid].0.intervaled_wait = data.get_data();
                }
                5 => {
                    self.generic.spawn_wait_input.value = data.get_data();
                    self.data[self.currentid].0.spawn_wait = data.get_data();
                }
                _ => return None,
            },
            Message::ItemInput((id, data)) => {
                self.generic.item_drops[id].item_id.value = data.get_data();
                self.data[self.currentid].0.drops[id].0 = data.get_data();
            }
            Message::ItemChanceInput((id, data)) => {
                self.generic.item_drops[id].chance.value = data.get_data();
                self.data[self.currentid].0.drops[id].1 = data.get_data();
            }
            Message::ItemAmountInput((id, data)) => {
                self.generic.item_drops[id].amount.value = data.get_data();
                self.data[self.currentid].0.drops[id].2 = data.get_data();
            }
            Message::ChooseTime1 => {
                self.settings.show_time[0] = true;
                return None;
            }
            Message::SubmitTime1(time) => {
                use chrono::Timelike;
                let naive: chrono::NaiveTime = time.into();
                self.data[self.currentid].0.spawntime.0.hour = naive.hour();
                self.data[self.currentid].0.spawntime.0.min = naive.minute();
                self.data[self.currentid].0.spawntime.0.sec = naive.second();
            }
            Message::ChooseTime2 => {
                self.settings.show_time[1] = true;
                return None;
            }
            Message::SubmitTime2(time) => {
                use chrono::Timelike;
                let naive: chrono::NaiveTime = time.into();
                self.data[self.currentid].0.spawntime.1.hour = naive.hour();
                self.data[self.currentid].0.spawntime.1.min = naive.minute();
                self.data[self.currentid].0.spawntime.1.sec = naive.second();
            }
            Message::CancelTime => {
                self.settings.show_time[0] = false;
                self.settings.show_time[1] = false;
            }
            Message::EnemyAddButtonPress => {
                let count = self.enemies.data.len();

                let list_data = ListData {
                    id: count,
                    name: self.data[0].0.name.clone(),
                };

                self.enemies.currentid = list_data.id;
                self.enemies.data_selected = Some(list_data.clone());
                self.enemies.data.push(list_data);
                self.data[self.currentid].0.enemies.push(0);

                if let Some(enemy) = self.enemies.data.get(self.enemies.currentid) {
                    if let Some(npcid) = self.data[self.currentid].0.enemies.get_mut(enemy.id - 1) {
                        self.enemies.npc_selected =
                            Some(self.enemies.npc_data[*npcid as usize].clone());
                    }
                }
            }
            Message::EnemyDelButtonPress => {
                if self.enemies.currentid == 0 {
                    return None;
                }

                let old_data = self.enemies.data.remove(self.enemies.currentid);

                for list in self.enemies.data.iter_mut() {
                    if list.id > old_data.id {
                        list.id -= 1;
                    }
                }

                self.data[self.currentid].0.enemies.remove(old_data.id - 1);
                self.enemies.currentid -= 1;
                self.enemies.data_selected =
                    Some(self.enemies.data[self.enemies.currentid].clone());

                if let Some(enemy) = self.enemies.data.get(self.enemies.currentid) {
                    self.enemies.data_selected = Some(enemy.clone());
                }
            }
            Message::NameInput(value) => {
                if value.len() < 64 {
                    self.generic.txt_value = value;
                    self.data[self.currentid].0.name = self.generic.txt_value.clone();
                    self.menu.list[self.currentid].name = self.generic.txt_value.clone();
                    self.menu.list_selected = Some(self.menu.list[self.currentid].clone());
                    self.enemies.npc_data[self.currentid].name = self.generic.txt_value.clone();
                } else {
                    return None;
                }
            }
        }

        self.data[self.currentid].1 = true;
        None
    }

    fn view(&self) -> Element<Message> {
        self.layout()
    }

    fn title(&self) -> &str {
        "Npc Editor"
    }
}

impl NpcUI {
    pub fn new() -> Self {
        let mut ui = NpcUI {
            data: NpcData::load_files().unwrap(),
            ..Default::default()
        };

        for (i, v) in ui.data.iter().enumerate() {
            ui.menu.list.push(ListData::new(i, v.0.name.clone()));
            ui.enemies.npc_data.push(ListData::new(i, v.0.name.clone()));
        }

        for i in 0..=AIBehavior::ReactiveHealer as usize {
            ui.generic
                .behaviours
                .push(AIBehavior::try_from(i as u8).unwrap_or(AIBehavior::ReactiveHealer))
        }

        ui.menu.list_selected = Some(ui.menu.list[0].clone());
        ui.set_object_to_layout(0);
        ui
    }

    pub fn save_all(&mut self) {
        for (i, v) in self.data.iter().enumerate() {
            if !v.1 {
                continue;
            }

            if let Err(e) = v.0.save_file(i) {
                println!("Could not save Quest {}, err {}", i, e);
            }
        }
    }

    fn set_object_to_layout(&mut self, index: usize) {
        self.enemies.data.clear();
        self.enemies.data.push(ListData::new(0, "None".to_string()));

        for (i, v) in self.data[index].0.enemies.iter().enumerate() {
            if let Some(npc) = self.data.get(*v as usize) {
                self.enemies
                    .data
                    .push(ListData::new(i + 1, npc.0.name.clone()));
            }
        }

        self.enemies.data_selected = Some(ListData::new(0, "None".to_string()));
        self.enemies.currentid = 0;
        self.enemies.npc_selected = None;
        self.generic.txt_value = self.data[index].0.name.clone();
        self.generic.sprite_input.value = self.data[index].0.sprite;
        self.generic.behaviour_selected = Some(self.data[index].0.behaviour);
        self.settings.target_auto_switch = self.data[index].0.target_auto_switch;
        self.settings.target_attacked_switch = self.data[index].0.target_attacked_switch;
        self.settings.target_range_dropout = self.data[index].0.target_range_dropout;
        self.settings.can_target = self.data[index].0.can_target;
        self.settings.can_move = self.data[index].0.can_move;
        self.settings.can_attack_player = self.data[index].0.can_attack_player;
        self.settings.has_enemies = self.data[index].0.has_enemies;
        self.settings.has_allys = self.data[index].0.has_allys;
        self.settings.can_attack = self.data[index].0.can_attack;
        self.settings.canpassthru = self.data[index].0.canpassthru;
        self.generic.sizex_input.value = self.data[index].0.size.x;
        self.generic.sizey_input.value = self.data[index].0.size.y;
        self.generic.sizeh_input.value = self.data[index].0.size.height;
        self.generic.sizew_input.value = self.data[index].0.size.width;
        self.generic.sprite_input.value = self.data[index].0.sprite;
        self.generic.level_input.value = self.data[index].0.level;
        self.generic.sight_input.value = self.data[index].0.sight;
        self.generic.follow_sight_input.value = self.data[index].0.follow_sight;
        self.generic.maxhp_input.value = self.data[index].0.maxhp;
        self.generic.maxsp_input.value = self.data[index].0.maxsp;
        self.generic.maxmp_input.value = self.data[index].0.maxmp;
        self.generic.walkdistance_input.value = self.data[index].0.walkdistance;
        self.generic.pdamage_input.value = self.data[index].0.pdamage;
        self.generic.pdef_input.value = self.data[index].0.pdefense;
        self.generic.maxdamage_input.value = self.data[index].0.maxdamage;
        self.generic.mindamage_input.value = self.data[index].0.mindamage;
        self.settings.target_switch_chance.value = self.data[index].0.target_auto_switch_chance;
        self.generic.respawn_wait_input.value = self.data[index].0.respawn_wait;
        self.generic.movement_wait_input.value = self.data[index].0.movement_wait;
        self.generic.attack_wait_input.value = self.data[index].0.attack_wait;
        self.generic.intervaled_wait_input.value = self.data[index].0.intervaled_wait;
        self.generic.spawn_wait_input.value = self.data[index].0.spawn_wait;
        self.generic.max_drops.value = self.data[index].0.drops_max as u32;

        for i in 0..10 {
            self.generic.item_drops[i].item_id.value = self.data[index].0.drops[i].0;
            self.generic.item_drops[i].chance.value = self.data[index].0.drops[i].1;
            self.generic.item_drops[i].amount.value = self.data[index].0.drops[i].2;
        }
    }

    pub fn layout(&self) -> Element<Message> {
        let column = Column::new()
            .spacing(20)
            .push(self.menu.layout())
            .push(Scrollable::new(
                Row::new()
                    .push(
                        Column::new()
                            .spacing(5)
                            .push(
                                Container::new(self.generic.layout())
                                    .style(araiseal_styles::UiContainer)
                                    .padding(5)
                                    .width(Length::Fill)
                                    .center_x()
                                    .center_y(),
                            )
                            .push(
                                Container::new(self.settings.layout(&self.data[self.currentid].0))
                                    .style(araiseal_styles::UiContainer)
                                    .padding(5)
                                    .width(Length::Fill)
                                    .center_y(),
                            )
                            .push(
                                Container::new(self.enemies.layout())
                                    .style(araiseal_styles::UiContainer)
                                    .padding(5)
                                    .width(Length::Fill)
                                    .center_y(),
                            )
                            .width(Length::FillPortion(30)),
                    )
                    .push(Column::new().width(Length::FillPortion(1))),
            ));

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}