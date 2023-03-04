use std::time::Duration;

use cozy_chess::*;
use cozy_uci::UciFormatOptions;
use cozy_uci::command::{UciCommand, UciInitPos, UciGoParams};
use cozy_uci::remark::{UciRemark, UciIdInfo, UciOptionInfo, UciInfo, UciScore, UciScoreKind};

fn test_command(cmd_str: &str, expected: UciCommand, options: &mut UciFormatOptions) {
    let cmd = UciCommand::parse_from(cmd_str, options)
        .expect("failed to parse command");
    assert_eq!(cmd, expected, "command was not parsed as expected");
    let cmd_roundtripped = UciCommand::parse_from(&cmd.format(options), options)
        .expect("failed to parse reformatted command");
    assert_eq!(cmd, cmd_roundtripped, "roundtripped command is not identical");

    if let UciCommand::SetOption { name, value } = cmd {
        match name.as_str() {
            "UCI_Chess960" => options.chess960 = value.as_deref() == Some("true"),
            "UCI_ShowWDL" => options.wdl = value.as_deref() == Some("true"),
            _ => {}
        }
    }
}

fn test_remark(rmk_str: &str, expected: UciRemark, options: &mut UciFormatOptions) {
    let rmk = UciRemark::parse_from(rmk_str, options)
        .expect("failed to parse remark");
    assert_eq!(rmk, expected, "remark was not parsed as expected");
    let rmk_roundtripped = UciRemark::parse_from(&rmk.format(options), options)
        .expect("failed to parse reformatted remark");
    assert_eq!(rmk, rmk_roundtripped, "roundtripped remark is not identical");
}

macro_rules! impl_test {
    ($($kind:tt $test:literal => $expected:expr,)*) => {{
        let mut options = UciFormatOptions::default();
        $(impl_test!(@test_fn $kind)($test, $expected, &mut options);)*
    }};

    (@test_fn >) => {
        test_command
    };

    (@test_fn <) => {
        test_remark
    };
}

