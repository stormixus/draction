use draction_app_core::{DractionRuntime, IngestResult, RuleSummary, RunSummary};
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Fill, Size, Subscription, Task, Theme, window};
use std::path::PathBuf;

fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    iced::application(boot, update, view)
        .title(title)
        .theme(theme)
        .subscription(subscription)
        .window(window::Settings {
            size: Size::new(880.0, 620.0),
            min_size: Some(Size::new(640.0, 420.0)),
            level: window::Level::Normal,
            ..window::Settings::default()
        })
        .run()
}

fn boot() -> (NativeApp, Task<Message>) {
    (
        NativeApp::default(),
        Task::perform(
            async {
                DractionRuntime::bootstrap()
                    .await
                    .map_err(|e| e.to_string())
            },
            Message::Bootstrapped,
        ),
    )
}

fn title(state: &NativeApp) -> String {
    if state.drop_hovering {
        "Draction - drop to ingest".to_string()
    } else {
        "Draction Native".to_string()
    }
}

fn theme(_state: &NativeApp) -> Theme {
    Theme::TokyoNight
}

#[derive(Default)]
struct NativeApp {
    runtime: Option<DractionRuntime>,
    active_tab: Tab,
    boot_error: Option<String>,
    status: String,
    drop_hovering: bool,
    busy: bool,
    runs: Vec<RunSummary>,
    rules: Vec<RuleSummary>,
    last_ingested: Vec<IngestResult>,
}

#[derive(Debug, Clone)]
enum Message {
    Bootstrapped(Result<DractionRuntime, String>),
    Refresh,
    RunsLoaded(Result<Vec<RunSummary>, String>),
    RulesLoaded(Result<Vec<RuleSummary>, String>),
    WindowEvent(window::Event),
    IngestFinished(Result<Vec<IngestResult>, String>),
    TabSelected(Tab),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Tab {
    #[default]
    Runs,
    Rules,
}

fn update(state: &mut NativeApp, message: Message) -> Task<Message> {
    match message {
        Message::Bootstrapped(Ok(runtime)) => {
            state.status = format!(
                "Ready - API http://127.0.0.1:{} - {}",
                runtime.api_port,
                runtime.base_dir.display()
            );
            state.runtime = Some(runtime);
            refresh_all(state)
        }
        Message::Bootstrapped(Err(error)) => {
            state.boot_error = Some(error.clone());
            state.status = format!("Boot failed: {error}");
            Task::none()
        }
        Message::Refresh => refresh_all(state),
        Message::RunsLoaded(Ok(runs)) => {
            state.runs = runs;
            Task::none()
        }
        Message::RunsLoaded(Err(error)) => {
            state.status = format!("Failed to load runs: {error}");
            Task::none()
        }
        Message::RulesLoaded(Ok(rules)) => {
            state.rules = rules;
            Task::none()
        }
        Message::RulesLoaded(Err(error)) => {
            state.status = format!("Failed to load rules: {error}");
            Task::none()
        }
        Message::WindowEvent(event) => match event {
            window::Event::FileHovered(_) => {
                state.drop_hovering = true;
                Task::none()
            }
            window::Event::FilesHoveredLeft => {
                state.drop_hovering = false;
                Task::none()
            }
            window::Event::FileDropped(path) => {
                state.drop_hovering = false;
                ingest_paths(state, vec![path])
            }
            _ => Task::none(),
        },
        Message::IngestFinished(Ok(results)) => {
            state.busy = false;
            state.status = format!("Processed {} file(s)", results.len());
            state.last_ingested = results;
            refresh_all(state)
        }
        Message::IngestFinished(Err(error)) => {
            state.busy = false;
            state.status = format!("Ingest failed: {error}");
            Task::none()
        }
        Message::TabSelected(tab) => {
            state.active_tab = tab;
            Task::none()
        }
    }
}

fn refresh_all(state: &NativeApp) -> Task<Message> {
    let Some(runtime) = state.runtime.clone() else {
        return Task::none();
    };

    let runs_runtime = runtime.clone();
    let rules_runtime = runtime;

    Task::batch([
        Task::perform(
            async move { runs_runtime.list_runs(50).map_err(|e| e.to_string()) },
            Message::RunsLoaded,
        ),
        Task::perform(
            async move { rules_runtime.list_rules().map_err(|e| e.to_string()) },
            Message::RulesLoaded,
        ),
    ])
}

fn ingest_paths(state: &mut NativeApp, paths: Vec<PathBuf>) -> Task<Message> {
    let Some(runtime) = state.runtime.clone() else {
        state.status = "Runtime is not ready yet".to_string();
        return Task::none();
    };

    state.busy = true;
    state.status = format!("Processing {} dropped item(s)...", paths.len());

    Task::perform(
        async move { runtime.ingest_paths(paths, None).await.map_err(|e| e.to_string()) },
        Message::IngestFinished,
    )
}

fn subscription(_state: &NativeApp) -> Subscription<Message> {
    window::events().map(|(_id, event)| Message::WindowEvent(event))
}

fn view(state: &NativeApp) -> Element<'_, Message> {
    let header = row![
        column![
            text("Draction Native").size(28),
            text(&state.status).size(13),
        ]
        .spacing(4)
        .width(Fill),
        button("Refresh").on_press(Message::Refresh),
    ]
    .spacing(16);

