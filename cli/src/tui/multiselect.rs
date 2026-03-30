/// A group of items in a tab
pub struct ItemGroup {
    pub label: String,
    pub items: Vec<SelectItem>,
}

/// A tab containing grouped items
pub struct Tab {
    pub name: String,
    pub groups: Vec<ItemGroup>,
}

/// Action to perform when a confirm dialog is accepted
#[derive(Clone, PartialEq)]
pub enum ConfirmAction {
    Proceed,
    UpdateAll,
    UninstallAll,
    RemoveSource {
        source: String,
        packages: Vec<String>,
    },
}

/// An item in the multi-select list
#[derive(Clone)]
pub struct SelectItem {
    pub label: String,
    pub description: String,
    pub selected: bool,
    /// Tag shown before the label (e.g., "agent", "skill")
    pub tag: Option<String>,
    /// Suffix annotation (e.g., "detected", dependency info)
    pub suffix: Option<String>,
    /// Whether this item is locked (auto-selected as dependency)
    pub locked: bool,
    /// Whether this item is currently installed
    pub installed: bool,
    /// Scope where the item is installed: project, global, or both.
    pub installed_scope: Option<String>,
    /// Whether the installed copy is outdated (source changed since install)
    pub outdated: bool,
}

#[derive(Clone)]
pub struct RepoOption {
    pub label: String,
    pub source: String,
}

pub struct RepoDialog {
    pub options: Vec<RepoOption>,
    pub cursor: usize,
    pub input_mode: bool,
    pub input: String,
}

/// Tabbed multi-select with grouped items
pub struct TabbedSelect {
    pub title: String,
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    pub cursor: usize,
    pub scroll: usize,
    pub multi: bool,
    /// Whether the primary action button currently has keyboard focus.
    pub action_button_focused: bool,
    /// Whether advancing without new selections is allowed.
    pub allow_empty_confirm: bool,
    /// Current package source label shown in the header.
    pub source_label: Option<String>,
    /// Known selectable sources.
    pub source_options: Vec<RepoOption>,
    /// Step indicator (e.g., "1/3")
    pub step: Option<String>,
    /// Short labels for each step in the flow.
    pub step_labels: Vec<String>,
    /// Optional summary shown in the final confirmation dialog.
    pub confirm_summary: Option<String>,
    /// Temporary message shown in status bar
    pub flash_message: Option<String>,
    /// Confirmation dialog (message + action)
    pub confirm_dialog: Option<(String, ConfirmAction)>,
    /// Scroll offset within the confirmation dialog body.
    pub confirm_dialog_scroll: usize,
    /// Repo selector / add-source dialog.
    pub repo_dialog: Option<RepoDialog>,
    /// Layout areas from last render (for mouse hit testing)
    pub layout_tab_bar: ratatui::layout::Rect,
    pub layout_list: ratatui::layout::Rect,
    /// Exact clickable primary action area from the last render.
    pub action_button_area: ratatui::layout::Rect,
    /// Exact clickable source chip area from the last render.
    pub source_chip_area: ratatui::layout::Rect,
    /// Exact clickable step badge areas from the last render.
    pub step_hit_areas: Vec<ratatui::layout::Rect>,
    /// Exact clickable tab areas from the last render.
    pub tab_hit_areas: Vec<ratatui::layout::Rect>,
    /// Inner content area of the repo dialog from the last render.
    pub repo_dialog_inner: ratatui::layout::Rect,
    /// Visible list rows from the last render, mapped to item indices.
    pub rendered_list_rows: Vec<Option<usize>>,
    /// Total rendered row count from the last draw.
    pub rendered_total_rows: usize,
    /// Visible list height from the last render.
    pub list_visible_rows: usize,
}

