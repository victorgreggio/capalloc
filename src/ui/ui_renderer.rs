use crate::domain::RiskCalculationResult;
use crate::ui::AppState;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn render_list(f: &mut Frame, state: &AppState, area: Rect) {
    let items: Vec<ListItem> = state
        .display_order
        .iter()
        .enumerate()
        .map(|(display_idx, &result_idx)| {
            let result = &state.results[result_idx];
            let is_risk = state.is_selected_by_risk(result);
            let is_priority = state.is_selected_by_priority(result);
            let is_combined = state.is_selected_by_combined(result);

            let style = if display_idx == state.selected {
                // Currently selected row - white background
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            // Format with fixed-width columns for alignment
            // Asset(20) Alternative(18) R(2) P(2) C(2)
            let mut spans = vec![
                Span::raw(format!("{:<20} ", result.asset.asset_id)),
                Span::styled(
                    format!("{:<18}", result.asset.alternative_id),
                    Style::default().fg(Color::Cyan),
                ),
            ];

            // Add strategy indicators
            if state.optimization_budget.is_some() {
                // Risk strategy column
                spans.push(Span::styled(
                    if is_risk { " R" } else { "  " },
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ));
                // Priority strategy column
                spans.push(Span::styled(
                    if is_priority { " P" } else { "  " },
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ));
                // Combined strategy column
                spans.push(Span::styled(
                    if is_combined { " C" } else { "  " },
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ));
            }

            let line = Line::from(spans);
            ListItem::new(line).style(style)
        })
        .collect();

    let title = if state.optimization_budget.is_some() {
        format!(
            " Asset Alternatives ({}) | Budget: ${:.0} | R=Risk P=Priority C=Combined ",
            state.results.len(),
            state.optimization_budget.unwrap(),
        )
    } else {
        format!(
            " Asset Alternatives ({}) | Total Time: {:.2}ms ",
            state.results.len(),
            state.total_time.as_secs_f64() * 1000.0
        )
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    // Create a ListState to enable scrolling
    let mut list_state = ListState::default();
    list_state.select(Some(state.selected));

    f.render_stateful_widget(list, area, &mut list_state);
}

pub fn render_details(f: &mut Frame, state: &AppState, area: Rect) {
    if let Some(result) = state.get_selected() {
        let content = if state.expanded {
            render_expanded_view(result, state)
        } else {
            render_summary_view(result)
        };

        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(if state.expanded {
                        " Detailed Analysis (Enter to collapse) "
                    } else {
                        " Summary (Enter to expand) "
                    }),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }
}

fn render_summary_view(result: &RiskCalculationResult) -> Vec<Line<'_>> {
    vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Asset ID: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(result.asset.asset_id.clone()),
        ]),
        Line::from(vec![
            Span::styled(
                "Alternative: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                result.asset.alternative_id.clone(),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Priority Score: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{:.4}", result.priority_score),
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Cost Effectiveness: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{:.2}/100", result.cost_effectiveness),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Investment Cost: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("${:.2}", result.asset.cost_usd),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Risk Reduction: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("${:.2}", result.risk_reduction),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::styled("ROI: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:.4}", result.roi),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Payback Period: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{:.1} months", result.payback_period),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Safety Level: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                result.asset.safety_risk_level.clone(),
                if result.asset.is_critical() {
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                } else if result.asset.is_high_risk() {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::Green)
                },
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Criticality Score: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("{:.2}", result.criticality_score)),
        ]),
    ]
}

