use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::{ai::AIConfiguration, providers::TicketProviderConfiguration, Configuration};

pub fn generate(configuration: &Configuration) -> String {
    let binding = ColorfulTheme::default();

    let mut prompt = Input::with_theme(&binding).with_prompt("📝 What is the ticket id:");

    match &configuration.ticket {
        crate::providers::TicketProviderConfiguration::JIRA(jira) => {
            if let Some(prefix) = &jira.prefix {
                prompt = prompt.with_initial_text(prefix);
            };
        }
    }

    prompt.interact_text().unwrap()
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
        _ => panic!("AI [{}] not found!\nCheck the list on ai::AVAILABLE_AI", ai_index),
    }

    let provider_index = Select::with_theme(&binding)
        .with_prompt("🗃️ Choose your project mangement tool:")
        .items(&crate::providers::AVAILABLE_TICKET_PROVIDERS[..])
        .default(0)
        .interact()
        .unwrap();

    let ticket: TicketProviderConfiguration;

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

            ticket = crate::providers::TicketProviderConfiguration::JIRA(jira);
        }
        _ => panic!("Ticket provider not found!"),
    }

    let mut instructions = vec![];

    loop {
        let last_instruction: String = Input::with_theme(&binding)
            .with_prompt("📐 What are the instructions for the AI (to finish keep empty):")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        if last_instruction.is_empty() {
            break;
        }

        instructions.push(last_instruction);
    }

    let configuration = Configuration {
        ai,
        ticket,
        instructions,
    };

    configuration.save()
}