impl TabbedSelect {
    pub fn new(title: &str, tabs: Vec<Tab>, multi: bool) -> Self {
        Self {
            title: title.to_string(),
            tabs,
            active_tab: 0,
            cursor: 0,
            scroll: 0,
            multi,
            action_button_focused: false,
            allow_empty_confirm: false,
            source_label: None,
            source_options: Vec::new(),
            step: None,
            step_labels: Vec::new(),
            confirm_summary: None,
            flash_message: None,
            confirm_dialog: None,
            confirm_dialog_scroll: 0,
            repo_dialog: None,
            layout_tab_bar: ratatui::layout::Rect::default(),
            layout_list: ratatui::layout::Rect::default(),
            action_button_area: ratatui::layout::Rect::default(),
            source_chip_area: ratatui::layout::Rect::default(),
            step_hit_areas: Vec::new(),
            tab_hit_areas: Vec::new(),
            repo_dialog_inner: ratatui::layout::Rect::default(),
            rendered_list_rows: Vec::new(),
            rendered_total_rows: 0,
            list_visible_rows: 0,
        }
    }

    pub fn with_step(mut self, step: &str) -> Self {
        self.step = Some(step.to_string());
        self
    }

    pub fn with_step_labels(mut self, labels: &[&str]) -> Self {
        self.step_labels = labels.iter().map(|label| label.to_string()).collect();
        self
    }

    pub fn allow_empty_confirm(mut self, allow: bool) -> Self {
        self.allow_empty_confirm = allow;
        self
    }

    pub fn with_confirm_summary(mut self, summary: String) -> Self {
        self.confirm_summary = Some(summary);
        self
    }

    pub fn with_source_selector(mut self, label: String, options: Vec<RepoOption>) -> Self {
        self.source_label = Some(label);
        self.source_options = options;
        self
    }

    pub fn step_position(&self) -> Option<(usize, usize)> {
        let step = self.step.as_ref()?;
        let (cur, tot) = step.split_once('/')?;
        Some((cur.parse().ok()?, tot.parse().ok()?))
    }

    pub fn open_repo_dialog(&mut self) {
        self.repo_dialog = Some(RepoDialog {
            options: self.source_options.clone(),
            cursor: 0,
            input_mode: false,
            input: String::new(),
        });
    }

    /// Flat list of all items in current tab (across all groups)
    fn flat_items(&self) -> Vec<(usize, usize, &SelectItem)> {
        let tab = &self.tabs[self.active_tab];
        let mut flat = Vec::new();
        for (gi, group) in tab.groups.iter().enumerate() {
            for (ii, item) in group.items.iter().enumerate() {
                flat.push((gi, ii, item));
            }
        }
        flat
    }

    fn flat_items_mut(&mut self) -> Vec<(usize, usize)> {
        let tab = &self.tabs[self.active_tab];
        let mut flat = Vec::new();
        for (gi, group) in tab.groups.iter().enumerate() {
            for (ii, _) in group.items.iter().enumerate() {
                flat.push((gi, ii));
            }
        }
        flat
    }

    pub fn item_count(&self) -> usize {
        self.tabs[self.active_tab]
            .groups
            .iter()
            .map(|g| g.items.len())
            .sum()
    }

    pub fn move_up(&mut self) {
        let count = self.item_count();
        if count == 0 {
            return;
        }
        if self.cursor > 0 {
            self.cursor -= 1;
        } else {
            self.cursor = count - 1;
        }
        self.action_button_focused = false;
        self.adjust_scroll();
    }

    pub fn move_down(&mut self) {
        let count = self.item_count();
        if count == 0 {
            return;
        }
        if self.cursor < count - 1 {
            self.cursor += 1;
        } else {
            self.cursor = 0;
        }
        self.action_button_focused = false;
        self.adjust_scroll();
    }

    pub fn next_tab(&mut self) {
        if self.tabs.len() > 1 {
            self.active_tab = (self.active_tab + 1) % self.tabs.len();
            self.cursor = 0;
            self.scroll = 0;
            self.action_button_focused = false;
        }
    }

