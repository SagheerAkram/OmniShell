import sys

def process(filename):
    with open(filename, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Replace struct Cli option
    content = content.replace('command: Commands,', 'command: Option<Commands>,')
    
    # Replace the match 
    content = content.replace('match cli.command {\n        Commands::Init', 'match cli.command {\n        Some(Commands::Init')
    
    # Wrap all Commands:: with Some(Commands::)
    lines = content.split('\n')
    for i, line in enumerate(lines):
        if '        Commands::' in line and '=> {' in line:
            lines[i] = line.replace('        Commands::', '        Some(Commands::')
            
    # Add None => ... right before the final brace of match cli.command
    # Find the end of run()
    content = '\n'.join(lines)
    
    # Add the None branch before Ok(()) at end of run()
    run_idx = content.find('async fn run(')
    if run_idx != -1:
        ok_idx = content.find('Ok(())', run_idx)
        if ok_idx != -1:
            # We want to insert None before Ok(()) which is just after match ends.
            match_str = "        }\n    }\n    Ok(())"
            rep_str = "        }\n        None => {\n            ui::interactive::run_dashboard().await?;\n        }\n    }\n    Ok(())"
            content = content.replace(match_str, rep_str)

    with open(filename, 'w', encoding='utf-8') as f:
        f.write(content)

process('src/main.rs')
