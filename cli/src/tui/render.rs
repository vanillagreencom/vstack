use super::multiselect::TabbedSelect;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph, Tabs};

pub fn draw_tabbed_select(frame: &mut Frame, select: &mut TabbedSelect) {
    let area = frame.area();

    frame.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        area,
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title
            Constraint::Length(1), // steps
            Constraint::Length(2), // separator above tabs
            Constraint::Length(1), // tabs
            Constraint::Length(1), // separator below tabs
            Constraint::Min(5),    // list
            Constraint::Length(2), // status
            Constraint::Length(3), // help (wraps on narrow terminals)
        ])
        .split(area);

    // Store layout areas for mouse hit testing
    select.layout_tab_bar = chunks[3];
    select.layout_list = chunks[5];
    select.source_chip_area = Rect::default();

    draw_header(
        frame, chunks[0], chunks[1], chunks[2], chunks[3], chunks[4], select,
    );
    draw_list(frame, chunks[5], select);
    draw_status(frame, chunks[6], select);
    draw_help(frame, chunks[7], select);

    // Confirm dialog overlay
    if let Some((message, _)) = select.confirm_dialog.clone() {
        draw_confirm_dialog(frame, select, &message);
    } else if select.repo_dialog.is_some() {
        draw_repo_dialog(frame, select);
    }
}

fn build_tab_titles(select: &TabbedSelect) -> Vec<Line<'static>> {
    select
        .tabs
        .iter()
        .enumerate()
        .map(|(i, tab)| {
            let is_active = i == select.active_tab;
            let count: usize = tab
                .groups
                .iter()
                .flat_map(|g| &g.items)
                .filter(|item| item.selected)
                .count();

            let name_style = if is_active {
                Style::default().fg(Color::Cyan).bold()
            } else {
                Style::default().fg(Color::DarkGray)
            };

            if tab.name.starts_with("Updates") {
                let base = tab.name.split('(').next().unwrap_or(&tab.name).trim();
                let n: usize = tab.groups.iter().flat_map(|g| &g.items).count();
                let mut spans = vec![Span::styled(format!(" {base} "), name_style)];
                spans.push(Span::styled(
                    format!("({n})"),
                    Style::default().fg(Color::Yellow),
                ));
                if count > 0 {
                    spans.push(Span::styled(
                        format!(" +{count}"),
                        Style::default().fg(Color::Magenta),
                    ));
                }
                spans.push(Span::raw(" "));
                return Line::from(spans);
            }

            if count > 0 {
                Line::from(vec![
                    Span::styled(format!(" {} ", tab.name), name_style),
                    Span::styled(format!("({count})"), Style::default().fg(Color::Magenta)),
                    Span::raw(" "),
                ])
            } else {
                Line::from(Span::styled(format!(" {} ", tab.name), name_style))
            }
        })
        .collect()
}