    pub fn prev_tab(&mut self) {
        if self.tabs.len() > 1 {
            self.active_tab = if self.active_tab > 0 {
                self.active_tab - 1
            } else {
                self.tabs.len() - 1
            };
            self.cursor = 0;
            self.scroll = 0;
            self.action_button_focused = false;
        }
    }

    pub fn toggle(&mut self) {
        let indices = self.flat_items_mut();
        if let Some(&(gi, ii)) = indices.get(self.cursor) {
            let item = &mut self.tabs[self.active_tab].groups[gi].items[ii];
            if item.locked {
                return;
            }
            if self.multi {
                item.selected = !item.selected;
            } else {
                // Single select: deselect all in all tabs, select current
                for tab in &mut self.tabs {
                    for group in &mut tab.groups {
                        for item in &mut group.items {
                            item.selected = false;
                        }
                    }
                }
                self.tabs[self.active_tab].groups[gi].items[ii].selected = true;
            }
        }
    }

    pub fn toggle_all(&mut self) {
        let tab = &self.tabs[self.active_tab];
        let all_selected = tab
            .groups
            .iter()
            .flat_map(|g| &g.items)
            .filter(|i| !i.locked)
            .all(|i| i.selected);

        let tab = &mut self.tabs[self.active_tab];
        for group in &mut tab.groups {
            for item in &mut group.items {
                if !item.locked {
                    item.selected = !all_selected;
                }
            }
        }
    }

    /// Get all selected items across all tabs as (tab_name, item_label)
    pub fn all_selected(&self) -> Vec<(&str, &str)> {
        let mut selected = Vec::new();
        for tab in &self.tabs {
            for group in &tab.groups {
                for item in &group.items {
                    if item.selected {
                        selected.push((tab.name.as_str(), item.label.as_str()));
                    }
                }
            }
        }
        selected
    }

    pub fn selected_count(&self) -> usize {
        self.tabs[self.active_tab]
            .groups
            .iter()
            .flat_map(|g| &g.items)
            .filter(|i| i.selected)
            .count()
    }

    pub fn total_selected(&self) -> usize {
        self.tabs
            .iter()
            .flat_map(|t| &t.groups)
            .flat_map(|g| &g.items)
            .filter(|i| i.selected)
            .count()
    }

    pub fn set_visible_height(&mut self, height: usize) {
        self.list_visible_rows = height;
        self.clamp_scroll();
    }

    /// Compute the rendered row index for the current cursor item.
    ///
    /// Rendered rows include group headers and blank separators that don't
    /// exist in the flat item index, so we walk the tab structure to count them.
    fn cursor_row(&self) -> usize {
        let tab = &self.tabs[self.active_tab];
        let mut row = 0;
        let mut flat_idx = 0;

        for (gi, group) in tab.groups.iter().enumerate() {
            if !group.label.is_empty() && tab.groups.len() > 1 {
                if gi > 0 {
                    row += 1; // blank separator between groups
                }
                row += 1; // group header
            }
            for _ in &group.items {
                if flat_idx == self.cursor {
                    return row;
                }
                row += 1;
                flat_idx += 1;
            }
        }
        row
    }

    fn adjust_scroll(&mut self) {
        let visible = self.list_visible_rows.max(1);
        let row = self.cursor_row();
        // Reserve space for a description line below the cursor
        let row_end = row + 2;

        if row < self.scroll {
            self.scroll = row;
        } else if row_end >= self.scroll + visible {
            self.scroll = row_end.saturating_sub(visible - 1);
        }
        self.clamp_scroll();
    }

    pub fn scroll_up(&mut self, rows: usize) {
        self.scroll = self.scroll.saturating_sub(rows);
    }

    pub fn scroll_down(&mut self, rows: usize) {
        self.scroll = self.scroll.saturating_add(rows);
        self.clamp_scroll();
    }

    fn clamp_scroll(&mut self) {
        self.scroll = self.scroll.min(self.max_scroll());
    }

