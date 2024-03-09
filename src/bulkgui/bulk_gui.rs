use crate::bulkgui::bulk_enum::*;

#[derive(Debug, Clone)]
pub struct Folder {
    pub path_full: String,
    pub list_files: Vec<FolderItem>,
    pub list_folders: Vec<FolderItem>,
    pub list_all_items: Vec<FolderItem>
}

#[derive(Debug, Clone)]
pub struct FolderItem {
    pub is_dir: bool,
    pub name: String,
    pub read_into: bool,
    pub read_only: bool,
    pub full_path: String
}

pub struct EdittedItem {
    pub name_original: String,
    pub name_edited: String,
    pub path_original: String,
    pub path_edited: String
}

pub struct Edit {
    pub tag: String,
    pub items: Vec<EdittedItem>,
    pub edits: u32,
}

pub struct BulkGui {
    pub os_string: &'static str,
    pub version: &'static str,
    pub program_name: &'static str,
    pub first_cycle: bool,
    pub update_all: bool,
    pub show_all_items: bool,

    pub errors: Vec<String>,
    pub edits_redo: Vec<Edit>,
    pub edits_undo: Vec<Edit>,

    pub dialog_open_save: bool,
    pub dialog_save_enabled: bool,
    pub dialog_open_saving: bool,
    pub dialog_saving_enabled: bool,
    pub dialog_open_preview: bool,
    pub dialog_preview_enabled: bool,
    pub dialog_open_error: bool,
    pub dialog_error_enabled: bool,
    pub dialog_open_redo: bool,
    pub dialog_redo_enabled: bool,
    pub dialog_open_undo: bool,
    pub dialog_undo_enabled: bool,

    pub files_selected: Vec<String>,
    pub modifications_total: u16,
    pub window_min_size: Vec<f32>,
    pub window_size: egui::Vec2,
    pub default_path: String,

    pub section_browser_enabled: bool,
    pub section_files_enabled: bool,
    pub section_options_enabled: bool,
    pub section_options_save_enabled: bool,

    pub section_files_hovered: bool,

    pub browser_folder: Folder,
    pub browser_list_folders: Vec<String>,
    pub browser_path_current: String,
    pub browser_path_last: String,
    pub browser_path_line: String,

    pub table_files: Vec<String>,
    pub table_files_selected: Vec<bool>,
    pub table_files_selected_vec: Vec<usize>,
    pub table_files_renamed: Vec<String>,
    pub table_files_item_clicked: bool,
    pub table_files_last_selected: u32,
    pub table_files_selected_total: u32,
    
    pub section_browser_size_x: f32,
    pub section_browser_percentage_min: u8,
    pub section_browser_percentage_max: u8,
    pub section_browser_percentage_current: u8,
    pub section_browser_size_y: f32,

    pub section_files_size_x: f32,
    pub section_files_percentage_min: u8,
    pub section_files_percentage_max: u8,
    pub section_files_percentage_current: u8,
    pub section_files_size_y: f32,

    pub section_options_size_x: f32,
    pub section_options_percentage_min: u8,
    pub section_options_percentage_current: u8,
    pub section_options_size_y: f32,

    pub section_options_name_enabled: bool,
    pub section_options_name_textbox_enabled: bool,
    pub section_options_combobox_name: String,
    pub section_options_combobox_value: NamingMode,
    pub section_options_combobox_default_value: NamingMode,
    pub section_options_name_value: String,

    pub section_options_case_enabled: bool,
    pub section_options_case_widgets_enabled: bool,
    pub section_options_case_combobox_name: String,
    pub section_options_case_combobox_value: CaseMode,
    pub section_options_case_combobox_default_value: CaseMode,
    pub section_options_case_combobox_label: String,
    pub section_options_case_from: u32,
    pub section_options_case_to: u32,

    pub section_options_replace_enabled: bool,
    pub section_options_replace_match_with: String,
    pub section_options_replace_replace_with: String,
    pub section_options_replace_match_first: bool,

    pub section_options_remove_enabled: bool,
    pub section_options_remove_first: u32,
    pub section_options_remove_last: u32,
    pub section_options_remove_to: u32,
    pub section_options_remove_from: u32,

    pub section_options_add_enabled: bool,
    pub section_options_add_prefix: String,
    pub section_options_add_insert: String,
    pub section_options_add_suffix: String,
    pub section_options_add_at: u32,

    pub section_options_numbering_enabled: bool,
    pub section_options_numbering_widgets_enabled: bool,
    pub section_options_numbering_insert_enabled: bool,
    pub section_options_numbering_combobox_name: String,
    pub section_options_numbering_combobox_value: NumberingMode,
    pub section_options_numbering_combobox_default_value: NumberingMode,
    pub section_options_numbering_combobox_label: String,
    pub section_options_numbering_at: u32,
    pub section_options_numbering_start: u32,
    pub section_options_numbering_pad: u32,

    pub button_defaults_enabled: bool,
    pub button_undo_enabled: bool,
    pub button_redo_enabled: bool,
    pub button_browser_up_enabled: bool,
    pub button_browser_directory_enabled: bool,

    pub checkbox_lock_section_resizing: bool,
    pub double_click_deselect_enabled: bool,

    pub saving_progress: f32,
    pub window_error_index: usize,
    pub window_error_text_box: String
}