fn draw_header(
    frame: &mut Frame,
    title_area: Rect,
    step_area: Rect,
    sep_area: Rect,
    tab_area: Rect,
    sep2_area: Rect,
    select: &mut TabbedSelect,
) {
    select.step_hit_areas.clear();

    // Title line
    let title_spans = vec![
        Span::styled(
            " vstack ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Indexed(45))
                .bold(),
        ),
        Span::styled(
            format!("  {}", select.title),
            Style::default().fg(Color::White).bold(),
        ),
    ];
    let default_step_labels = ["Packages", "Scope", "Harnesses", "Method"];

    let top_y = title_area.y.saturating_add(1);
    let brand_title_width = Line::from(title_spans.clone()).width() as u16;
    frame.render_widget(
        Paragraph::new(Line::from(title_spans)),
        Rect {
            x: title_area.x,
            y: top_y,
            width: brand_title_width.min(title_area.width),
            height: 1,
        },
    );

    if let Some(ref source_label) = select.source_label {
        let raw = format!(" repo: {} ▾ ", source_label);
        let max_inner = title_area.width.saturating_sub(4) as usize;
        let clipped = if raw.chars().count() > max_inner && max_inner > 3 {
            let keep = max_inner.saturating_sub(4);
            let short: String = source_label.chars().take(keep).collect();
            format!(" repo: {}… ▾ ", short)
        } else {
            raw
        };
        let chip_width = clipped.chars().count() as u16;
        if chip_width < title_area.width {
            let chip_x = title_area.right().saturating_sub(chip_width + 1);
            select.source_chip_area = Rect {
                x: chip_x,
                y: top_y,
                width: chip_width,
                height: 1,
            };
            frame.render_widget(
                Paragraph::new(Line::from(vec![Span::styled(
                    clipped,
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Indexed(240))
                        .bold(),
                )])),
                select.source_chip_area,
            );
        }
    }

    let step_prefix_art = " ╰─ ";
    let step_prefix_label = "Flow ";
    let mut step_spans = vec![
        Span::styled(
            step_prefix_art,
            Style::default().fg(Color::Indexed(45)).bold(),
        ),
        Span::styled(
            step_prefix_label,
            Style::default().fg(Color::Indexed(240)).italic(),
        ),
    ];
    let mut step_x = step_area.x.saturating_add(
        (step_prefix_art.chars().count() + step_prefix_label.chars().count()) as u16,
    );
    let step_y = step_area.y;
    if let Some(ref step) = select.step
        && let Some((cur_s, tot_s)) = step.split_once('/')
        && let (Ok(cur), Ok(tot)) = (cur_s.parse::<usize>(), tot_s.parse::<usize>())
    {
        for i in 1..=tot {
            if i > 1 {
                let dash_style = if i <= cur {
                    Style::default().fg(Color::Indexed(45))
                } else {
                    Style::default().fg(Color::Indexed(236))
                };
                step_spans.push(Span::styled(" ── ", dash_style));
                step_x = step_x.saturating_add(4);
            }
            let label = select
                .step_labels
                .get(i - 1)
                .cloned()
                .or_else(|| {
                    default_step_labels
                        .get(i - 1)
                        .map(|label| label.to_string())
                })
                .unwrap_or_else(|| i.to_string());
            let (badge, style) = if i < cur {
                (
                    format!("◆ {label}"),
                    Style::default().fg(Color::Indexed(45)).bold(),
                )
            } else if i == cur {
                (
                    format!("▌ {label} ▐"),
                    Style::default().fg(Color::Black).bg(Color::Yellow).bold(),
                )
            } else {
                (
                    format!("◇ {label}"),
                    Style::default().fg(Color::Indexed(240)),
                )
            };
            let badge_width = badge.chars().count() as u16;
            step_spans.push(Span::styled(badge.clone(), style));
            select.step_hit_areas.push(Rect {
                x: step_x,
                y: step_y,
                width: badge_width,
                height: 1,
            });
            step_x = step_x.saturating_add(badge_width);
        }
    }
    frame.render_widget(Paragraph::new(Line::from(step_spans)), step_area);

    // Full-width separator
    let sep = Paragraph::new("─".repeat(sep_area.width as usize))
        .style(Style::default().fg(Color::Indexed(236)));
    frame.render_widget(
        sep,
        Rect {
            x: sep_area.x,
            y: sep_area.y + sep_area.height.saturating_sub(1),
            width: sep_area.width,
            height: 1,
        },
    );

    // Tab bar
    select.tab_hit_areas.clear();
    if select.tabs.len() > 1 {
        let tab_titles = build_tab_titles(select);
        let divider_width = 3u16;
        let horizontal_padding = 1u16;
        let mut x = tab_area.x.saturating_add(horizontal_padding);
        let inner_right = tab_area.right().saturating_sub(horizontal_padding);

        for (i, title) in tab_titles.iter().enumerate() {
            let width = (title.width() as u16).saturating_add(2);
            if x >= inner_right {
                break;
            }

            let clamped_width = width.min(inner_right.saturating_sub(x));
            if clamped_width > 0 {
                select.tab_hit_areas.push(Rect {
                    x,
                    y: tab_area.y,
                    width: clamped_width,
                    height: 1,
                });
            }

            x = x.saturating_add(width);
            if i + 1 < tab_titles.len() {
                x = x.saturating_add(divider_width);
            }
        }

        let tabs = Tabs::new(tab_titles)
            .select(select.active_tab)
            .style(Style::default().fg(Color::DarkGray))
            .highlight_style(Style::default()) // no override — styled per-span above
            .divider(Span::styled(
                " │ ",
                Style::default().fg(Color::Indexed(236)),
            ))
            .padding(" ", " ");

        frame.render_widget(
            tabs.block(Block::default().padding(Padding::horizontal(1))),
            tab_area,
        );
    }

    // Separator below tabs
    let sep2 = Paragraph::new("─".repeat(sep2_area.width as usize))
        .style(Style::default().fg(Color::Indexed(236)));
    frame.render_widget(sep2, sep2_area);
}

