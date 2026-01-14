use crate::domain::OptimizationResult;
use crate::ui::AppState;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn render_list(f: &mut Frame, state: &AppState, area: Rect) {
    let items: Vec<ListItem> = state
        .results
        .iter()
        .enumerate()
        .map(|(i, result)| {
            let style = if i == state.selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            // Format with fixed-width columns for alignment
            let line = Line::from(vec![
                Span::raw(format!("{:<20} ", result.asset.asset_id)),
                Span::styled(
                    format!("{:<18}", result.asset.alternative_id),
                    Style::default().fg(Color::Cyan),
                ),
            ]);

            ListItem::new(line).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(
                " Asset Alternatives ({}) | Total Time: {:.2}ms ",
                state.results.len(),
                state.total_time.as_secs_f64() * 1000.0
            )),
    );

    f.render_widget(list, area);
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

fn render_summary_view(result: &OptimizationResult) -> Vec<Line<'_>> {
    vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Asset ID: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(result.asset.asset_id.clone()),
        ]),
        Line::from(vec![
            Span::styled("Alternative: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(result.asset.alternative_id.clone(), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Priority Score: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:.4}", result.priority_score),
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Cost Effectiveness: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:.2}/100", result.cost_effectiveness),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Investment Cost: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("${:.2}", result.asset.cost_usd),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled("Risk Reduction: ", Style::default().add_modifier(Modifier::BOLD)),
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
            Span::styled("Payback Period: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:.1} months", result.payback_period),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Safety Level: ", Style::default().add_modifier(Modifier::BOLD)),
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
            Span::styled("Criticality Score: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("{:.2}", result.criticality_score)),
        ]),
    ]
}

fn render_expanded_view<'a>(result: &'a OptimizationResult, state: &'a AppState) -> Vec<Line<'a>> {
    let avg_time = state.total_time.as_secs_f64() * 1000.0 / state.results.len() as f64;

    vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "═══ ASSET INFORMATION ═══",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::raw("Asset ID:        "),
            Span::styled(result.asset.asset_id.clone(), Style::default().fg(Color::White)),
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
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
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
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
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
    ]
}
