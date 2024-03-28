use dialoguer::{theme::ColorfulTheme, Editor, Input, MultiSelect, Select};

use crate::{
    ai::AIConfiguration, configuration::Instructions, providers::TicketProviderConfiguration,
    Configuration,
};

pub fn get_task_ids(configuration: &Configuration) -> Vec<String> {
    let binding = ColorfulTheme::default();

    if configuration.ticket.is_none() {
        return Vec::new();
    }

    let mut task_id = Input::with_theme(&binding)
        .with_prompt("📝 What is the ticket ids? To use more then one use comman (,):");

    match &configuration.ticket.as_ref().unwrap() {
        crate::providers::TicketProviderConfiguration::JIRA(jira) => {
            if let Some(prefix) = &jira.prefix {
                task_id = task_id.with_initial_text(prefix);
            };
        }
    }

    let task_id_input: String = task_id.interact_text().unwrap();

    task_id_input
        .split(',')
        .into_iter()
        .map(|v| v.to_string())
        .collect()
}

pub fn get_source_and_target_branches(default_source: String) -> (String, String) {
    let binding = ColorfulTheme::default();

    let source_branch = Input::with_theme(&binding)
        .with_prompt("⤴️ What is the source branch:")
        .allow_empty(false)
        .with_initial_text(default_source)
        .interact_text()
        .unwrap();

    let target_branch = Input::with_theme(&binding)
        .with_prompt("⤴️ What is the target branch:")
        .allow_empty(false)
        .with_initial_text("main")
        .interact_text()
        .unwrap();

    (source_branch, target_branch)
}

pub fn get_message_change(suggested_title: &String, suggested_body: &String) -> (String, String) {
    let binding = ColorfulTheme::default();

    let mut pull_request_title =
        Input::with_theme(&binding).with_prompt("📓 Pull request title will be:");

    if !suggested_title.is_empty() {
        pull_request_title = pull_request_title.with_initial_text(suggested_title);
    }

    let title = pull_request_title.interact_text().unwrap();

    let pull_request_body = Editor::new();

    let mut body = String::new();

    if let Some(input) = pull_request_body.edit(suggested_body.as_str()).unwrap() {
        body = input;
    }

    (title, body)
}

pub fn initiate() {
    let binding = ColorfulTheme::default();

    let ai_index = Select::with_theme(&binding)
        .with_prompt("🧠 Choose your AI:")
        .items(&crate::ai::AVAILABLE_AI[..])
        .default(0)
        .max_length(1)
        .interact()
        .unwrap();

    let ai: AIConfiguration;

    match crate::ai::AVAILABLE_AI.get(ai_index) {
        Some(&"OpenAI") => {
            let open_ai = crate::ai::open_ai::OpenAIConfiguration {
                model: Input::with_theme(&binding)
                    .with_prompt("What is your OpenAI model:")
                    .allow_empty(false)
                    .with_initial_text("gpt-3.5-turbo")
                    .interact_text()
                    .unwrap(),
            };

            ai = crate::ai::AIConfiguration::OpenAI(open_ai);
        }
        _ => panic!("AI not found!"),
    }

    let provider_index = Select::with_theme(&binding)
        .with_prompt("🗃️ Choose your project mangement tool:")
        .items(&crate::providers::AVAILABLE_TICKET_PROVIDERS[..])
        .default(0)
        .interact()
        .unwrap();

    let ticket: Option<TicketProviderConfiguration>;

    match crate::providers::AVAILABLE_TICKET_PROVIDERS.get(provider_index) {
        Some(&"JIRA") => {
            let jira = crate::providers::jira::JiraConfiguration {
                host: Input::with_theme(&binding)
                    .with_prompt("What is the JIRA host:")
                    .allow_empty(false)
                    .interact_text()
                    .unwrap(),
                prefix: Some(
                    Input::with_theme(&binding)
                        .with_prompt("What is the prefix of your board at JIRA prefix:")
                        .allow_empty(true)
                        .interact_text()
                        .unwrap(),
                ),
            };

            ticket = Some(crate::providers::TicketProviderConfiguration::JIRA(jira));
        }
        Some(&"None") => ticket = None,
        _ => panic!("Ticket provider not found!"),
    }

    let features = MultiSelect::with_theme(&binding)
        .with_prompt("✨ Which features would you like to use?")
        .items(&["Generate commit messages", "Create pull requests"])
        .defaults(&[true, true])
        .interact();

    let mut commit_instructions = vec![];

    if features.as_ref().unwrap().contains(&0) {
        loop {
            let last_instruction: String = Input::with_theme(&binding)
                .with_prompt("📐 What are the instructions for the AI generate the commit message? Type one and press `Enter` to finish keep empty:")
                .allow_empty(true)
                .interact_text()
                .unwrap();

            if last_instruction.is_empty() {
                break;
            }

            commit_instructions.push(last_instruction);
        }
    }

    let mut vcs = None;
    let mut pull_request_instructions = None;

    if features.as_ref().unwrap().contains(&1) {
        let vcs_index = Select::with_theme(&binding)
            .with_prompt("🔀 Choose your version control system:")
            .items(&crate::vcs::AVAILABLE_VCS[..])
            .default(0)
            .interact()
            .unwrap();

        vcs = crate::vcs::VCSConfiguration::from_index(vcs_index);
        let mut instructions = vec![];

        loop {
            let last_instruction: String = Input::with_theme(&binding)
                .with_prompt("📐 What are the instructions for the AI generate the Pull Request? Type one and press `Enter` to finish keep empty:")
                .allow_empty(true)
                .interact_text()
                .unwrap();

            if last_instruction.is_empty() {
                break;
            }

            instructions.push(last_instruction);
        }

        pull_request_instructions = Some(instructions);
    }

    let instructions = Instructions {
        commit: commit_instructions,
        pr: pull_request_instructions,
    };

    let configuration = Configuration {
        ai,
        vcs,
        ticket,
        instructions,
    };

    configuration.save()
}