fn draw_list(frame: &mut Frame, area: Rect, select: &mut TabbedSelect) {
    select.set_visible_height(area.height as usize);
    let visible = area.height as usize;
    let tab = &select.tabs[select.active_tab];
    let on_installed_tab = tab.name == "Installed";
    let is_final_step = matches!(select.step_position(), Some((cur, tot)) if cur == tot);
    let button_label = " Install (i) ";
    let button_width = button_label.chars().count() as u16;
    select.action_button_area = Rect::default();
    let content_width = area.width.saturating_sub(2) as usize; // padding

    // Build all rows first, then slice for scroll window
    let mut all_rows: Vec<ListItem> = Vec::new();
    let mut row_items: Vec<Option<usize>> = Vec::new();
    let mut flat_idx = 0usize;

    for (gi, group) in tab.groups.iter().enumerate() {
        if !group.label.is_empty() && tab.groups.len() > 1 {
            // Blank line between sections (not before first)
            if gi > 0 {
                all_rows.push(ListItem::new(Line::from("")));
                row_items.push(None);
            }
            let header_style = Style::default().fg(Color::DarkGray).bold();
            all_rows.push(ListItem::new(Line::from(vec![
                Span::styled(format!("  {} ", group.label), header_style),
                Span::styled(
                    "─".repeat(content_width.saturating_sub(group.label.len() + 4)),
                    Style::default().fg(Color::Indexed(236)),
                ),
            ])));
            row_items.push(None);
        }

        for item in &group.items {
            let is_cursor = flat_idx == select.cursor && !select.action_button_focused;

            let check = if select.multi {
                if item.locked && item.selected {
                    Span::styled(" ✓ ", Style::default().fg(Color::Cyan))
                } else if item.selected {
                    Span::styled(" ✓ ", Style::default().fg(Color::Magenta).bold())
                } else if item.outdated {
                    Span::styled(" ● ", Style::default().fg(Color::Yellow))
                } else if item.installed {
                    Span::styled(" ● ", Style::default().fg(Color::Green))
                } else {
                    Span::styled(" ◇ ", Style::default().fg(Color::Indexed(240)))
                }
            } else if item.selected {
                Span::styled(" ● ", Style::default().fg(Color::Cyan))
            } else {
                Span::styled(" ○ ", Style::default().fg(Color::Indexed(240)))
            };

            let cursor_span = if is_cursor {
                Span::styled("▸ ", Style::default().fg(Color::Cyan))
            } else {
                Span::raw("  ")
            };

            let label_style = if is_cursor {
                Style::default().fg(Color::Cyan).bold()
            } else if item.locked {
                Style::default().fg(Color::Indexed(248))
            } else if item.selected {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::Gray)
            };

            let mut spans = vec![cursor_span, check, Span::styled(&item.label, label_style)];

            // Inline suffix + installed for all items (cursor and non-cursor)
            if let Some(ref suffix) = item.suffix {
                spans.push(Span::styled(
                    format!("  {suffix}"),
                    Style::default().fg(Color::Indexed(240)).italic(),
                ));
            }
            if item.outdated {
                spans.push(Span::styled(
                    "  outdated",
                    Style::default().fg(Color::Yellow),
                ));
            } else if item.installed && !on_installed_tab {
                let installed_label = match item.installed_scope.as_deref() {
                    Some(scope) if !scope.is_empty() => format!("  installed · {scope}"),
                    _ => "  installed".into(),
                };
                spans.push(Span::styled(
                    installed_label,
                    Style::default().fg(Color::Green),
                ));
            } else if item.selected && select.multi {
                spans.push(Span::styled(
                    "  selected",
                    Style::default().fg(Color::Magenta),
                ));
            }

            all_rows.push(ListItem::new(Line::from(spans)));
            row_items.push(Some(flat_idx));

            // Description on separate line below cursor item
            if is_cursor {
                let mut detail_parts: Vec<String> = Vec::new();
                if !item.description.is_empty() {
                    detail_parts.push(item.description.clone());
                }
                if !detail_parts.is_empty() {
                    let detail = detail_parts.join("  ·  ");
                    let indent = "       ";
                    let wrap_width = content_width.saturating_sub(indent.len());
                    for line in wrap_text(&detail, wrap_width) {
                        all_rows.push(ListItem::new(Line::from(vec![
                            Span::raw(indent),
                            Span::styled(line, Style::default().fg(Color::DarkGray)),
                        ])));
                        row_items.push(Some(flat_idx));
                    }
                }
            }

            flat_idx += 1;
        }
    }

    let total_rows = all_rows.len();
    select.rendered_total_rows = total_rows;
    let max_scroll = total_rows.saturating_sub(visible);
    let scroll = select.scroll.min(max_scroll);
    select.scroll = scroll;

    let end = (scroll + visible).min(total_rows);
    select.rendered_list_rows = row_items
        .iter()
        .skip(scroll)
        .take(end - scroll)
        .copied()
        .collect();
    let visible_rows: Vec<ListItem> = all_rows
        .into_iter()
        .skip(scroll)
        .take(end - scroll)
        .collect();

    let list = List::new(visible_rows).block(Block::default().padding(Padding::new(1, 1, 0, 0)));
    frame.render_widget(list, area);

    if is_final_step {
        let button_y = area.y.saturating_add(total_rows as u16 + 1);
        if button_y < area.bottom() && area.width > button_width + 2 {
            select.action_button_area = Rect {
                x: area.x + 2,
                y: button_y,
                width: button_width,
                height: 1,
            };
            let button_style = if select.action_button_focused {
                Style::default().fg(Color::Black).bg(Color::Yellow).bold()
            } else {
                Style::default().fg(Color::Black).bg(Color::Green).bold()
            };
            frame.render_widget(
                Paragraph::new(Line::from(vec![Span::styled(button_label, button_style)])),
                select.action_button_area,
            );
        }
    }
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.is_empty() {
            current = word.to_string();
        } else if current.len() + 1 + word.len() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current);
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

