use crate::PieceData;
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::ToString;
/// 标准四连方块和标准 SRS 踢墙表
/// https://tetris.wiki/Super_Rotation_System
pub fn new_default_piece_data() -> Vec<PieceData> {
    let srs_jlstz = vec![
        vec![
            vec![(0, 0)],                                      // 0 -> 0
            vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)], // 0 -> R
            vec![(0, 0)],                                      // 0 -> 2
            vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],    // 0 -> L
        ],
        vec![
            vec![(0, 0)],                                      // R -> R
            vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],     // R -> 2
            vec![(0, 0)],                                      // R -> L
            vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],     // R -> 0
        ],
        vec![
            vec![(0, 0)],                                      // 2 -> 2
            vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],    // 2 -> L
            vec![(0, 0)],                                      // 2 -> 0
            vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)], // 2 -> R
        ],
        vec![
            vec![(0, 0)],                                      // L -> L
            vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],  // L -> 0
            vec![(0, 0)],                                      // L -> R
            vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],  // L -> 2
        ],
    ];
    let srs_i = vec![
        vec![
            vec![(0, 0)],                                      // 0 -> 0
            vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],   // 0 -> R
            vec![(0, 0)],                                      // 0 -> 2
            vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],   // 0 -> L
        ],
        vec![
            vec![(0, 0)],                                      // R -> R
            vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],   // R -> 2
            vec![(0, 0)],                                      // R -> L
            vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],   // R -> 0
        ],
        vec![
            vec![(0, 0)],                                      // 2 -> 2
            vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],   // 2 -> L
            vec![(0, 0)],                                      // 2 -> 0
            vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],   // 2 -> R
        ],
        vec![
            vec![(0, 0)],                                      // L -> L
            vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],   // L -> 0
            vec![(0, 0)],                                      // L -> R
            vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],   // L -> 2
        ],
    ];
    let srs_o = vec![
        vec![
            vec![(0, 0)],
            vec![(0, 0)],
            vec![(0, 0)],
            vec![(0, 0)],
        ],
        vec![
            vec![(0, 0)],
            vec![(0, 0)],
            vec![(0, 0)],
            vec![(0, 0)],
        ],
        vec![
            vec![(0, 0)],
            vec![(0, 0)],
            vec![(0, 0)],
            vec![(0, 0)],
        ],
        vec![
            vec![(0, 0)],
            vec![(0, 0)],
            vec![(0, 0)],
            vec![(0, 0)],
        ],
    ];
    vec![
        PieceData {
            name: "I".to_string(),
            initial_width: 4,
            orientation: vec![
                vec![(0, 1), (1, 1), (2, 1), (3, 1)], // 0
                vec![(2, 0), (2, 1), (2, 2), (2, 3)], // R
                vec![(0, 2), (1, 2), (2, 2), (3, 2)], // 2
                vec![(1, 0), (1, 1), (1, 2), (1, 3)], // L
            ],
            test_table: srs_i.clone(),
        },
        PieceData {
            name: "J".to_string(),
            initial_width: 3,
            orientation: vec![
                vec![(0, 1), (1, 1), (2, 1), (0, 0)],
                vec![(1, 0), (1, 1), (1, 2), (2, 0)],
                vec![(0, 1), (1, 1), (2, 1), (2, 2)],
                vec![(1, 0), (1, 1), (1, 2), (0, 2)],
            ],
            test_table: srs_jlstz.clone(),
        },
        PieceData {
            name: "L".to_string(),
            initial_width: 3,
            orientation: vec![
                vec![(0, 1), (1, 1), (2, 1), (2, 0)],
                vec![(1, 0), (1, 1), (1, 2), (2, 2)],
                vec![(0, 1), (1, 1), (2, 1), (0, 2)],
                vec![(1, 0), (1, 1), (1, 2), (0, 0)],
            ],
            test_table: srs_jlstz.clone(),
        },
        PieceData {
            name: "O".to_string(),
            initial_width: 2,
            orientation: vec![
                vec![(0, 0), (0, 1), (1, 1), (1, 0)],
                vec![(0, 0), (0, 1), (1, 1), (1, 0)],
                vec![(0, 0), (0, 1), (1, 1), (1, 0)],
                vec![(0, 0), (0, 1), (1, 1), (1, 0)],
            ],
            test_table: srs_o.clone(),
        },
        PieceData {
            name: "S".to_string(),
            initial_width: 3,
            orientation: vec![
                vec![(0, 1), (1, 1), (1, 0), (2, 0)],
                vec![(1, 0), (1, 1), (2, 1), (2, 2)],
                vec![(0, 2), (1, 2), (1, 1), (2, 1)],
                vec![(0, 0), (0, 1), (1, 1), (1, 2)],
            ],
            test_table: srs_jlstz.clone(),
        },
        PieceData {
            name: "T".to_string(),
            initial_width: 3,
            orientation: vec![
                vec![(1, 1), (0, 1), (1, 0), (2, 1)],
                vec![(1, 1), (1, 2), (1, 0), (2, 1)],
                vec![(1, 1), (0, 1), (1, 2), (2, 1)],
                vec![(1, 1), (0, 1), (1, 0), (1, 2)],
            ],
            test_table: srs_jlstz.clone(),
        },
        PieceData {
            name: "Z".to_string(),
            initial_width: 3,
            orientation: vec![
                vec![(0, 0), (1, 0), (1, 1), (2, 1)],
                vec![(2, 0), (2, 1), (1, 1), (1, 2)],
                vec![(0, 1), (1, 1), (1, 2), (2, 2)],
                vec![(1, 0), (1, 1), (0, 1), (0, 2)],
            ],
            test_table: srs_jlstz.clone(),
        },
    ]
}