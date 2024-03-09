use crate::bulkgui::bulk_gui::*;
use crate::bulkgui::bulk_enum::*;
use crate::bulkgui::bulk_impl::*;

use std::path::Path;

#[allow(dead_code)]
impl eframe::App for BulkGui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        // Save Window
        if self.dialog_open_save {
            egui::Window::new("Save")
            .resizable(false)
            .collapsible(false)
            .title_bar(true)
            .default_pos(egui::Pos2::new(frame.info().window_info.size.x * 0.4, frame.info().window_info.size.y * 0.4))
            .show(ctx, |ui| {
                ui.set_min_size(egui::Vec2::new(300.0, 105.0));
                ui.set_max_size(egui::Vec2::new(300.0, 105.0));
                ui.vertical( |ui| {
                    ui.add_space(7.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::Vec2::new(290.0, 90.0));
                        ui.set_max_size(egui::Vec2::new(290.0, 90.0));
                        ui.label(format!("You are about to write these changes to disk. \n\nYou have made {} changes to {} files \n\n\n Are you sure you want to save the changes?", self.modifications_total, self.table_files_selected_total));
                        ui.separator();
                        ui.horizontal( |ui| {
                            if ui.button("Cancel").clicked() {
                                // Cancel
                                self.window_save_close();
                            };
                            ui.separator();
                            ui.add_space(180.0);
                            ui.separator();
                            if ui.button("Save").clicked() {
                                // Do some saving
                                self.window_save_close();
                                self.save();
                            };
                        })
                    });
                });
            });
        }
        
        // Saving Window
        if self.dialog_open_saving {
            egui::Window::new("Writing Changes...")
            .resizable(false)
            .collapsible(false)
            .title_bar(true)
            .default_pos(egui::Pos2::new(frame.info().window_info.size.x * 0.4, frame.info().window_info.size.y * 0.4))
            .show(ctx, |ui| {
                ui.set_min_size(egui::Vec2::new(300.0, 40.0));
                ui.set_max_size(egui::Vec2::new(300.0, 40.0));
                ui.vertical(|ui| {
                    ui.separator();
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(egui::ProgressBar::new(self.saving_progress / 100.0));
                            ui.separator();
                            ui.add(egui::Label::new((self.saving_progress as u32).to_string()));
                        })
                    })
                })
            });
        }

        // Preview Window
        if self.dialog_open_preview {
            egui::Window::new("Preview Changes")
            .resizable(false)
            .collapsible(false)
            .title_bar(true)
            .default_pos(egui::Pos2::new(frame.info().window_info.size.x * 0.4, frame.info().window_info.size.y * 0.4))
            .show(ctx, |ui| {
                if ui.button("Close").clicked() {
                    self.window_preview_close();
                }
            });
        }

        // Error Window
        if self.dialog_open_error {
            egui::Window::new("Errors")
            .resizable(false)
            .collapsible(false)
            .title_bar(true)
            .default_pos(egui::Pos2::new(frame.info().window_info.size.x * 0.4, frame.info().window_info.size.y * 0.4))
            .show(ctx, |ui| {
                ui.set_min_size(egui::Vec2::new(300.0, 135.0));
                ui.set_max_size(egui::Vec2::new(300.0, 135.0));
                ui.add_space(7.0);
                ui.group(|ui| {
                    ui.set_min_size(egui::Vec2::new(295.0, 100.0));
                    ui.set_max_size(egui::Vec2::new(295.0, 100.0));
                    egui::ScrollArea::vertical()
                    .id_source("error_scroll")
                    .show(ui, |ui| {
                        ui.set_width(295.0);
                        ui.wrap_text();
                        ui.label(self.errors[self.window_error_index].to_owned());
                    });
                });
                ui.separator();
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add_space(110.0);
                    if ui.button("Close").clicked() {
                        self.window_error_close();
                    }
                    if ui.button("<").clicked() {
                        if self.window_error_index > 0 as usize {
                            self.window_error_index -= 1;
                        }
                    }
                    if ui.button(">").clicked() {
                        if self.window_error_index < self.errors.len() - 1 {
                            self.window_error_index += 1;
                        }
                    }
                    ui.add_space(100.0);
                })
            });
        }

        // Undo Window
        if self.dialog_open_undo {
            egui::Window::new("Confirm Undo")
            .resizable(false)
            .collapsible(false)
            .title_bar(true)
            .default_pos(egui::Pos2::new(frame.info().window_info.size.x * 0.4, frame.info().window_info.size.y * 0.4))
            .show(ctx, |ui| {
                ui.vertical( |ui| {
                ui.add_space(7.0);
                ui.group(|ui| {
                    ui.set_min_size(egui::Vec2::new(260.0, 45.0));
                    ui.set_max_size(egui::Vec2::new(260.0, 45.0));
                    ui.label(format!("Are you sure you'd like to undo the last action?"));
                    ui.separator();
                    ui.horizontal( |ui| {
                        if ui.button("Cancel").clicked() {
                            // Cancel
                            self.window_undo_close();
                        };
                        ui.separator();
                        ui.add_space(180.0);
                        ui.separator();
                        if ui.button("Confirm").clicked() {
                            // Do some undoing
                            self.window_undo_close();
                            self.undo();
                        };
                    })
                });
            });
            });
        }

        // Redo Window
        if self.dialog_open_redo {
            egui::Window::new("Confirm Redo")
            .resizable(false)
            .collapsible(false)
            .title_bar(true)
            .default_pos(egui::Pos2::new(frame.info().window_info.size.x * 0.4, frame.info().window_info.size.y * 0.4))
            .show(ctx, |ui| {
                ui.vertical( |ui| {
                ui.add_space(7.0);
                ui.group(|ui| {
                    ui.set_min_size(egui::Vec2::new(260.0, 45.0));
                    ui.set_max_size(egui::Vec2::new(260.0, 45.0));
                    ui.label(format!("Are you sure you'd like to redo the last undo?"));
                    ui.separator();
                    ui.horizontal( |ui| {
                        if ui.button("Cancel").clicked() {
                            // Cancel
                            self.window_redo_close();
                        };
                        ui.separator();
                        ui.add_space(180.0);
                        ui.separator();
                        if ui.button("Confirm").clicked() {
                            // Do some redoing
                            self.window_redo_close();
                            self.redo();
                        };
                    })
                });
            });
            });
        }

        if self.first_cycle {
            if cfg!(windows) {
                self.os_string = "Windows";
                self.default_path = "C:/".to_string();
                self.browser_path_current = "C:/".to_string();
                self.browser_path_line = "C:/".to_string();
            }else if cfg!(unix) {
                self.os_string = "Unix";
                self.default_path = "/".to_string();
                self.browser_path_current = "/".to_string();
                self.browser_path_line = "/".to_string();
            };
            self.first_cycle = !self.first_cycle;
        }

        if self.update_all {
            self.table_files.clear();
            self.table_files_selected.clear();
            self.table_files_renamed.clear();
            self.browser_list_folders.clear();
            match get_folder(self.browser_path_current.to_owned(), true) {
                Ok(folder) => {
                    self.browser_folder = folder.clone();
                    for (_index, f) in folder.list_files.iter().enumerate() {
                        if f.read_into {
                            self.table_files.push(f.name.to_owned());
                            self.table_files_selected.push(false);
                            self.table_files_renamed.push(f.name.to_owned());
                        }
                    };
                    for (_index, f) in folder.list_folders.iter().enumerate() {
                        if self.show_all_items {
                            self.browser_list_folders.push(f.name.to_owned());
                        } else if f.read_into {
                            self.browser_list_folders.push(f.name.to_owned());
                        }
                    };
                },
                Err(error) => {
                    //Display Error
                    println!("{:?}", error);
                    self.errors.push(error.to_string().to_owned());
                    self.window_error_open();
                }
            }
            self.update_all = false;
            self.browser_path_line = self.browser_path_current.to_owned();
            if !self.first_cycle && ((self.browser_path_current.len() * 6) > (self.section_browser_size_x - 58.0) as usize) {
                //Deobfuscate Text

            }
        }

        // Main GUI Group Sizing
        self.section_browser_size_x = (frame.info().window_info.size.x * (self.section_browser_percentage_current as f32 / 100.0)).floor();
        self.section_browser_size_y = (frame.info().window_info.size.y - 95.0).floor();

        self.section_files_size_x = (frame.info().window_info.size.x * (self.section_files_percentage_current as f32 / 100.0)).floor();
        self.section_files_size_y = (frame.info().window_info.size.y - 95.0).floor();

        self.section_options_size_x = (frame.info().window_info.size.x * (self.section_options_percentage_current as f32 / 100.0) - 80.0).floor();
        self.section_options_size_y = (frame.info().window_info.size.y - 95.0).floor();

        // Instantiate Main Window 
        egui::CentralPanel::default().show(ctx, |ui| {
            //ctx.set_visuals(egui::style::Visuals::dark());

            //Set title bar name. (Technically Redundant, but for reference)
            frame.set_window_title(self.program_name);

            // Top Bar
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("🗁 Open Folder").clicked() {
                        
                    };
                });
                ui.menu_button("Edit", |ui| {
                    let undo_button = ui.add_enabled(self.button_undo_enabled, egui::Button::new("↩ Undo Last Action"));
                    if undo_button.clicked(){
                        self.window_undo_open();
                    };
                    let redo_button = ui.add_enabled(self.button_redo_enabled, egui::Button::new("↪ Redo Last Action"));
                    if redo_button.clicked(){
                        self.window_redo_open();
                    };
                });
                
                ui.menu_button("Options", |ui| {
                    if ui.checkbox(&mut self.checkbox_lock_section_resizing, "Lock Section Resizing").clicked() {

                    }
                    if ui.checkbox(&mut self.double_click_deselect_enabled, "Double-Click Deselect").clicked() {

                    }
                    if ui.checkbox(&mut self.show_all_items, "Show All Items").clicked() {
                        self.update_all = true;
                    }
                });
                ui.menu_button("Extras", |ui| {
                    ui.menu_button("Resize Panels", |ui| {
                        let browser_slider = ui.add_enabled(!self.checkbox_lock_section_resizing, egui::Slider::new(&mut self.section_browser_percentage_current, 
                            self.section_browser_percentage_min..=self.section_browser_percentage_max)
                            .logarithmic(true)
                            .text("Browser Width %")
                        );
                        if browser_slider.changed() {
                            //Resize this section.
                            let offset: f32 = 100.00 - (self.section_browser_percentage_current as f32 + self.section_options_percentage_current as f32);
                            self.section_files_percentage_current = offset as u8;
    
                        };
                    });
                    ui.menu_button("Theme", |ui| {
                        if ui.button("🌙 Dark").clicked() {
                            ctx.set_visuals(egui::style::Visuals::dark());
                        };
                        if ui.button("☀ Light").clicked() {
                            ctx.set_visuals(egui::style::Visuals::light());
                        };
                    });
                });
                ui.menu_button("About", |ui| {
                    if ui.button("ℹ egui").clicked() {

                    }
                    if ui.button(format!("ℹ {}", self.program_name)).clicked() {

                    }
                });
                /*
                ui.label(format!("\t\tWidth: {}", frame.info().window_info.size.x.to_string()));
                ui.label(format!("Height: {}", frame.info().window_info.size.y.to_string()));
                let pointer = ctx.pointer_latest_pos();
                match pointer{
                    None => {
                        ui.label("\t\tPointer X: 0, Pointer Y: 0");
                    },
                    Some(pointer) => {
                        ui.label(format!("\t\tPointer X: {:?}, Pointer Y: {:?}", pointer.x.floor(), pointer.y.floor()));
                    }
                };
                */
                //ui.label(format!("{}", self.section_files_hovered.to_string()));
            });
            
            ui.separator();

            // Main GUI Group
            ui.group(|ui| {
                ui.set_min_size(egui::vec2(frame.info().window_info.size.x - 28.0, frame.info().window_info.size.y - 86.0));
                ui.horizontal_top(|ui| {
                    //Browser Section
                    ui.add_enabled_ui(self.section_browser_enabled, |ui|{
                        ui.group(|ui| {
                            ui.set_min_size(egui::vec2(self.section_browser_size_x, self.section_browser_size_y));
                            ui.vertical(|ui| {
                                ui.set_max_size(egui::vec2(self.section_browser_size_x - 5.0, self.section_browser_size_y - 5.0));

                                //Top Bar
                                ui.horizontal(|ui| {
                                    ui.set_width(self.section_browser_size_x - 5.0);
                                    if self.browser_path_current == self.default_path { self.button_browser_up_enabled = false }
                                    else { self.button_browser_up_enabled = true };
                                    let dir_up = ui.add_enabled(self.button_browser_up_enabled, egui::Button::new("⬆"));
                                    if dir_up.on_hover_text_at_pointer("Up in the Directory Stucture").clicked() {
                                        if cfg!(windows) {
                                            let mut sliced_path: Vec<String> = vec![];
                                            let path: String;
                                            for (_index, slice) in self.browser_path_current.split("/").enumerate() {
                                                if slice != "" {
                                                    sliced_path.push(slice.to_string());
                                                }
                                            }
                                            if sliced_path.len() > 0 {
                                                sliced_path.pop();
                                            }
                                            let joined: String = sliced_path.join("/");
                                            if joined.len() > 1 {
                                                path = format!("{}/", joined);
                                            } else { path = "C:/".to_string() }
                                            self.browser_path_last = self.browser_path_current.to_owned();
                                            self.browser_path_current = path.to_owned();
                                            self.update_all = true;
                                        } else if cfg!(unix) {
                                            let mut sliced_path: Vec<String> = vec![];
                                            let path: String;
                                            for (_index, slice) in self.browser_path_current.split("/").enumerate() {
                                                if slice != "" {
                                                    sliced_path.push(slice.to_string());
                                                }
                                            }
                                            if sliced_path.len() > 0 {
                                                sliced_path.pop();
                                            }
                                            let joined: String = sliced_path.join("/");
                                            if joined.len() > 0 {
                                                path = format!("/{}/", joined);
                                            } else { path = "/".to_string() }
                                            self.browser_path_last = self.browser_path_current.to_owned();
                                            self.browser_path_current = path.to_owned();
                                            self.update_all = true;
                                        }
                                    }
                                    let text_field = ui.add_enabled(true,  egui::TextEdit::singleline(&mut self.browser_path_line)
                                        .desired_width(self.section_browser_size_x - 60.0)
                                    );
                                    if text_field.lost_focus() && (self.browser_path_line != self.browser_path_current) {
                                        if !self.browser_path_line.ends_with("/") {
                                            self.browser_path_line += "/";
                                        }
                                        self.browser_path_last = self.browser_path_current.to_owned();
                                        self.browser_path_current = self.browser_path_line.to_owned();
                                        self.update_all = true;
                                        //println!("Changed AND Lost Focus");
                                    }
                                    text_field.on_hover_text_at_pointer(&self.browser_path_line);
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                        let dir_reset = ui.add_enabled(self.button_browser_directory_enabled, egui::Button::new("⟲"));
                                        if dir_reset.on_hover_text_at_pointer("Reset back to default.").clicked() {
                                            self.browser_path_current = self.default_path.to_owned();
                                            self.update_all = true;
                                        }
                                    });
                                });
                                ui.separator();
                                
                                //File Browser
                                egui::ScrollArea::vertical()
                                .id_source("1")
                                .show(ui, |ui| {
                                    ui.set_width(self.section_browser_size_x - 15.0);
                                    for folder in &self.browser_folder.list_folders {
                                        if folder.read_into {
                                            if ui.selectable_label(false, format!("🗀 {}", folder.name.to_owned())).clicked() {
                                                let path = format!("{}{}/", self.browser_path_current, folder.name.to_owned());
                                                self.browser_path_last = self.browser_path_current.to_owned();
                                                self.browser_path_current = path;
                                                self.update_all = true;
                                            };
                                        };
                                    };
                                });

                            });
                        });
                    });

                    //Files Section
                    ui.add_enabled_ui(self.section_files_enabled, |ui|{
                        let file_group = ui.group(|ui| {
                            ui.set_min_size(egui::vec2(self.section_files_size_x, self.section_files_size_y));
                            ui.vertical(|ui| {
                                ui.set_max_size(egui::vec2(self.section_files_size_x - 5.0, self.section_files_size_y - 5.0));
                                ui.horizontal(|ui| {
                                    ui.strong("Files");
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                        let unselect = ui.add_enabled(true, egui::Button::new("⟲".to_string()));
                                        if unselect.clicked() {
                                            for (index, _item) in self.table_files.iter().enumerate() {
                                                self.table_files_selected[index] = false;
                                            };
                                        };
                                        unselect.on_hover_text_at_pointer("De-select all.")
                                    });
                                });
                                ui.separator();
                                egui_extras::TableBuilder::new(ui)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::TOP))
                                .column(egui_extras::Column::auto().at_most(self.section_files_size_x - 5.0).clip(true))
                                .striped(true)
                                .body(|mut body| {
                                    for (index, item) in self.table_files.clone().iter().enumerate() {
                                        body.row(18.0, |mut row| {
                                            row.col(|ui| {
                                                ui.set_width(self.section_files_size_x - 5.0);
                                                let label = ui.toggle_value(&mut self.table_files_selected[index], item);
                                                if label.clicked() {
                                                    self.table_files_item_clicked = true 
                                                } else { 
                                                    self.table_files_item_clicked = false 
                                                };
                                                ctx.input(|keys| {
                                                    if self.table_files_item_clicked && keys.modifiers.shift && !keys.modifiers.ctrl {
                                                        let last_selected = self.table_files_last_selected.clone() as usize;
                                                        if index > last_selected{
                                                            let select = index - last_selected;
                                                            for i in 0..=select {
                                                                self.table_files_selected[last_selected+i] = true;
                                                            };
                                                        }
                                                        else if index < last_selected {
                                                            for i in index..=last_selected {
                                                                self.table_files_selected[i] = true;
                                                            };
                                                        };
                                                    };
                                                });
                                                if self.table_files_item_clicked {
                                                    self.table_files_last_selected = index as u32;
                                                };
                                                ui.end_row();
                                            });
                                        });
                                        if self.table_files_selected[index] {
                                            body.row(18.0, |mut row| {
                                                row.col(|ui| {
                                                    if ui.label(format!("---> {}", self.table_files_renamed[index].to_string())).secondary_clicked() {
                                                        
                                                    }
                                                    ui.end_row();
                                                });
                                            });
                                        };
                                    };
                                });
                            });
                        });

                        //Hovering
                        if file_group.response.hovered() {
                            self.section_files_hovered = true;
                            ctx.input(|keys| {
                                //Ctrl+A to select all.
                                if keys.modifiers.ctrl && keys.key_pressed(egui::Key::A) {
                                    for (index, _item) in self.table_files.iter().enumerate() {
                                        self.table_files_selected[index] = true;
                                    };
                                };

                                //If double clicking in the panel. De-select all items (Reset view)
                                if self.double_click_deselect_enabled &&keys.pointer.button_double_clicked(egui::PointerButton::Primary) 
                                && (!keys.modifiers.ctrl 
                                && !keys.modifiers.shift) 
                                && !self.table_files_item_clicked {
                                    for (index, _item) in self.table_files.iter().enumerate() {
                                        self.table_files_selected[index] = false;
                                    };
                                };
                            });
                        } else { self.section_files_hovered = false };
                    });

                    //Options Section
                    ui.add_enabled_ui(self.section_options_enabled, |ui|{
                        ui.group(|ui| {
                            ui.set_min_size(egui::vec2(self.section_options_size_x, self.section_options_size_y));
                            ui.set_max_size(egui::vec2(self.section_options_size_x, self.section_options_size_y));
                            ui.vertical(|ui| {
                                egui::ScrollArea::vertical()
                                .id_source("3")
                                .show(ui, |ui| {
                                    ui.set_max_size(egui::vec2(self.section_options_size_x - 5.0, self.section_options_size_y - 5.0));
                                    ui.horizontal(|ui| {
                                        ui.strong("Options");
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                            let reset = ui.add_enabled(true, egui::Button::new("⟲".to_string()));
                                            if reset.clicked() {
                                                
                                            };
                                            reset.on_hover_text_at_pointer("Reset back to defaults.")
                                        });
                                    });
                                    ui.separator();
    
                                    //Renaming Options
                                    ui.vertical(|ui| {
    
                                        //Name Options Priority #01
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.set_min_size(egui::vec2(self.section_options_size_x * 0.98, 60.0));
                                                ui.set_max_size(egui::vec2(self.section_options_size_x * 0.98, 60.0));
                                                ui.vertical(|ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label("Name");
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                            if ui.checkbox(&mut self.section_options_name_enabled, "").changed() {

                                                            };
                                                        });
                                                    });
                                                    ui.separator();
                                                    ui.add_enabled_ui(self.section_options_name_enabled, |ui| {
                                                        ui.horizontal(|ui| {
                                                            ui.label("Mode");
                                                            egui::ComboBox::new("CB1", "")
                                                            .selected_text(self.section_options_combobox_name.to_owned())
                                                            .show_ui(ui, |ui| {
                                                                if ui.selectable_label(false, "Keep").clicked() {
                                                                    self.section_options_combobox_value = NamingMode::Keep;
                                                                    self.section_options_combobox_name = "Keep".to_string();
                                                                };
                                                                if ui.selectable_label(false, "Remove").clicked() {
                                                                    self.section_options_combobox_value = NamingMode::Remove;
                                                                    self.section_options_combobox_name = "Remove".to_string();
                                                                };
                                                                if ui.selectable_label(false, "Reverse").clicked() {
                                                                    self.section_options_combobox_value = NamingMode::Reverse;
                                                                    self.section_options_combobox_name = "Reverse".to_string();
                                                                };
            
                                                            });
                                                        });
                                                        ui.horizontal(|ui| {
                                                            ui.label("  ");
                                                            ui.add_enabled(self.section_options_name_textbox_enabled, 
                                                                egui::TextEdit::singleline(&mut self.section_options_name_value)
                                                                .desired_width(self.section_options_size_x));
                                                        });
                                                    });
                                                });
                                            });
                                            match self.section_options_combobox_value {
                                                NamingMode::Keep => {self.section_options_name_textbox_enabled = false},
                                                NamingMode::Remove => {self.section_options_name_textbox_enabled = true},
                                                NamingMode::Reverse => {self.section_options_name_textbox_enabled = false}
                                            }
                                        });

                                        //Case Priority #02
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.set_min_size(egui::vec2(self.section_options_size_x * 0.98, 80.0));
                                                ui.set_max_size(egui::vec2(self.section_options_size_x * 0.98, 80.0));
                                                ui.vertical(|ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label("Case");
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                            ui.checkbox(&mut self.section_options_case_enabled, "");
                                                        });
                                                    });
                                                    ui.separator();
                                                    ui.add_enabled_ui(self.section_options_case_enabled, |ui| {
                                                        //Mode
                                                        ui.horizontal(|ui| {
                                                            ui.label("Mode");
                                                            egui::ComboBox::new("2", "")
                                                            .selected_text(self.section_options_case_combobox_name.to_owned())
                                                            .show_ui(ui, |ui| {
                                                                if ui.selectable_label(false, "Same").clicked() {
                                                                    self.section_options_case_combobox_value = CaseMode::Same;
                                                                    self.section_options_case_combobox_name = "Same".to_string();
                                                                };
                                                                if ui.selectable_label(false, "Upper").clicked() {
                                                                    self.section_options_case_combobox_value = CaseMode::Upper;
                                                                    self.section_options_case_combobox_name = "Upper".to_string();
                                                                };
                                                                if ui.selectable_label(false, "Lower").clicked() {
                                                                    self.section_options_case_combobox_value = CaseMode::Lower;
                                                                    self.section_options_case_combobox_name = "Lower".to_string();
                                                                };
                                                            });
                                                            //Match and Disable Widgets
                                                            match self.section_options_case_combobox_value {
                                                                CaseMode::Same => {
                                                                    self.section_options_case_widgets_enabled = false;
                                                                },
                                                                _ => {
                                                                    self.section_options_case_widgets_enabled = true;
                                                                }
                                                            }
                                                        });

                                                        //Except
                                                        ui.horizontal(|ui| {
                                                            ui.add_enabled_ui(self.section_options_case_widgets_enabled, |ui| {
                                                                ui.label("Except from");
                                                                let first_value = ui.add_enabled(true, 
                                                                    egui::DragValue::new(&mut self.section_options_case_from)
                                                                    .clamp_range(0..=25565)
                                                                    .speed(0.05)
                                                                );
                                                                if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                    self.section_options_case_from += 1;
                                                                } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                    if self.section_options_case_from >= 1 {
                                                                        self.section_options_case_from -= 1;
                                                                    }
                                                                };
                                                                if ui.small_button("➖").clicked() {
                                                                    if self.section_options_case_from >= 1 {
                                                                        self.section_options_case_from -= 1;
                                                                    }
                                                                };
    
                                                                ui.separator();
    
                                                                if ui.small_button("➕").clicked() {
                                                                    self.section_options_case_from += 1;
                                                                };
    
                                                                ui.label("to");
    
                                                                let second_value = ui.add_enabled(true, 
                                                                    egui::DragValue::new(&mut self.section_options_case_to)
                                                                    .clamp_range(self.section_options_case_from..=25565)
                                                                    .speed(0.05)
                                                                );
                                                                if second_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                    self.section_options_case_to += 1;
                                                                } else if second_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                    if self.section_options_case_to >= 1 {
                                                                        self.section_options_case_to -= 1;
                                                                    }
                                                                };
                                                                if ui.small_button("➖").clicked() {
                                                                    if self.section_options_case_to >= 1 {
                                                                        self.section_options_case_to -= 1;
                                                                    }
                                                                };
    
                                                                ui.separator();
    
                                                                if ui.small_button("➕").clicked() {
                                                                    self.section_options_case_to += 1;
                                                                };
                                                            });
                                                        });
                                                    });
                                                });
                                            });
                                        });

                                        //Replace Options Priority #03
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.set_min_size(egui::vec2(self.section_options_size_x * 0.98, 80.0));
                                                ui.set_max_size(egui::vec2(self.section_options_size_x * 0.98, 80.0));
                                                ui.vertical(|ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label("Replace");
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                            ui.checkbox(&mut self.section_options_replace_enabled, "");
                                                        });
                                                    });
                                                    ui.separator();
    
                                                    ui.add_enabled_ui(self.section_options_replace_enabled, |ui| {
                                                        ui.horizontal(|ui| {
                                                            ui.label("Replace ");
                                                            ui.add_enabled(true, 
                                                                egui::TextEdit::singleline(&mut self.section_options_replace_match_with)
                                                                .desired_width(self.section_options_size_x));
                                                        });
                                                        ui.horizontal(|ui| {
                                                            ui.label("With ");
                                                            ui.add_enabled(true, 
                                                                egui::TextEdit::singleline(&mut self.section_options_replace_replace_with)
                                                                .desired_width(self.section_options_size_x));
                                                        });
                                                        ui.separator();
                                                        ui.horizontal(|ui| {
                                                            ui.checkbox(&mut self.section_options_replace_match_first, "First");
                                                        });
                                                    });
                                                });
                                            });
                                        });
    
                                        //Remove Options Priority #04
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.set_min_size(egui::vec2(self.section_options_size_x * 0.98, 80.0));
                                                ui.set_max_size(egui::vec2(self.section_options_size_x * 0.98, 80.0));
                                                ui.vertical(|ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label("Remove");
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                            ui.checkbox(&mut self.section_options_remove_enabled, "");
                                                        });
                                                    });
                                                    ui.separator();
                                                    ui.add_enabled_ui(self.section_options_remove_enabled, |ui| {

                                                        // First / Last
                                                        ui.horizontal(|ui| {
    
                                                            //First
                                                            ui.group(|ui| {
                                                                ui.label("First");
                                                                let first_value = ui.add_enabled(true, 
                                                                    egui::DragValue::new(&mut self.section_options_remove_first)
                                                                    .clamp_range(0..=25565)
                                                                    .speed(0.05)
                                                                );
                                                                if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                    self.section_options_remove_first += 1;
                                                                } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                    if self.section_options_remove_first >= 1 {
                                                                        self.section_options_remove_first -= 1;
                                                                    }
                                                                };
                                                                if ui.small_button("➖").clicked() {
                                                                    if self.section_options_remove_first >= 1 {
                                                                        self.section_options_remove_first -= 1;
                                                                    }
                                                                };
    
                                                                ui.separator();
    
                                                                if ui.small_button("➕").clicked() {
                                                                    self.section_options_remove_first += 1;
                                                                };
                                                            });
    
                                                                ui.separator();
    
                                                            //Last
                                                            ui.group(|ui| {
                                                                ui.label("Last");
                                                                let first_value = ui.add_enabled(true, 
                                                                    egui::DragValue::new(&mut self.section_options_remove_last)
                                                                    .clamp_range(0..=25565)
                                                                    .speed(0.05)
                                                                );
                                                                if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                    self.section_options_remove_last += 1;
                                                                } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                    if self.section_options_remove_last >= 1 {
                                                                        self.section_options_remove_last -= 1;
                                                                    }
                                                                };
    
                                                                if ui.small_button("➖").clicked() {
                                                                    if self.section_options_remove_last >= 1 {
                                                                        self.section_options_remove_last -= 1;
                                                                    }
                                                                };
    
                                                                ui.separator();
    
                                                                if ui.small_button("➕").clicked() {
                                                                    self.section_options_remove_last += 1;
                                                                };
                                                            });
                                                        });

                                                        // To / From
                                                        ui.horizontal(|ui| {
    
                                                            //From
                                                            ui.group(|ui| {
                                                                ui.label("From ");
                                                                let first_value = ui.add_enabled(true, 
                                                                    egui::DragValue::new(&mut self.section_options_remove_from)
                                                                    .clamp_range(0..=25565)
                                                                    .speed(0.05)
                                                                );
                                                                if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                    self.section_options_remove_from += 1;
                                                                } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                    if self.section_options_remove_from >= 1 {
                                                                        self.section_options_remove_from -= 1;
                                                                    }
                                                                };
                                                                if ui.small_button("➖").clicked() {
                                                                    if self.section_options_remove_from >= 1 {
                                                                        self.section_options_remove_from -= 1;
                                                                    }
                                                                };
    
                                                                ui.separator();
    
                                                                if ui.small_button("➕").clicked() {
                                                                    self.section_options_remove_from += 1;
                                                                };
                                                            });
    
    
                                                            ui.separator();
    
                                                            //To
                                                            ui.group(|ui| {
                                                                ui.label("To");
                                                                let first_value = ui.add_enabled(true, 
                                                                    egui::DragValue::new(&mut self.section_options_remove_to)
                                                                    .clamp_range(self.section_options_remove_from..=25565)
                                                                    .speed(0.05)
                                                                );
                                                                if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                    self.section_options_remove_to += 1;
                                                                } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                    if self.section_options_remove_to >= 1 {
                                                                        self.section_options_remove_to -= 1;
                                                                    }
                                                                };
                                                                if ui.small_button("➖").clicked() {
                                                                    if self.section_options_remove_to >= 1 {
                                                                        self.section_options_remove_to -= 1;
                                                                    }
                                                                };
    
                                                                ui.separator();
    
                                                                if ui.small_button("➕").clicked() {
                                                                    self.section_options_remove_to += 1;
                                                                };
                                                            });
                                                        });
                                                    })
                                                });
                                            });
    
                                        });
    
                                        //Add Options Priority #05
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.vertical(|ui| {
                                                    ui.set_min_size(egui::vec2(self.section_options_size_x * 0.98, 85.0));
                                                    ui.set_max_size(egui::vec2(self.section_options_size_x * 0.98, 85.0));
                                                    ui.horizontal(|ui| {
                                                        ui.label("Add");
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                            ui.checkbox(&mut self.section_options_add_enabled, "");
                                                        });
                                                    });
                                                    ui.separator();
                                                    ui.add_enabled_ui(self.section_options_add_enabled, |ui| {

                                                        //Prefix
                                                        ui.horizontal(|ui| {
                                                            ui.label("Prefix");
                                                            ui.add(egui::TextEdit::singleline(&mut self.section_options_add_prefix)
                                                            .desired_width(self.section_options_size_x - 20.0));
                                                        });

                                                        //Insert
                                                        ui.horizontal(|ui| {
                                                            ui.label("Insert");
                                                            ui.add(egui::TextEdit::singleline(&mut self.section_options_add_insert)
                                                            .desired_width(self.section_options_size_x - 200.0));
                                                            ui.label("at");
                                                            let first_value = ui.add_enabled(true, 
                                                                egui::DragValue::new(&mut self.section_options_add_at)
                                                                .clamp_range(0..=25565)
                                                                .speed(0.05)
                                                            );
                                                            if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                self.section_options_add_at += 1;
                                                            } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                if self.section_options_add_at >= 1 {
                                                                    self.section_options_add_at -= 1;
                                                                }
                                                            };
                                                            if ui.small_button("➖").clicked() {
                                                                if self.section_options_add_at >= 1 {
                                                                    self.section_options_add_at -= 1;
                                                                }
                                                            };

                                                            ui.separator();

                                                            if ui.small_button("➕").clicked() {
                                                                self.section_options_add_at += 1;
                                                            };

                                                        });

                                                        //Suffix
                                                        ui.horizontal(|ui| {
                                                            ui.label("Suffix");
                                                            ui.add(egui::TextEdit::singleline(&mut self.section_options_add_suffix)
                                                            .desired_width(self.section_options_size_x - 20.0));
                                                        });
                                                    });
                                                });
                                            });
                                        });

                                        //Numbering Options Priority #06
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.vertical(|ui| {
                                                    ui.set_min_size(egui::vec2(self.section_options_size_x * 0.98, 65.0));
                                                    ui.set_max_size(egui::vec2(self.section_options_size_x * 0.98, 65.0));
                                                    ui.vertical(|ui| {
                                                        ui.horizontal(|ui| {
                                                            ui.label("Numbering");
                                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                                ui.checkbox(&mut self.section_options_numbering_enabled, "");
                                                            });
                                                        });
                                                        ui.separator();
                                                        ui.add_enabled_ui(self.section_options_numbering_enabled, |ui| {
                                                            //Mode and At
                                                            ui.horizontal(|ui| {
                                                                ui.label("Mode");
                                                                egui::ComboBox::new("6", "")
                                                                .selected_text(self.section_options_numbering_combobox_name.to_owned())
                                                                .show_ui(ui, |ui| {
                                                                    if ui.selectable_label(false, "None").clicked() {
                                                                        self.section_options_numbering_combobox_value = NumberingMode::None;
                                                                        self.section_options_numbering_combobox_name = "None".to_string();
                                                                    };
                                                                    if ui.selectable_label(false, "Prefix").clicked() {
                                                                        self.section_options_numbering_combobox_value = NumberingMode::Prefix;
                                                                        self.section_options_numbering_combobox_name = "Prefix".to_string();
                                                                    };
                                                                    if ui.selectable_label(false, "Suffix").clicked() {
                                                                        self.section_options_numbering_combobox_value = NumberingMode::Suffix;
                                                                        self.section_options_numbering_combobox_name = "Suffix".to_string();
                                                                    };
                                                                    if ui.selectable_label(false, "Prefix&Suffix").clicked() {
                                                                        self.section_options_numbering_combobox_value = NumberingMode::PrefixAndSuffix;
                                                                        self.section_options_numbering_combobox_name = "Pre+Suffix".to_string();
                                                                    };
                                                                    if ui.selectable_label(false, "Insert").clicked() {
                                                                        self.section_options_numbering_combobox_value = NumberingMode::Insert;
                                                                        self.section_options_numbering_combobox_name = "Insert".to_string();
                                                                    };
                                                                });

                                                                //Match and Disable Widgets
                                                                match self.section_options_numbering_combobox_value {
                                                                    NumberingMode::None => {
                                                                        self.section_options_numbering_insert_enabled = false;
                                                                        self.section_options_numbering_widgets_enabled = false;
                                                                    },
                                                                    NumberingMode::Insert => {
                                                                        self.section_options_numbering_insert_enabled = true;
                                                                        self.section_options_numbering_widgets_enabled = true;
                                                                    },
                                                                    NumberingMode::Prefix => {
                                                                        self.section_options_numbering_insert_enabled = false;
                                                                        self.section_options_numbering_widgets_enabled = true;
                                                                    },
                                                                    NumberingMode::Suffix => {
                                                                        self.section_options_numbering_insert_enabled = false;
                                                                        self.section_options_numbering_widgets_enabled = true;
                                                                    },
                                                                    NumberingMode::PrefixAndSuffix => {
                                                                        self.section_options_numbering_insert_enabled = false;
                                                                        self.section_options_numbering_widgets_enabled = true;
                                                                    }
                                                                }

                                                                //at
                                                                ui.add_enabled_ui(self.section_options_numbering_widgets_enabled, |ui| {
                                                                    ui.label("at");
                                                                    let first_value = ui.add_enabled(self.section_options_numbering_insert_enabled, 
                                                                        egui::DragValue::new(&mut self.section_options_numbering_at)
                                                                        .clamp_range(0..=25565)
                                                                        .speed(0.05)
                                                                    );
                                                                    if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                        self.section_options_numbering_at += 1;
                                                                    } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                        if self.section_options_numbering_at >= 1 {
                                                                            self.section_options_numbering_at -= 1;
                                                                        }
                                                                    };
                                                                    if ui.small_button("➖").clicked() {
                                                                        if self.section_options_numbering_at >= 1 {
                                                                            self.section_options_numbering_at -= 1;
                                                                        }
                                                                    };
        
                                                                    ui.separator();
        
                                                                    if ui.small_button("➕").clicked() {
                                                                        self.section_options_numbering_at += 1;
                                                                    };
                                                                });

                                                            });
                                                            
                                                            //Start and Pad
                                                            ui.horizontal(|ui| {
                                                                ui.add_enabled_ui(self.section_options_numbering_widgets_enabled, |ui| {
                                                                    ui.label("Start");
                                                                    let first_value = ui.add_enabled(true, 
                                                                        egui::DragValue::new(&mut self.section_options_numbering_start)
                                                                        .clamp_range(0..=999999)
                                                                        .speed(0.05)
                                                                    );
                                                                    if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                        self.section_options_numbering_start += 1;
                                                                    } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                        if self.section_options_numbering_start >= 1 {
                                                                            self.section_options_numbering_start -= 1;
                                                                        }
                                                                    };
                                                                    if ui.small_button("➖").clicked() {
                                                                        if self.section_options_numbering_start >= 1 {
                                                                            self.section_options_numbering_start -= 1;
                                                                        }
                                                                    };
        
                                                                    ui.separator();
        
                                                                    if ui.small_button("➕").clicked() {
                                                                        self.section_options_numbering_start += 1;
                                                                    };

                                                                    ui.label("Pad");
                                                                    let first_value = ui.add_enabled(true, 
                                                                        egui::DragValue::new(&mut self.section_options_numbering_pad)
                                                                        .clamp_range(0..=16)
                                                                        .speed(0.05)
                                                                    );
                                                                    if first_value.hovered() && ui.input(|input| {input.scroll_delta.y >= 1.0}){
                                                                        self.section_options_numbering_pad += 1;
                                                                    } else if first_value.hovered() && ui.input(|input| {input.scroll_delta.y <= -1.0}) {
                                                                        if self.section_options_numbering_pad >= 1 {
                                                                            self.section_options_numbering_pad -= 1;
                                                                        }
                                                                    };
                                                                    if ui.small_button("➖").clicked() {
                                                                        if self.section_options_numbering_pad >= 1 {
                                                                            self.section_options_numbering_pad -= 1;
                                                                        }
                                                                    };
        
                                                                    ui.separator();
        
                                                                    if ui.small_button("➕").clicked() {
                                                                        self.section_options_numbering_pad += 1;
                                                                    };
                                                                });
                                                            });
                                                        });
                                                    });
                                                });
                                            });
                                        });

                                        //Save Box
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.vertical(|ui| {
                                                    ui.set_min_size(egui::vec2(self.section_options_size_x * 0.98, 70.0));
                                                    ui.set_max_size(egui::vec2(self.section_options_size_x * 0.98, 70.0));
                                                    ui.vertical(|ui| {
                                                        ui.add_enabled_ui(self.section_options_save_enabled, |ui| {
                                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                                let button_save = ui.add_sized(egui::vec2(64.0, 70.0), egui::Button::new("Save"));
                                                                if button_save.clicked() && self.dialog_save_enabled {
                                                                    //Summon Dialog to confirm save
                                                                    self.window_save_open();
                                                                };
                                                                ui.vertical(|ui| {
                                                                    let button_preview = ui.button("Preview");
                                                                    if button_preview.clicked() && self.dialog_preview_enabled {
                                                                        //Summon Dialog to preview changes
                                                                        self.window_preview_open();
                                                                    }
                                                                });
                                                            });
                                                        });
                                                    });
                                                });
                                            });
                                        });
                                    });
                                });
                            });
                        });
                    });
                });
            });

            ui.separator();

            // Bottom Bar
            ui.horizontal(|ui| {
                ui.label(format!("OS: {}, \tVersion: {}, \tSelected Files: {}, \tModifications: {}", 
                    self.os_string.to_string(), 
                    self.version, 
                    self.table_files_selected_vec.len().to_string(), 
                    self.modifications_total)
                );
            });

            let mut elements: Vec<usize> = vec![];
            //Create Vector of all selected items
            for index in 0..self.table_files_selected.len() {
                if self.table_files_selected[index] {
                    elements.push(index);
                };
            };

            // Update Vector if something is changed
            if elements != self.table_files_selected_vec {
                //println!("Cleared table_files buffer");

                self.table_files_selected_vec.clear();
                for ele in elements {
                    self.table_files_selected_vec.push(ele);
                };
            };
        });

        // Fill selected_names & Make edits
        let mut selected_names:Vec<(String, String)> = vec![];
        let mut selected_total: u32 = 0;
        for (index, filename) in self.table_files.iter().enumerate() {
            let extension = Path::new(&filename).extension().unwrap_or_default().to_str().unwrap_or_default();
            let file: &str;
            let ext: &str;
            if extension.len() >= 1 {
                let (f, e) = filename.split_at(filename.len() - extension.len() - 1);
                file = f; ext = e;
            } else { 
                let (f, e) = filename.split_at(filename.len() - extension.len());
                file = f; ext = e;
            };
            let mut file: String = file.to_string();
            let ext: String = ext.to_string();
            // Modify
            if self.table_files_selected[index] {
                selected_total += 1;

                // #01 Name
                if self.section_options_name_enabled {
                    match self.section_options_combobox_value {
                        NamingMode::Remove => {
                            let removed: String;
                            removed = self.section_options_name_value.to_owned();
                            file = removed;
                        },
                        NamingMode::Reverse => {
                            let mut reversedvec: Vec<(usize, String)> = vec![];
                            for (usize, char) in file.to_owned().chars().enumerate() {
                                reversedvec.push((usize, char.to_string()));
                            }
                            reversedvec.reverse();
                            
                            let mut reversed: String = "".to_string();
                            for (_usize, mut char) in reversedvec {
                                match char.as_str() {
                                    ">" => {char = "<".to_string()},
                                    "<" => {char = ">".to_string()},
                                    "(" => {char = ")".to_string()},
                                    ")" => {char = "(".to_string()},
                                    "{" => {char = "}".to_string()},
                                    "}" => {char = "{".to_string()},
                                    "[" => {char = "]".to_string()},
                                    "]" => {char = "[".to_string()},
                                    "/" => {char = "\\".to_string()},
                                    "\\" => {char = "/".to_string()},
                                    _ => {}
                                }
                                reversed = reversed + &char;
                            };
                            file = reversed;
                        },
                        NamingMode::Keep => {
                            // Don't do anything
                        }
                    }
                }
    
                // #02 Case
                if self.section_options_case_enabled {
                    let clamped_from = self.section_options_case_from.clamp(0, file.len() as u32);
                    let clamped_to = self.section_options_case_to.clamp(clamped_from, file.len() as u32);
                    match self.section_options_case_combobox_value {
                        CaseMode::Lower => {
                            if self.section_options_case_to >= 1 {
                                //Determine the kept section.
                                let keep: String;
                                if clamped_from == 0 && clamped_to == 0 {
                                    keep = "".to_string();
                                } else {
                                    keep = file[clamped_from as usize..=clamped_to as usize - 1].to_string();
                                };
                                if clamped_from == 0 {
                                    file = format!(
                                        "{}{}",
                                        keep,
                                        file[clamped_to as usize..=file.len() - 1].to_string().to_lowercase()
                                    )
                                } else {
                                    file = format!(
                                        "{}{}{}",
                                        file[0..=clamped_from as usize - 1].to_string().to_lowercase(),
                                        keep,
                                        file[clamped_to as usize..=file.len() - 1].to_string().to_lowercase()
                                    );
                                };
                            } else {
                                file = file.to_lowercase();
                            };
                        },
    
                        CaseMode::Upper => {
                            if self.section_options_case_to >= 1 {
                                //Determine the kept section.
                                let keep: String;
                                if clamped_from == 0 && clamped_to == 0 {
                                    keep = "".to_string();
                                } else {
                                    keep = file[clamped_from as usize..=clamped_to as usize - 1].to_string();
                                };
                                if clamped_from == 0 {
                                    file = format!(
                                        "{}{}",
                                        keep,
                                        file[clamped_to as usize..=file.len() - 1].to_string().to_uppercase()
                                    );
                                } else {
                                    file = format!(
                                        "{}{}{}",
                                        file[0..=clamped_from as usize - 1].to_string().to_uppercase(),
                                        keep,
                                        file[clamped_to as usize..=file.len() - 1].to_string().to_uppercase()
                                    );
                                };
                            } else {
                                file = file.to_uppercase();
                            };
                        },
    
                        CaseMode::Same => {
                            // Do nothing..
                        }
                    }
                }
    
                // #03 Replace
                if self.section_options_replace_enabled {
                    if self.section_options_replace_match_first == true {
                    let find = file.find(&self.section_options_replace_match_with);
                    match find {
                        Some(stringindex) => {
                            let (start, _mid) = file.split_at(stringindex);
                            let (_mid, end) = file.split_at(stringindex + self.section_options_replace_match_with.len());
                            file = format!("{}{}{}", start, self.section_options_replace_replace_with, end);
                        },
    
                        None => {
                            
                        }
                    }} else {
                        file = file.as_str().replace(self.section_options_replace_match_with.as_str(), self.section_options_replace_replace_with.as_str());
                    };
                }
    
                // #04 Remove
                if self.section_options_remove_enabled {
                    let clamped_first = self.section_options_remove_first.clamp(0, file.len() as u32);
                        
                    // Remove some chars from the beginning
                    if file.len() >= 1{
                        //file = file[clamped_first as usize..file.len()].to_string();
                        file = file.chars().skip(clamped_first as usize).collect();

                    }
                    
                    // Remove some chars from the end
                    let clamped_last = self.section_options_remove_last.clamp(0, file.len() as u32);
                    if file.len() >= 1 {
                        //file = file[0..file.len() - clamped_last as usize].to_string();
                        file = file.chars().take(file.char_indices().count() - clamped_last as usize).collect();
                    }
    
                    // Remove a section
                    let clamped_from = self.section_options_remove_from.clamp(0, file.len() as u32);
                    let clamped_to = self.section_options_remove_to.clamp(clamped_from, file.len() as u32);
                    if clamped_to >= 1 && file.len() >= 1{
                        let mut file_vec: Vec<char> = vec![];
                        let mut file_chars: String = "".to_string();
                        for character in file.chars() {
                            file_vec.push(character);
                        }
                        for character in &file_vec[0..clamped_from as usize] {
                            file_chars.push(character.to_owned());
                        }
                        for character in &file_vec[clamped_to as usize..file_vec.len()] {
                            file_chars.push(character.to_owned());
                        }
                        file = file_chars;
                        
                    }
                }
    
                // #05 Add
                if self.section_options_add_enabled {
                    if self.section_options_add_prefix.len() >= 1 {
                        file = format!("{}{}", self.section_options_add_prefix, file);
                    }
                    if self.section_options_add_insert.len() >= 1 {
                        let add_at: u32 = self.section_options_add_at.clamp(0, file.len() as u32);
                        let (start, end) = file.split_at(add_at as usize);
                        file = format!("{}{}{}", start, self.section_options_add_insert, end);
                    }
                    if self.section_options_add_suffix.len() >= 1 {
                        file = format!("{}{}", file, self.section_options_add_suffix);
                    }
                }
    
                // #06 Numbering
                if self.section_options_numbering_enabled {
                    //Padding
                    let number_index: u32 = self.section_options_numbering_start + selected_total - 1;
                    let number_length: u32 =  number_index.to_string().len() as u32;
                    let mut number: String = number_index.to_string();
                    if number_length < self.section_options_numbering_pad {
                        let padding: u32 = self.section_options_numbering_pad - number_length;
                        for _ in 0..padding {
                            number.insert(0, '0');
                        }
                    }
    
                    //Match and insert
                    match self.section_options_numbering_combobox_value {
                        NumberingMode::Prefix => {
                            file = format!("{}{}", number, file);
                        },
                        NumberingMode::Suffix => {
                            file = format!("{}{}", file, number);
                        },
                        NumberingMode::Insert => {
                            let number_at: u32 = self.section_options_numbering_at.clamp(0, file.len() as u32);
                            let (start, end) = file.split_at(number_at as usize);
                            file = format!("{}{}{}", start, number, end);
                        },
                        NumberingMode::PrefixAndSuffix => {
                            file = format!("{}{}{}", number, file, number);
                        },
                        _ => {}
                    }
                }
                
            }
            selected_names.push((file, ext));
        };
        self.table_files_selected_total = selected_total;

        // End Sections, re add names to table if something is changed.
        if selected_names.len() >= 1 {
            self.table_files_renamed.clear();
            for (name, ext) in selected_names {
                self.table_files_renamed.push(format!("{}{}", name, ext));
            }
        };

        // Get modifications total
        {
            let mut total:u16 = 0;
            if self.section_options_name_enabled { total += 1 };
            if self.section_options_case_enabled { total += 1 };
            if self.section_options_replace_enabled { total += 1};
            if self.section_options_remove_enabled { total += 1 };
            if self.section_options_add_enabled { total += 1};
            if self.section_options_numbering_enabled { total += 1};
            self.modifications_total = total;
            if total == 0 {
                self.dialog_preview_enabled = false;
                self.dialog_save_enabled = false;
            } else {
                self.dialog_preview_enabled = true;
                self.dialog_save_enabled = true;
            };
        };
        
        //Enable / Disable; Undo button
        if self.edits_undo.len() >= 1 {
            self.button_undo_enabled = true;
        } else { self.button_undo_enabled = false };

        //Enable / Disable; Redo button
        if self.edits_redo.len() >= 1 {
            self.button_redo_enabled = true;
        } else { self.button_redo_enabled = false };

        // Enable / Disable; Save / Preview button
        if self.table_files_selected_vec.len() >= 1 && self.modifications_total >= 1 {
            self.section_options_save_enabled = true;
        } else { self.section_options_save_enabled = false };
    }
}