fn wrap_text_lines(text: &str, width: usize) -> Vec<String> {
    let mut out = Vec::new();
    for raw_line in text.lines() {
        let wrapped = wrap_text(raw_line, width);
        if wrapped.is_empty() {
            out.push(String::new());
        } else {
            out.extend(wrapped);
        }
    }
    if out.is_empty() {
        out.push(String::new());
    }
    out
}

fn draw_status(frame: &mut Frame, area: Rect, select: &mut TabbedSelect) {
    frame.render_widget(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Indexed(236))),
        area,
    );

    let tab_count = select.selected_count();
    let total = select.total_selected();

    let mut status_text = if let Some(ref msg) = select.flash_message {
        format!("  {msg}")
    } else if select.multi {
        if select.tabs.len() > 1 {
            format!("  {} selected in tab, {} total", tab_count, total)
        } else {
            format!("  {} selected", total)
        }
    } else if total > 0 {
        let flat = select.tabs[select.active_tab]
            .groups
            .iter()
            .flat_map(|g| &g.items)
            .collect::<Vec<_>>();
        if let Some(item) = flat.get(select.cursor) {
            format!("  → {}", item.label)
        } else {
            "  None selected".to_string()
        }
    } else {
        "  None selected".to_string()
    };

    let text_style = if select.flash_message.is_some() {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let text_x = area.x;
    let text_width = area.width;

    if text_width > 0 {
        status_text.truncate(text_width.saturating_sub(1) as usize);
    }

    frame.render_widget(
        Paragraph::new(Line::from(vec![Span::styled(status_text, text_style)])),
        Rect {
            x: text_x,
            y: area.y + 1,
            width: text_width,
            height: 1,
        },
    );
}

fn draw_help(frame: &mut Frame, area: Rect, select: &TabbedSelect) {
    let mut keys: Vec<(&str, &str)> = vec![("↑↓", "navigate")];

    if select.tabs.len() > 1 {
        keys.push(("tab", "switch tab"));
    }
    if !select.source_options.is_empty() {
        keys.push(("r", "repos"));
    }
    keys.push(("←", "back"));

    let cur_tab = &select.tabs[select.active_tab].name;
    let on_installed = cur_tab == "Installed";
    let on_updates = cur_tab.starts_with("Updates");

    if !on_installed && !on_updates {
        keys.push(("enter", "toggle"));
        if select.multi {
            keys.push(("a", "all"));
        }
    }

    if on_updates {
        keys.push(("u", "update all"));
    }

    let has_installed = select.tabs.iter().any(|t| t.name == "Installed");
    if has_installed {
        keys.push(("d", "remove"));
        if on_installed {
            keys.push(("D", "remove all"));
        }
    }

    let is_final_step = matches!(select.step_position(), Some((cur, tot)) if cur == tot);
    if is_final_step {
        keys.push(("i", "install"));
    } else {
        keys.push(("→", "next"));
    }
    keys.push(("esc", "quit"));

    if select.tabs[select.active_tab].name.starts_with("Updates") {
        keys.push(("U", "update all"));
    }

    // Build help spans, splitting into multiple lines if needed
    let avail_width = area.width.saturating_sub(2) as usize; // padding
    let mut lines: Vec<Line> = Vec::new();
    let mut current_spans: Vec<Span> = Vec::new();
    let mut current_width: usize = 0;

    for (idx, (key, desc)) in keys.iter().enumerate() {
        let entry_width = 1 + key.len() + 1 + desc.len();
        let sep_width = if idx < keys.len() - 1 { 2 } else { 0 };

        if !current_spans.is_empty() && current_width + entry_width + sep_width > avail_width {
            lines.push(Line::from(std::mem::take(&mut current_spans)));
            current_width = 0;
        }

        current_spans.push(Span::styled(
            format!(" {key}"),
            Style::default().fg(Color::Cyan),
        ));
        current_spans.push(Span::styled(
            format!(" {desc}"),
            Style::default().fg(Color::DarkGray),
        ));
        current_width += entry_width;

        if idx < keys.len() - 1 {
            current_spans.push(Span::styled("  ", Style::default().fg(Color::Indexed(236))));
            current_width += sep_width;
        }
    }
    if !current_spans.is_empty() {
        lines.push(Line::from(current_spans));
    }

    let help = Paragraph::new(lines).block(Block::default().padding(Padding::horizontal(1)));
    frame.render_widget(help, area);
}