#[test]
fn uci_example() {
    use UciCommand::*;
    use UciRemark::*;
    impl_test! {
        > "uci" => Uci,
        < "id name Shredder" => Id(UciIdInfo::Name("Shredder".to_owned())),
        < "id author Stefan MK" => Id(UciIdInfo::Author("Stefan MK".to_owned())),
        < "option name Hash type spin default 1 min 1 max 128" => Option {
            name: "Hash".to_owned(),
            info: UciOptionInfo::Spin {
                default: 1,
                min: 1,
                max: 128,
            },
        },
        < "option name NalimovPath type string default <empty>" => Option {
            name: "NalimovPath".to_owned(),
            info: UciOptionInfo::String {
                default: "<empty>".to_owned(),
            },
        },
        < "option name NalimovCache type spin default 1 min 1 max 32" => Option {
            name: "NalimovCache".to_owned(),
            info: UciOptionInfo::Spin {
                default: 1,
                min: 1,
                max: 32,
            },
        },
        < "option name Nullmove type check default true" => Option {
            name: "Nullmove".to_owned(),
            info: UciOptionInfo::Check {
                default: true,
            },
        },
        < "option name Style type combo default Normal var Solid var Normal var Risky" => Option {
            name: "Style".to_owned(),
            info: UciOptionInfo::Combo {
                default: "Normal".to_owned(),
                labels: vec![
                    "Solid".to_owned(),
                    "Normal".to_owned(),
                    "Risky".to_owned(),
                ],
            },
        },
        < "uciok" => UciOk,
        > "setoption name Hash value 32" => SetOption {
            name: "Hash".to_owned(),
            value: Some("32".to_owned()),
        },
        > "setoption name NalimovCache value 1" => SetOption {
            name: "NalimovCache".to_owned(),
            value: Some("1".to_owned()),
        },
        > "setoption name NalimovPath value d:\\tb;c\\tb" => SetOption {
            name: "NalimovPath".to_owned(),
            value: Some("d:\\tb;c\\tb".to_owned()),
        },
        > "isready" => IsReady,
        < "readyok" => ReadyOk,
        > "ucinewgame" => UciNewGame,
        > "setoption name UCI_AnalyseMode value true" => SetOption {
            name: "UCI_AnalyseMode".to_owned(),
            value: Some("true".to_owned()),
        },
        > "position startpos moves e2e4 e7e5" => Position {
            init_pos: UciInitPos::StartPos,
            moves: vec![
                Move {
                    from: Square::E2,
                    to: Square::E4,
                    promotion: None,
                },
                Move {
                    from: Square::E7,
                    to: Square::E5,
                    promotion: None,
                },
            ],
        },
        > "go infinite" => Go(UciGoParams {
            infinite: true,
            ..Default::default()
        }),
        < "info depth 1 seldepth 0" => Info(UciInfo {
            depth: Some(1),
            seldepth: Some(0),
            ..Default::default()
        }),
        < "info score cp 13  depth 1 nodes 13 time 15 pv f1b5" => Info(UciInfo {
            depth: Some(1),
            time: Some(Duration::from_millis(15)),
            nodes: Some(13),
            pv: Some(vec![
                Move {
                    from: Square::F1,
                    to: Square::B5,
                    promotion: None,
                },
            ]),
            score: Some(UciScore {
                cp: Some(13),
                mate: None,
                wdl: None,
                kind: UciScoreKind::Exact,
            }),
            ..Default::default()
        }),
        < "info depth 2 seldepth 2" => Info(UciInfo {
            depth: Some(2),
            seldepth: Some(2),
            ..Default::default()
        }),
        < "info nps 15937" => Info(UciInfo {
            nps: Some(15937),
            ..Default::default()
        }),
        < "info score cp 14  depth 2 nodes 255 time 15 pv f1c4 f8c5" => Info(UciInfo {
            depth: Some(2),
            time: Some(Duration::from_millis(15)),
            nodes: Some(255),
            pv: Some(vec![
                Move {
                    from: Square::F1,
                    to: Square::C4,
                    promotion: None,
                },
                Move {
                    from: Square::F8,
                    to: Square::C5,
                    promotion: None,
                },
            ]),
            score: Some(UciScore {
                cp: Some(14),
                mate: None,
                wdl: None,
                kind: UciScoreKind::Exact,
            }),
            ..Default::default()
        }),
        < "info depth 2 seldepth 7 nodes 255" => Info(UciInfo {
            depth: Some(2),
            seldepth: Some(7),
            nodes: Some(255),
            ..Default::default()
        }),
        < "info depth 3 seldepth 7" => Info(UciInfo {
            depth: Some(3),
            seldepth: Some(7),
            ..Default::default()
        }),
        < "info nps 26437" => Info(UciInfo {
            nps: Some(26437),
            ..Default::default()
        }),
        < "info score cp 20  depth 3 nodes 423 time 15 pv f1c4 g8f6 b1c3" => Info(UciInfo {
            depth: Some(3),
            time: Some(Duration::from_millis(15)),
            nodes: Some(423),
            pv: Some(vec![
                Move {
                    from: Square::F1,
                    to: Square::C4,
                    promotion: None,
                },
                Move {
                    from: Square::G8,
                    to: Square::F6,
                    promotion: None,
                },
                Move {
                    from: Square::B1,
                    to: Square::C3,
                    promotion: None,
                },
            ]),
            score: Some(UciScore {
                cp: Some(20),
                mate: None,
                wdl: None,
                kind: UciScoreKind::Exact,
            }),
            ..Default::default()
        }),
        < "info nps 41562" => Info(UciInfo {
            nps: Some(41562),
            ..Default::default()
        }),
        > "stop" => Stop,
        < "bestmove g1f3 ponder d8f6" => BestMove {
            mv: Move {
                from: Square::G1,
                to: Square::F3,
                promotion: None,
            },
            ponder: Some(Move {
                from: Square::D8,
                to: Square::F6,
                promotion: None,
            }),
        },        
    }
}
