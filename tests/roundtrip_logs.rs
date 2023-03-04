use cozy_uci::command::UciCommand;
use cozy_uci::remark::UciRemark;
use cozy_uci::UciFormatOptions;

fn roundtrip_logs(log_path: &str) {
    let log = std::fs::read_to_string(log_path).expect("failed to find log file");
    let mut options = UciFormatOptions::default();
    for line in log.lines() {
        match line.split_at(1) {
            (">", cmd_str) => {
                let cmd =
                    UciCommand::parse_from(cmd_str, &options).expect("failed to parse command");
                let cmd_roundtripped = UciCommand::parse_from(&cmd.format(&options), &options)
                    .expect("failed to parse reformatted command");
                assert_eq!(
                    cmd, cmd_roundtripped,
                    "roundtripped command is not identical"
                );

                if let UciCommand::SetOption { name, value } = cmd {
                    match name.as_str() {
                        "UCI_Chess960" => options.chess960 = value.as_deref() == Some("true"),
                        "UCI_ShowWDL" => options.wdl = value.as_deref() == Some("true"),
                        _ => {}
                    }
                }
            }
            ("<", rmk_str) => {
                eprintln!("{}", rmk_str);
                let rmk = UciRemark::parse_from(rmk_str, &options).expect("failed to parse remark");
                let rmk_roundtripped = UciRemark::parse_from(&rmk.format(&options), &options)
                    .expect("failed to parse reformatted remark");
                assert_eq!(
                    rmk, rmk_roundtripped,
                    "roundtripped remark is not identical"
                );
            }
            _ => panic!("incorrectly formatted line in log file"),
        }
    }
}

#[test]
fn roundtrip_berserk_w_game_1() {
    roundtrip_logs("tests/uci_logs/berserk_w_game_1.txt");
}

#[test]
fn roundtrip_berserk_b_game_1() {
    roundtrip_logs("tests/uci_logs/berserk_b_game_1.txt");
}

#[test]
fn roundtrip_sf_w_game_1() {
    roundtrip_logs("tests/uci_logs/sf_w_game_1.txt");
}

#[test]
fn roundtrip_sf_b_game_1() {
    roundtrip_logs("tests/uci_logs/sf_b_game_1.txt");
}

#[test]
fn roundtrip_sf_w_game_2() {
    roundtrip_logs("tests/uci_logs/sf_w_game_2.txt");
}

#[test]
fn roundtrip_sf_b_game_2() {
    roundtrip_logs("tests/uci_logs/sf_b_game_2.txt");
}

#[test]
fn roundtrip_sf_w_game_3() {
    roundtrip_logs("tests/uci_logs/sf_w_game_3.txt");
}

#[test]
fn roundtrip_sf_b_game_3() {
    roundtrip_logs("tests/uci_logs/sf_b_game_3.txt");
}