pub fn draw_summary(frame: &mut Frame, data: &super::SummaryData, scroll: usize) {
    let area = frame.area();
    frame.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        area,
    );

    let summary_height = (3 + data.notes.len() as u16).min(area.height.saturating_sub(6));
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),              // header
            Constraint::Length(1),              // separator
            Constraint::Length(summary_height), // summary info
            Constraint::Length(1),              // separator
            Constraint::Min(5),                 // items
            Constraint::Length(2),              // help
        ])
        .split(area);

    // Header
    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            " vstack ",
            Style::default().fg(Color::Black).bg(Color::Green).bold(),
        ),
        Span::styled(
            "  Installation complete",
            Style::default().fg(Color::White).bold(),
        ),
    ]))
    .block(Block::default().padding(Padding::top(1)));
    frame.render_widget(header, chunks[0]);

    // Separator
    let sep = Paragraph::new("─".repeat(area.width as usize))
        .style(Style::default().fg(Color::Indexed(236)));
    frame.render_widget(sep, chunks[1]);

    // Summary info (what · how · where)
    let total = data.agents.len() + data.skills.len() + data.hooks.len();
    let n_updated = data.updated.len();
    let n_new = total - n_updated;

    let mut count_spans: Vec<Span> = Vec::new();
    if n_new > 0 {
        count_spans.push(Span::styled(
            format!("  {n_new} installed"),
            Style::default().fg(Color::Green),
        ));
    }
    if n_updated > 0 {
        if !count_spans.is_empty() {
            count_spans.push(Span::styled(" · ", Style::default().fg(Color::DarkGray)));
        }
        count_spans.push(Span::styled(
            format!(
                "{}{n_updated} updated",
                if count_spans.is_empty() { "  " } else { "" }
            ),
            Style::default().fg(Color::Yellow),
        ));
    }

    let mut summary_lines = vec![
        Line::from(count_spans),
        Line::from(Span::styled(
            format!("  {} · {} scope", data.method, data.scope),
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            format!("  → {}", data.harnesses.join(", ")),
            Style::default().fg(Color::DarkGray),
        )),
    ];
    for note in &data.notes {
        summary_lines.push(Line::from(Span::styled(
            format!("  ! {note}"),
            Style::default().fg(Color::Yellow),
        )));
    }
    frame.render_widget(Paragraph::new(summary_lines), chunks[2]);

    // Separator
    let sep2 = Paragraph::new("─".repeat(area.width as usize))
        .style(Style::default().fg(Color::Indexed(236)));
    frame.render_widget(sep2, chunks[3]);

    // Build item lines — split into updated vs new
    let content_width = area.width.saturating_sub(2) as usize;
    let mut all_lines: Vec<Line> = Vec::new();

    let updated_set: std::collections::HashSet<&str> =
        data.updated.iter().map(|s| s.as_str()).collect();

    // Updated items first
    let updated_agents: Vec<_> = data
        .agents
        .iter()
        .filter(|n| updated_set.contains(n.as_str()))
        .cloned()
        .collect();
    let updated_skills: Vec<_> = data
        .skills
        .iter()
        .filter(|n| updated_set.contains(n.as_str()))
        .cloned()
        .collect();
    let updated_hooks: Vec<_> = data
        .hooks
        .iter()
        .filter(|(n, _)| updated_set.contains(n.as_str()))
        .cloned()
        .collect();

    let new_agents: Vec<_> = data
        .agents
        .iter()
        .filter(|n| !updated_set.contains(n.as_str()))
        .cloned()
        .collect();
    let new_skills: Vec<_> = data
        .skills
        .iter()
        .filter(|n| !updated_set.contains(n.as_str()))
        .cloned()
        .collect();
    let new_hooks: Vec<_> = data
        .hooks
        .iter()
        .filter(|(n, _)| !updated_set.contains(n.as_str()))
        .cloned()
        .collect();

    let has_updates =
        !updated_agents.is_empty() || !updated_skills.is_empty() || !updated_hooks.is_empty();
    let has_new = !new_agents.is_empty() || !new_skills.is_empty() || !new_hooks.is_empty();

    if has_updates {
        all_lines.push(section_header("Updated", content_width));
        let mut all_updated: Vec<String> = Vec::new();
        all_updated.extend(updated_agents);
        all_updated.extend(updated_skills);
        all_updated.extend(updated_hooks.iter().map(|(n, _)| n.clone()));
        name_grid_color(&all_updated, content_width, Color::Yellow, &mut all_lines);
        all_lines.push(Line::from(""));
    }

    if has_new {
        if !new_agents.is_empty() {
            all_lines.push(section_header("Agents", content_width));
            name_grid(&new_agents, content_width, &mut all_lines);
            all_lines.push(Line::from(""));
        }

        if !new_skills.is_empty() {
            all_lines.push(section_header("Skills", content_width));
            name_grid(&new_skills, content_width, &mut all_lines);
            all_lines.push(Line::from(""));
        }

        if !new_hooks.is_empty() {
            all_lines.push(section_header("Hooks", content_width));
            for (name, event) in &new_hooks {
                all_lines.push(Line::from(vec![
                    Span::styled("    ◆ ", Style::default().fg(Color::Green)),
                    Span::styled(name.clone(), Style::default().fg(Color::Cyan)),
                    Span::styled(format!("  {event}"), Style::default().fg(Color::DarkGray)),
                ]));
            }
        }
    } else if !has_updates {
        // Edge case: nothing at all (shouldn't happen but be safe)
        if !data.agents.is_empty() {
            all_lines.push(section_header("Agents", content_width));
            name_grid(&data.agents, content_width, &mut all_lines);
            all_lines.push(Line::from(""));
        }
        if !data.skills.is_empty() {
            all_lines.push(section_header("Skills", content_width));
            name_grid(&data.skills, content_width, &mut all_lines);
            all_lines.push(Line::from(""));
        }
        if !data.hooks.is_empty() {
            all_lines.push(section_header("Hooks", content_width));
            for (name, event) in &data.hooks {
                all_lines.push(Line::from(vec![
                    Span::styled("    ◆ ", Style::default().fg(Color::Green)),
                    Span::styled(name.clone(), Style::default().fg(Color::Cyan)),
                    Span::styled(format!("  {event}"), Style::default().fg(Color::DarkGray)),
                ]));
            }
        }
    }

    // Render scrollable items
    let visible = chunks[4].height as usize;
    let total = all_lines.len();
    let sc = scroll.min(total.saturating_sub(1));
    let end = (sc + visible).min(total);
    let visible_items: Vec<ListItem> = all_lines[sc..end]
        .iter()
        .map(|l| ListItem::new(l.clone()))
        .collect();

    let list = List::new(visible_items).block(Block::default().padding(Padding::new(1, 1, 0, 0)));
    frame.render_widget(list, chunks[4]);

    // Help
    let help_spans = vec![
        Span::styled(" ↑↓", Style::default().fg(Color::Cyan)),
        Span::styled(" scroll", Style::default().fg(Color::DarkGray)),
        Span::styled("  i", Style::default().fg(Color::Cyan)),
        Span::styled(" install more", Style::default().fg(Color::DarkGray)),
        Span::styled("  enter/q", Style::default().fg(Color::Cyan)),
        Span::styled(" exit", Style::default().fg(Color::DarkGray)),
    ];
    let help = Paragraph::new(Line::from(help_spans))
        .block(Block::default().padding(Padding::horizontal(1)));
    frame.render_widget(help, chunks[5]);
}