fn render_expanded_view<'a>(result: &'a RiskCalculationResult, state: &'a AppState) -> Vec<Line<'a>> {
    let avg_time = state.total_time.as_secs_f64() * 1000.0 / state.results.len() as f64;
    let is_risk = state.is_selected_by_risk(result);
    let is_priority = state.is_selected_by_priority(result);
    let is_combined = state.is_selected_by_combined(result);
    let is_any = is_risk || is_priority || is_combined;

    let mut lines = vec![Line::from("")];

    // Show optimization status if applicable
    if state.optimization_budget.is_some() {
        if is_any {
            lines.push(Line::from(vec![Span::styled(
                "SELECTED BY: ",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )]));

            if is_risk {
                lines.push(Line::from(vec![
                    Span::styled("  R ", Style::default().fg(Color::Red)),
                    Span::styled("Risk Reduction Strategy", Style::default().fg(Color::Red)),
                ]));
            }
            if is_priority {
                lines.push(Line::from(vec![
                    Span::styled("  P ", Style::default().fg(Color::Yellow)),
                    Span::styled(
                        "Priority Score Strategy",
                        Style::default().fg(Color::Yellow),
                    ),
                ]));
            }
            if is_combined {
                lines.push(Line::from(vec![
                    Span::styled("  C ", Style::default().fg(Color::Green)),
                    Span::styled(
                        "Combined Strategy (60% Risk, 40% Priority)",
                        Style::default().fg(Color::Green),
                    ),
                ]));
            }
        } else {
            lines.push(Line::from(vec![Span::styled(
                "○ Not Selected by Any Strategy",
                Style::default().fg(Color::Gray),
            )]));
        }
        lines.push(Line::from(""));
    }

    lines.extend(vec![
        Line::from(vec![Span::styled(
            "═══ ASSET INFORMATION ═══",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::raw("Asset ID:        "),
            Span::styled(
                result.asset.asset_id.clone(),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("Alternative:     "),
            Span::styled(
                result.asset.alternative_id.clone(),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::raw("Safety Risk:     "),
            Span::styled(
                result.asset.safety_risk_level.clone(),
                if result.asset.is_critical() {
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                } else if result.asset.is_high_risk() {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::Green)
                },
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "═══ FINANCIAL ANALYSIS ═══",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::raw("Investment Cost: "),
            Span::styled(
                format!("${:>12.2}", result.asset.cost_usd),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("CoF (Total):     "),
            Span::styled(
                format!("${:>12.2}", result.asset.cof_total_usd),
                Style::default().fg(Color::Red),
            ),
        ]),
        Line::from(vec![
            Span::raw("Payback Period:  "),
            Span::styled(
                format!("{:>12.1} months", result.payback_period),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "═══ RISK METRICS ═══",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::raw("PoF (Post):      "),
            Span::styled(
                format!("{:>12.4}", result.asset.pof_post_action),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("Baseline Risk:   "),
            Span::styled(
                format!("${:>12.2}", result.baseline_risk),
                Style::default().fg(Color::Red),
            ),
        ]),
        Line::from(vec![
            Span::raw("Post-Action Risk:"),
            Span::styled(
                format!("${:>12.2}", result.post_action_risk),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("Risk Reduction:  "),
            Span::styled(
                format!("${:>12.2}", result.risk_reduction),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Criticality:     "),
            Span::styled(
                format!("{:>12.2}", result.criticality_score),
                Style::default().fg(Color::Magenta),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "═══ OPTIMIZATION METRICS ═══",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::raw("ROI (Risk/Cost): "),
            Span::styled(
                format!("{:>12.4}", result.roi),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::raw("Cost Effectiveness:"),
            Span::styled(
                format!("{:>12.2}/100", result.cost_effectiveness),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("Priority Score:  "),
            Span::styled(
                format!("{:>12.4}", result.priority_score),
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Cost/Benefit:    "),
            Span::styled(
                format!("{:>12.4}", result.cost_benefit_ratio()),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "═══ PERFORMANCE ═══",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::raw("Calc Time:       "),
            Span::raw(format!("{:.3}ms", result.calculation_time_ms)),
        ]),
        Line::from(vec![
            Span::raw("Avg Time:        "),
            Span::raw(format!("{:.3}ms", avg_time)),
        ]),
    ]);

    lines
}
