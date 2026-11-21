mod modules;
mod AIcaller;
#[tokio::main]

async fn main() {
    let suid_check_output: String = modules::suid::run_initial_suid();
    let sudo_check_output: String = modules::sudo::run_initial_sudo();
    let groups_check_output: String = modules::groups::run_initial_groups_check();
    let kernel_check_output: String = modules::kernel::run_initial_kernel_check();
    let creds_check_output: String = modules::creds::run_initial_creds_check();
    let general_sys_prompt: &str = "You are a Linux privilege escalation expert. Analyze the given command output and identify exploitable vulnerabilities that could lead to root access. Respond ONLY in this exact format:\nSEVERITY: NONE/LOW/MED/HIGH\nFINDING: one or two sentences describing what you found.\nNEVER deviate from this format. No extra text, no explanations outside the format.";
    let (AI_suid, AI_sudo, AI_groups, AI_kernel, AI_creds) = tokio::join!(
        AIcaller::ask_grok(&general_sys_prompt, &suid_check_output),
        AIcaller::ask_grok(&general_sys_prompt, &sudo_check_output),
        AIcaller::ask_grok(&general_sys_prompt, &groups_check_output),
        AIcaller::ask_grok(&general_sys_prompt, &kernel_check_output),
        AIcaller::ask_grok(&general_sys_prompt, &creds_check_output),
    );

    fn decide_color(label: &str, ai_output: &str) {
        if ai_output.contains("SEVERITY: HIGH") {
            println!("\x1b[31m[{}]\x1b[0m {}", label, ai_output);
        }
        
        else if ai_output.contains("SEVERITY: MED") {
            println!("\x1b[33m[{}]\x1b[0m {}", label, ai_output);
        }
        else if ai_output.contains("SEVERITY: LOW") {
            println!("\x1b[32m[{}]\x1b[0m {}", label, ai_output);
        } else {
            println!("\x1b[0m[{}]\x1b[0m {}", label, ai_output);
        }
    }

    decide_color("SUID", &AI_suid.unwrap());
    decide_color("SUDO", &AI_sudo.unwrap());
    decide_color("GROUPS", &AI_groups.unwrap());
    decide_color("KERNEL", &AI_kernel.unwrap());
    decide_color("CREDS", &AI_creds.unwrap());

}