fn section_header<'a>(title: &str, width: usize) -> Line<'a> {
    let rule_len = width.saturating_sub(title.len() + 3);
    Line::from(vec![
        Span::styled(
            format!("  {title} "),
            Style::default().fg(Color::DarkGray).bold(),
        ),
        Span::styled(
            "─".repeat(rule_len),
            Style::default().fg(Color::Indexed(236)),
        ),
    ])
}

fn name_grid_color(names: &[String], content_width: usize, color: Color, out: &mut Vec<Line<'_>>) {
    let max_len = names.iter().map(|s| s.len()).max().unwrap_or(0);
    let entry_width = max_len + 8;
    let num_cols = (content_width / entry_width).max(1);

    for chunk in names.chunks(num_cols) {
        let mut spans: Vec<Span> = Vec::new();
        for name in chunk {
            spans.push(Span::styled("    ◆ ", Style::default().fg(color)));
            let padded = format!("{:<width$}", name, width = max_len + 2);
            spans.push(Span::styled(padded, Style::default().fg(Color::White)));
        }
        out.push(Line::from(spans));
    }
}

fn name_grid(names: &[String], content_width: usize, out: &mut Vec<Line<'_>>) {
    name_grid_color(names, content_width, Color::Green, out);
}

fn draw_confirm_dialog(frame: &mut Frame, select: &mut TabbedSelect, message: &str) {
    let area = frame.area();

    let dialog_width = 60u16.min(area.width.saturating_sub(4));
    let wrap_width = dialog_width.saturating_sub(6) as usize;
    let wrapped = wrap_text_lines(message, wrap_width);
    let dialog_height = (wrapped.len() as u16 + 6).min(area.height.saturating_sub(4));

    let dialog_area = Rect::new(
        (area.width.saturating_sub(dialog_width)) / 2,
        (area.height.saturating_sub(dialog_height)) / 2,
        dialog_width,
        dialog_height,
    );

    // Clear dialog area completely (wipe underlying content)
    frame.render_widget(Clear, dialog_area);
    frame.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        dialog_area,
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow).bg(Color::Black))
        .title(Span::styled(
            " Confirm ",
            Style::default().fg(Color::Yellow).bold(),
        ))
        .style(Style::default().bg(Color::Black))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(dialog_area);
    frame.render_widget(block, dialog_area);

    // Message
    let text_lines: Vec<Line> = wrapped
        .iter()
        .map(|l| {
            Line::from(Span::styled(
                l.as_str(),
                Style::default().fg(Color::White).bg(Color::Black),
            ))
        })
        .collect();

    let msg_height = inner.height.saturating_sub(2);
    let max_scroll = wrapped.len().saturating_sub(msg_height as usize);
    let scroll = select.confirm_dialog_scroll.min(max_scroll);
    select.confirm_dialog_scroll = scroll;
    let msg_area = Rect::new(inner.x, inner.y, inner.width, msg_height);
    frame.render_widget(
        Paragraph::new(text_lines)
            .scroll((scroll as u16, 0))
            .style(Style::default().bg(Color::Black)),
        msg_area,
    );

    // Help line at bottom
    let help_area = Rect::new(inner.x, inner.y + msg_height, inner.width, 1);
    let help = Line::from(vec![
        Span::styled("↑↓", Style::default().fg(Color::Cyan)),
        Span::styled(" scroll  ", Style::default().fg(Color::DarkGray)),
        Span::styled("enter", Style::default().fg(Color::Cyan)),
        Span::styled(" confirm  ", Style::default().fg(Color::DarkGray)),
        Span::styled("esc", Style::default().fg(Color::Cyan)),
        Span::styled(" cancel", Style::default().fg(Color::DarkGray)),
    ]);
    frame.render_widget(Paragraph::new(help), help_area);
}

