use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub month: u8,
    pub point: i32,
    pub yaku: [Option<u8>; 2],
}

pub fn create_card_info() -> HashMap<u8, Card> {
    let mut card_info = HashMap::new();

    //Card info https://en.wikipedia.org/wiki/Hanafuda
    //Updated scoring https://en.wikipedia.org/wiki/Sakura_(card_game)


    //Jan (Pine)
    card_info.insert(1, Card { month: 1, point: 20, yaku: [Some(2), None] });
    card_info.insert(2, Card { month: 1, point: 10, yaku: [Some(3), None] });
    card_info.insert(3, Card { month: 1, point: 0, yaku: [None, None] });

    //Feb (Plum)
    card_info.insert(4, Card { month: 2, point: 10, yaku: [Some(3), None] });
    card_info.insert(5, Card { month: 2, point: 5, yaku: [Some(2), None] });
    card_info.insert(6, Card { month: 2, point: 0, yaku: [None, None] });

    //Mar (Sakura)
    card_info.insert(7, Card { month: 3, point: 20, yaku: [Some(1), Some(2)] });
    card_info.insert(8, Card { month: 3, point: 10, yaku: [Some(3), None] });
    card_info.insert(9, Card { month: 3, point: 0, yaku: [None, None] });

    //Apr (Wisteria)
    card_info.insert(10, Card { month: 4, point: 10, yaku: [Some(6), None] });
    card_info.insert(11, Card { month: 4, point: 5, yaku: [Some(7), None] });
    card_info.insert(12, Card { month: 4, point: 0, yaku: [None, None] });

    //May (Iris)
    card_info.insert(13, Card { month: 5, point: 10, yaku: [Some(6), None] });
    card_info.insert(14, Card { month: 5, point: 5, yaku: [Some(7), None] });
    card_info.insert(15, Card { month: 5, point: 0, yaku: [None, None] });

    //Jun (Iris)
    card_info.insert(16, Card { month: 6, point: 10, yaku: [Some(4), None] });
    card_info.insert(17, Card { month: 6, point: 5, yaku: [Some(5), None] });
    card_info.insert(18, Card { month: 6, point: 0, yaku: [None, None] });

    //Jul (Bush clover)
    card_info.insert(19, Card { month: 7, point: 10, yaku: [Some(6), None] });
    card_info.insert(20, Card { month: 7, point: 5, yaku: [Some(7), Some(8)] });
    card_info.insert(21, Card { month: 7, point: 0, yaku: [None, None] });

    //Aug (Susuki grass)
    card_info.insert(22, Card { month: 8, point: 20, yaku: [Some(1), None] });
    card_info.insert(23, Card { month: 8, point: 5, yaku: [Some(8), None] });
    card_info.insert(24, Card { month: 8, point: 0, yaku: [None, None] });

    //Sep (Chrysanthemum)
    card_info.insert(25, Card { month: 9, point: 10, yaku: [Some(4), None] });
    card_info.insert(26, Card { month: 9, point: 5, yaku: [Some(1), Some(5)] });
    card_info.insert(27, Card { month: 9, point: 0, yaku: [None, None] });

    //Oct (Maple)
    card_info.insert(28, Card { month: 10, point: 10, yaku: [Some(4), None] });
    card_info.insert(29, Card { month: 10, point: 5, yaku: [Some(8), Some(5)] });
    card_info.insert(30, Card { month: 10, point: 0, yaku: [None, None] });


    //Nov (Willow)
    card_info.insert(31, Card { month: 11, point: 10, yaku: [None, None] });
    card_info.insert(32, Card { month: 11, point: 0, yaku: [None, None] });
    card_info.insert(33, Card { month: 11, point: 5, yaku: [None, None] });

    //Dec (Paulownia)
    card_info.insert(34, Card { month: 12, point: 20, yaku: [None, None] });
    card_info.insert(35, Card { month: 12, point: 10, yaku: [None, None] });
    card_info.insert(36, Card { month: 12, point: 0, yaku: [None, None] });

    card_info
}


/*
********* ********* ********* ********* ********* ********* ********* *********
*to   10* *to   10* *to   10* *to   10* *to   10* *to   10* *to   10* *to   10*
*       * *       * *       * *       * *       * *       * *       * *       *
*       * *       * *       * *       * *       * *       * *       * *       *
* may   * * may   * * may   * * may   * * may   * * may   * * may   * * may   *
********* ********* ********* ********* ********* ********* ********* *********
-------------------------------------------------------------------------------
********* ********* ********* ********* ********* ********* ********* *********
*to   10* *to   10* *to   10* *to   10* *to   10* *to   10* *to   10* *to   10*
*       * *       * *       * *       * *       * *       * *       * *       *
*       * *       * *       * *       * *       * *       * *       * *       *
* may   * * may   * * may   * * may   * * may   * * may   * * may   * * may   *
********* ********* ********* ********* ********* ********* ********* *********
CPU

********* ********* ********* ********* ********* ********* ********* *********
*to   10* *to   10* *to   10* *to   10* *to   10* *to   10* *to   10* *to   10*
*       * *       * *       * *       * *       * *       * *       * *       *
*       * *       * *       * *       * *       * *       * *       * *       *
* may   * * may   * * may   * * may   * * may   * * may   * * may   * * may   *
********* ********* ********* ********* ********* ********* ********* *********
********* ********* ********* ********* ********* ********* ********* *********
*to   10* *to   10* *to   10* *to   10* *to   10* *to   10* *to   10* *to   10*
*       * *       * *       * *       * *       * *       * *       * *       *
*       * *       * *       * *       * *       * *       * *       * *       *
* may   * * may   * * may   * * may   * * may   * * may   * * may   * * may   *
********* ********* ********* ********* ********* ********* ********* *********

Player
********* ********* ********* ********* ********* ********* ********* *********
*may  10* *to   10* *to   10* *to   10* *to   10* *to   10* *to   10* *to   10*
*       * *       * *       * *       * *       * *       * *       * *       *
*       * *       * *       * *       * *       * *       * *       * *       *
* 1   2 * * may   * * may   * * may   * * may   * * may   * * may   * * may   *
********* ********* ********* ********* ********* ********* ********* *********
-------------------------------------------------------------------------------
********* ********* ********* ********* ********* ********* ********* *********
*to   10* *to   10* *to   10* *to   10* *to   10* *to   10* *to   10* *to   10*
*       * *       * *       * *       * *       * *       * *       * *       *
*       * *       * *       * *       * *       * *       * *       * *       *
* may   * * may   * * may   * * may   * * may   * * may   * * may   * * may   *
********* ********* ********* ********* ********* ********* ********* *********
*/