    let drop_label = if state.busy {
        "Processing..."
    } else if state.drop_hovering {
        "Release to ingest"
    } else {
        "Drop files or folders here"
    };

    let drop_zone = container(
        column![
            text(drop_label).size(24),
            text("Rules are matched locally; workflows run in the Rust core.").size(13),
        ]
        .spacing(8),
    )
    .padding(24)
    .width(Fill);

    let tabs = row![
        tab_button("Runs", Tab::Runs, state.active_tab),
        tab_button("Rules", Tab::Rules, state.active_tab),
    ]
    .spacing(8);

    let body = match state.active_tab {
        Tab::Runs => view_runs(state),
        Tab::Rules => view_rules(state),
    };

    let mut content = column![header, drop_zone, tabs, body]
        .padding(24)
        .spacing(18);

    if let Some(error) = &state.boot_error {
        content = content.push(text(format!("Startup error: {error}")));
    }

    container(content).width(Fill).height(Fill).into()
}

fn tab_button(
    label: &'static str,
    tab: Tab,
    active: Tab,
) -> iced::widget::Button<'static, Message> {
    let button = button(label);
    if tab == active {
        button
    } else {
        button.on_press(Message::TabSelected(tab))
    }
}

fn view_runs(state: &NativeApp) -> Element<'_, Message> {
    if state.runs.is_empty() {
        return container(text("No runs yet. Drop a matching file to create one."))
            .padding(16)
            .width(Fill)
            .into();
    }

    let mut list = column![].spacing(8);
    for run in &state.runs {
        list = list.push(
            container(
                column![
                    row![text(&run.status).size(14), text(&run.workflow_id).size(14),].spacing(12),
                    text(format!("run {} / event {}", run.id, run.event_id)).size(12),
                    text(format!("started {}", run.started_at)).size(12),
                ]
                .spacing(4),
            )
            .padding(12)
            .width(Fill),
        );
    }

    scrollable(list).height(Fill).into()
}

fn view_rules(state: &NativeApp) -> Element<'_, Message> {
    if state.rules.is_empty() {
        return container(text("No rules configured."))
            .padding(16)
            .width(Fill)
            .into();
    }

    let mut list = column![].spacing(8);
    for rule in &state.rules {
        let enabled = if rule.enabled { "enabled" } else { "disabled" };
        list = list.push(
            container(
                column![
                    row![text(&rule.name).size(16), text(enabled).size(13),].spacing(12),
                    text(format!("{} -> {}", rule.id, rule.workflow_id)).size(12),
                ]
                .spacing(4),
            )
            .padding(12)
            .width(Fill),
        );
    }

    scrollable(list).height(Fill).into()
}