fn draw_repo_dialog(frame: &mut Frame, select: &mut TabbedSelect) {
    let Some(dialog) = select.repo_dialog.as_ref() else {
        return;
    };
    let area = frame.area();
    let dialog_width = 56u16.min(area.width.saturating_sub(4));
    let dialog_height = 12u16.min(area.height.saturating_sub(4));
    let dialog_area = Rect::new(
        (area.width.saturating_sub(dialog_width)) / 2,
        (area.height.saturating_sub(dialog_height)) / 2,
        dialog_width,
        dialog_height,
    );

    frame.render_widget(Clear, dialog_area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Indexed(45)))
        .title(Span::styled(
            " Package Source ",
            Style::default().fg(Color::Indexed(45)).bold(),
        ))
        .padding(Padding::new(1, 1, 1, 1));
    let inner = block.inner(dialog_area);
    select.repo_dialog_inner = inner;
    frame.render_widget(block, dialog_area);

    if dialog.input_mode {
        let prompt = vec![
            Line::from(Span::styled(
                "Enter repo or URL",
                Style::default().fg(Color::White).bold(),
            )),
            Line::from(Span::styled(
                "Examples: owner/repo or https://github.com/owner/repo",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("> ", Style::default().fg(Color::Indexed(45)).bold()),
                Span::styled(&dialog.input, Style::default().fg(Color::White)),
            ]),
        ];
        frame.render_widget(Paragraph::new(prompt), inner);
    } else {
        let mut lines: Vec<Line> = dialog
            .options
            .iter()
            .enumerate()
            .map(|(i, option)| {
                let style = if i == dialog.cursor {
                    Style::default().fg(Color::Black).bg(Color::Yellow).bold()
                } else {
                    Style::default().fg(Color::White)
                };
                Line::from(Span::styled(format!(" {} ", option.label), style))
            })
            .collect();
        let add_style = if dialog.cursor == dialog.options.len() {
            Style::default().fg(Color::Black).bg(Color::Yellow).bold()
        } else {
            Style::default().fg(Color::Indexed(45)).bold()
        };
        lines.push(Line::from(Span::styled(" + Add repo by link ", add_style)));
        frame.render_widget(Paragraph::new(lines), inner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::{ItemGroup, SelectItem, Tab};
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    #[test]
    fn draw_tabbed_select_tracks_wrapped_detail_rows_for_mouse_hit_testing() {
        let mut select = TabbedSelect::new(
            "Select items",
            vec![Tab {
                name: "Skills".into(),
                groups: vec![ItemGroup {
                    label: String::new(),
                    items: vec![
                        SelectItem {
                            label: "alpha".into(),
                            description: "A deliberately long description that must wrap across multiple terminal rows for stable mouse hit testing.".into(),
                            selected: false,
                            tag: None,
                            suffix: None,
                            locked: false,
                            installed: false,
                            installed_scope: None,
                            outdated: false,
                        },
                        SelectItem {
                            label: "beta".into(),
                            description: String::new(),
                            selected: false,
                            tag: None,
                            suffix: None,
                            locked: false,
                            installed: false,
                            installed_scope: None,
                            outdated: false,
                        },
                    ],
                }],
            }],
            true,
        );
        select.cursor = 0;

        let backend = TestBackend::new(36, 12);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| draw_tabbed_select(frame, &mut select))
            .unwrap();

        let alpha_rows = select
            .rendered_list_rows
            .iter()
            .filter(|row| **row == Some(0))
            .count();

        assert!(
            alpha_rows >= 2,
            "expected wrapped detail rows for the cursor item"
        );
        assert_eq!(
            select.rendered_list_rows.iter().find_map(|row| *row),
            Some(0)
        );
    }

    #[test]
    fn draw_tabbed_select_tracks_each_tab_hit_area() {
        let mut select = TabbedSelect::new(
            "Select items",
            vec![
                Tab {
                    name: "Agents".into(),
                    groups: vec![ItemGroup {
                        label: String::new(),
                        items: vec![],
                    }],
                },
                Tab {
                    name: "Skills".into(),
                    groups: vec![ItemGroup {
                        label: String::new(),
                        items: vec![],
                    }],
                },
                Tab {
                    name: "Updates (2)".into(),
                    groups: vec![ItemGroup {
                        label: String::new(),
                        items: vec![
                            SelectItem {
                                label: "one".into(),
                                description: String::new(),
                                selected: false,
                                tag: None,
                                suffix: None,
                                locked: false,
                                installed: false,
                                installed_scope: None,
                                outdated: false,
                            },
                            SelectItem {
                                label: "two".into(),
                                description: String::new(),
                                selected: false,
                                tag: None,
                                suffix: None,
                                locked: false,
                                installed: false,
                                installed_scope: None,
                                outdated: false,
                            },
                        ],
                    }],
                },
            ],
            true,
        )
        .with_step("2/3")
        .with_step_labels(&["Packages", "Scope", "Harnesses", "Method"]);

        let backend = TestBackend::new(60, 12);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| draw_tabbed_select(frame, &mut select))
            .unwrap();

        assert_eq!(select.tab_hit_areas.len(), 3);
        assert_eq!(select.step_hit_areas.len(), 3);
        assert!(
            select
                .tab_hit_areas
                .windows(2)
                .all(|pair| pair[0].x < pair[1].x)
        );
        assert!(select.tab_hit_areas.iter().all(|area| area.width > 0));
        assert!(
            select
                .step_hit_areas
                .windows(2)
                .all(|pair| pair[0].x < pair[1].x)
        );
    }

    #[test]
    fn draw_tabbed_select_shows_install_button_on_final_step() {
        let mut select = TabbedSelect::new(
            "Installation method",
            vec![Tab {
                name: "Method".into(),
                groups: vec![ItemGroup {
                    label: String::new(),
                    items: vec![SelectItem {
                        label: "Symlink".into(),
                        description: String::new(),
                        selected: true,
                        tag: None,
                        suffix: None,
                        locked: false,
                        installed: false,
                        installed_scope: None,
                        outdated: false,
                    }],
                }],
            }],
            false,
        )
        .with_step("3/3")
        .with_step_labels(&["Packages", "Scope", "Harnesses", "Method"]);

        let backend = TestBackend::new(60, 12);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| draw_tabbed_select(frame, &mut select))
            .unwrap();

        assert!(select.action_button_area.width > 0);
    }
}