    fn max_scroll(&self) -> usize {
        self.rendered_total_rows
            .saturating_sub(self.list_visible_rows.max(1))
    }

    /// Select an item by label across all tabs (used for auto-dependency selection)
    pub fn select_by_label(&mut self, label: &str, lock: bool) {
        for tab in &mut self.tabs {
            for group in &mut tab.groups {
                for item in &mut group.items {
                    if item.label == label {
                        item.selected = true;
                        if lock {
                            item.locked = true;
                        }
                    }
                }
            }
        }
    }

    /// Deselect an item by label (only if not locked)
    pub fn deselect_by_label(&mut self, label: &str) {
        for tab in &mut self.tabs {
            for group in &mut tab.groups {
                for item in &mut group.items {
                    if item.label == label && !item.locked {
                        item.selected = false;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ItemGroup, SelectItem, Tab, TabbedSelect};

    fn item(label: &str) -> SelectItem {
        SelectItem {
            label: label.to_string(),
            description: String::new(),
            selected: false,
            tag: None,
            suffix: None,
            locked: false,
            installed: false,
            installed_scope: None,
            outdated: false,
        }
    }

    #[test]
    fn adjust_scroll_accounts_for_group_headers() {
        // Two groups with 3 items each. Rendered rows:
        // row 0: group1 header
        // row 1: item 0
        // row 2: item 1
        // row 3: item 2
        // row 4: blank separator
        // row 5: group2 header
        // row 6: item 3
        // row 7: item 4
        // row 8: item 5
        let mut select = TabbedSelect::new(
            "Test",
            vec![Tab {
                name: "Items".into(),
                groups: vec![
                    ItemGroup {
                        label: "Group A".into(),
                        items: vec![item("a1"), item("a2"), item("a3")],
                    },
                    ItemGroup {
                        label: "Group B".into(),
                        items: vec![item("b1"), item("b2"), item("b3")],
                    },
                ],
            }],
            true,
        );

        select.list_visible_rows = 5;
        select.rendered_total_rows = 9;

        // Move cursor to item 4 (b2, rendered row 7)
        select.cursor = 4;
        select.adjust_scroll();
        // row_end = 7 + 2 = 9, 9 >= 0 + 5, scroll = 9 - 4 = 5
        assert!(
            select.scroll >= 4,
            "scroll should advance past group headers, got {}",
            select.scroll
        );

        // Cursor item row 7 must be within visible window [scroll, scroll+5)
        assert!(
            select.cursor_row() >= select.scroll
                && select.cursor_row() < select.scroll + select.list_visible_rows,
            "cursor row {} not in visible window [{}, {})",
            select.cursor_row(),
            select.scroll,
            select.scroll + select.list_visible_rows
        );
    }

    #[test]
    fn cursor_row_matches_flat_when_no_groups() {
        let mut select = TabbedSelect::new(
            "Test",
            vec![Tab {
                name: "Items".into(),
                groups: vec![ItemGroup {
                    label: String::new(),
                    items: vec![item("a"), item("b"), item("c")],
                }],
            }],
            true,
        );

        select.cursor = 0;
        assert_eq!(select.cursor_row(), 0);
        select.cursor = 2;
        assert_eq!(select.cursor_row(), 2);
    }

    #[test]
    fn wheel_scroll_clamps_at_content_bounds() {
        let mut select = TabbedSelect::new(
            "Test",
            vec![Tab {
                name: "Items".into(),
                groups: vec![ItemGroup {
                    label: String::new(),
                    items: vec![item("a"), item("b"), item("c")],
                }],
            }],
            true,
        );

        select.list_visible_rows = 4;
        select.rendered_total_rows = 10;

        select.scroll_down(3);
        assert_eq!(select.scroll, 3);

        select.scroll_down(10);
        assert_eq!(select.scroll, 6);

        select.scroll_up(2);
        assert_eq!(select.scroll, 4);

        select.scroll_up(10);
        assert_eq!(select.scroll, 0);
    }
}
