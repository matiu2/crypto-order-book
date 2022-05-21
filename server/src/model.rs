use std::cmp::Ordering;

use crate::api::Level;

impl From<binance::model::Price> for Level {
    fn from(input: binance::model::Price) -> Self {
        Level {
            exchange: "binance".to_string(),
            // TODO: rename amount and quantitiy in binance to reduce confusion
            price: input.amount,
            amount: input.quantity,
        }
    }
}

impl From<bitstamp::model::Price> for Level {
    fn from(input: bitstamp::model::Price) -> Self {
        Level {
            exchange: "bitstamp".to_string(),
            price: input.price,
            amount: input.quantity,
        }
    }
}

/// Takes the two order_books from our two client libraries and make a new order_book, ready to serve
pub fn make_merged_market_depth(
    a: binance::model::Depth,
    b: bitstamp::model::OrderBookData,
) -> crate::api::Summary {
    // Get the top 10 (highest) bids
    let mut bids: Vec<Level> = a
        .bids
        .into_iter()
        .map(|price| price.into())
        .chain(b.bids.into_iter().map(|price| price.into()))
        .collect();
    bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(Ordering::Equal));
    bids.truncate(10);
    // Get the 10 best (lowest) asks
    let mut asks: Vec<Level> = a
        .asks
        .into_iter()
        .map(|price| price.into())
        .chain(b.asks.into_iter().map(|price| price.into()))
        .collect();
    asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal));
    asks.truncate(10);

    // Get the spread
    let spread = bids
        .first()
        .zip(asks.first())
        .map(|(bid, ask)| bid.price - ask.price)
        .unwrap_or(0.0);

    crate::api::Summary { spread, bids, asks }
}

#[cfg(test)]
mod unit_test {
    use crate::api::{Level, Summary};
    use binance::model::{Depth, Price};
    use bitstamp::{model::Price as BitPrice, OrderBookData};
    use chrono::Utc;
    use ordered_float::OrderedFloat;

    #[test]
    fn test_make_merged_market_depth() {
        let binance = Depth {
            timestamp: Utc::now(),
            bids: vec![
                Price {
                    amount: 0.067077,
                    quantity: 26.5401,
                },
                Price {
                    amount: 0.067076,
                    quantity: 29.3521,
                },
                Price {
                    amount: 0.067075,
                    quantity: 2.5403,
                },
                Price {
                    amount: 0.067074,
                    quantity: 0.2542,
                },
                Price {
                    amount: 0.067072,
                    quantity: 14.3656,
                },
                Price {
                    amount: 0.067068,
                    quantity: 2.4905,
                },
                Price {
                    amount: 0.067067,
                    quantity: 2.0192,
                },
                Price {
                    amount: 0.067066,
                    quantity: 2.5949,
                },
                Price {
                    amount: 0.067065,
                    quantity: 1.9687,
                },
                Price {
                    amount: 0.067064,
                    quantity: 6.3,
                },
                Price {
                    amount: 0.067058,
                    quantity: 7.7146,
                },
                Price {
                    amount: 0.067056,
                    quantity: 0.5567,
                },
                Price {
                    amount: 0.067054,
                    quantity: 5.474,
                },
                Price {
                    amount: 0.067049,
                    quantity: 0.0708,
                },
                Price {
                    amount: 0.067039,
                    quantity: 0.2542,
                },
                Price {
                    amount: 0.067035,
                    quantity: 0.0442,
                },
                Price {
                    amount: 0.067029,
                    quantity: 0.0342,
                },
                Price {
                    amount: 0.067028,
                    quantity: 1.4991,
                },
                Price {
                    amount: 0.067017,
                    quantity: 1.5018,
                },
                Price {
                    amount: 0.067016,
                    quantity: 0.0437,
                },
            ],
            asks: vec![
                Price {
                    amount: 0.067078,
                    quantity: 0.3728,
                },
                Price {
                    amount: 0.067088,
                    quantity: 0.075,
                },
                Price {
                    amount: 0.06709,
                    quantity: 1.3444,
                },
                Price {
                    amount: 0.067091,
                    quantity: 3.1781,
                },
                Price {
                    amount: 0.067094,
                    quantity: 0.02,
                },
                Price {
                    amount: 0.067097,
                    quantity: 2.757,
                },
                Price {
                    amount: 0.067098,
                    quantity: 0.174,
                },
                Price {
                    amount: 0.067099,
                    quantity: 4.0187,
                },
                Price {
                    amount: 0.0671,
                    quantity: 5.6645,
                },
                Price {
                    amount: 0.067101,
                    quantity: 0.0971,
                },
                Price {
                    amount: 0.067103,
                    quantity: 1.0112,
                },
                Price {
                    amount: 0.067104,
                    quantity: 0.014,
                },
                Price {
                    amount: 0.067105,
                    quantity: 0.0675,
                },
                Price {
                    amount: 0.067111,
                    quantity: 0.0212,
                },
                Price {
                    amount: 0.067113,
                    quantity: 11.5,
                },
                Price {
                    amount: 0.067117,
                    quantity: 1.4905,
                },
                Price {
                    amount: 0.067118,
                    quantity: 1.5047,
                },
                Price {
                    amount: 0.06712,
                    quantity: 0.0127,
                },
                Price {
                    amount: 0.067121,
                    quantity: 5.474,
                },
                Price {
                    amount: 0.067122,
                    quantity: 6.3,
                },
            ],
        };
        let bitstamp = OrderBookData {
            timestamp: Utc::now(),
            bids: vec![
                BitPrice {
                    price: 0.06712,
                    quantity: 0.00521,
                    order_id: 1492210355929088,
                },
                BitPrice {
                    price: 0.06709444,
                    quantity: 0.495,
                    order_id: 1492210382880768,
                },
                BitPrice {
                    price: 0.06709423,
                    quantity: 2.54078178,
                    order_id: 1492210382667777,
                },
                BitPrice {
                    price: 0.06709148,
                    quantity: 5.08133812,
                    order_id: 1492210382671873,
                },
                BitPrice {
                    price: 0.06708276,
                    quantity: 0.495,
                    order_id: 1492210377195520,
                },
                BitPrice {
                    price: 0.06707185,
                    quantity: 0.02720903,
                    order_id: 1492210381746176,
                },
                BitPrice {
                    price: 0.06707133,
                    quantity: 8.13032046,
                    order_id: 1492210373160960,
                },
                BitPrice {
                    price: 0.06706819,
                    quantity: 13.21253335,
                    order_id: 1492210382663681,
                },
                BitPrice {
                    price: 0.06706638,
                    quantity: 6.3,
                    order_id: 1492210381574148,
                },
                BitPrice {
                    price: 0.06705342,
                    quantity: 0.02647296,
                    order_id: 1492210344398848,
                },
                BitPrice {
                    price: 0.06705188,
                    quantity: 13.21301126,
                    order_id: 1492210353078272,
                },
                BitPrice {
                    price: 0.06703735,
                    quantity: 0.02669537,
                    order_id: 1492210236440576,
                },
                BitPrice {
                    price: 0.06703619,
                    quantity: 13.21675573,
                    order_id: 1492210310717441,
                },
                BitPrice {
                    price: 0.06702107,
                    quantity: 0.02583902,
                    order_id: 1492210229022727,
                },
                BitPrice {
                    price: 0.06701951,
                    quantity: 13.22035161,
                    order_id: 1492210292396033,
                },
                BitPrice {
                    price: 0.06700851,
                    quantity: 12.6,
                    order_id: 1492210330406912,
                },
                BitPrice {
                    price: 0.06700245,
                    quantity: 13.22515283,
                    order_id: 1492210196975616,
                },
                BitPrice {
                    price: 0.06697938,
                    quantity: 10.0,
                    order_id: 1492210328870912,
                },
                BitPrice {
                    price: 0.06694791,
                    quantity: 23.37445437,
                    order_id: 1492210382671872,
                },
                BitPrice {
                    price: 0.06694339,
                    quantity: 23.38409383,
                    order_id: 1492210310459392,
                },
                BitPrice {
                    price: 0.0669425,
                    quantity: 1.46,
                    order_id: 1492210280992770,
                },
                BitPrice {
                    price: 0.06692101,
                    quantity: 4.90031903,
                    order_id: 1492210312802306,
                },
                BitPrice {
                    price: 0.066921,
                    quantity: 0.00303076,
                    order_id: 1492119620866048,
                },
                BitPrice {
                    price: 0.06691512,
                    quantity: 0.00303075,
                    order_id: 1492050675544068,
                },
                BitPrice {
                    price: 0.06690338,
                    quantity: 0.53692134,
                    order_id: 1492210310717440,
                },
                BitPrice {
                    price: 0.06690336,
                    quantity: 0.00303073,
                    order_id: 1492118792060928,
                },
                BitPrice {
                    price: 0.06689748,
                    quantity: 0.00303072,
                    order_id: 1492050673647616,
                },
                BitPrice {
                    price: 0.0668952,
                    quantity: 3.61,
                    order_id: 1492207206891527,
                },
                BitPrice {
                    price: 0.06688963,
                    quantity: 1.0,
                    order_id: 1492055880695808,
                },
                BitPrice {
                    price: 0.06688573,
                    quantity: 1.34230334,
                    order_id: 1492210325913600,
                },
                BitPrice {
                    price: 0.06688572,
                    quantity: 0.0030307,
                    order_id: 1492117479297024,
                },
                BitPrice {
                    price: 0.06687984,
                    quantity: 0.00303069,
                    order_id: 1492050590011392,
                },
                BitPrice {
                    price: 0.06686912,
                    quantity: 50.5,
                    order_id: 1492210232283138,
                },
                BitPrice {
                    price: 0.06686809,
                    quantity: 3.4902608,
                    order_id: 1492210308599810,
                },
                BitPrice {
                    price: 0.06686808,
                    quantity: 0.00303067,
                    order_id: 1492115018178560,
                },
                BitPrice {
                    price: 0.0668622,
                    quantity: 0.00303066,
                    order_id: 1492050588094464,
                },
                BitPrice {
                    price: 0.06685447,
                    quantity: 53.87641985,
                    order_id: 1492210326978561,
                },
                BitPrice {
                    price: 0.06685044,
                    quantity: 0.00303064,
                    order_id: 1492112719982592,
                },
                BitPrice {
                    price: 0.06684456,
                    quantity: 0.00303063,
                    order_id: 1491931743744001,
                },
                BitPrice {
                    price: 0.06683868,
                    quantity: 0.00303062,
                    order_id: 1492120541413376,
                },
                BitPrice {
                    price: 0.0668328,
                    quantity: 0.00303061,
                    order_id: 1491931919257603,
                },
                BitPrice {
                    price: 0.0668308,
                    quantity: 7.23,
                    order_id: 1492207289720833,
                },
                BitPrice {
                    price: 0.06683,
                    quantity: 2.2e-5,
                    order_id: 1491915034038273,
                },
                BitPrice {
                    price: 0.06682692,
                    quantity: 0.0030306,
                    order_id: 1491931741818880,
                },
                BitPrice {
                    price: 0.06682104,
                    quantity: 0.00303059,
                    order_id: 1492120539463680,
                },
                BitPrice {
                    price: 0.06681516,
                    quantity: 0.00303058,
                    order_id: 1491931917398019,
                },
                BitPrice {
                    price: 0.06680928,
                    quantity: 0.00303057,
                    order_id: 1491931739930624,
                },
                BitPrice {
                    price: 0.0668034,
                    quantity: 0.00303056,
                    order_id: 1492120537559041,
                },
                BitPrice {
                    price: 0.06679752,
                    quantity: 0.00303055,
                    order_id: 1491931830706177,
                },
                BitPrice {
                    price: 0.06679164,
                    quantity: 0.00303054,
                    order_id: 1491931738071041,
                },
                BitPrice {
                    price: 0.06678576,
                    quantity: 0.00303053,
                    order_id: 1492119963680768,
                },
                BitPrice {
                    price: 0.06677988,
                    quantity: 0.00303052,
                    order_id: 1491931736170498,
                },
                BitPrice {
                    price: 0.06677487,
                    quantity: 206.38711214,
                    order_id: 1492210322812928,
                },
                BitPrice {
                    price: 0.066774,
                    quantity: 0.00304051,
                    order_id: 1491931563515904,
                },
                BitPrice {
                    price: 0.06676812,
                    quantity: 0.0030405,
                    order_id: 1492119795286016,
                },
                BitPrice {
                    price: 0.06676224,
                    quantity: 0.00304049,
                    order_id: 1491931734134788,
                },
                BitPrice {
                    price: 0.06675636,
                    quantity: 0.00304048,
                    order_id: 1491930337165315,
                },
                BitPrice {
                    price: 0.06675048,
                    quantity: 0.00304047,
                    order_id: 1492119619026944,
                },
                BitPrice {
                    price: 0.0667446,
                    quantity: 0.00304046,
                    order_id: 1491931732205568,
                },
                BitPrice {
                    price: 0.06673872,
                    quantity: 0.00304045,
                    order_id: 1491930249023489,
                },
                BitPrice {
                    price: 0.06673284,
                    quantity: 0.00304044,
                    order_id: 1492119616905220,
                },
                BitPrice {
                    price: 0.06672696,
                    quantity: 0.00304043,
                    order_id: 1491931730272258,
                },
                BitPrice {
                    price: 0.06672108,
                    quantity: 0.00304042,
                    order_id: 1491930335252480,
                },
                BitPrice {
                    price: 0.0667152,
                    quantity: 0.00304041,
                    order_id: 1492119532212224,
                },
                BitPrice {
                    price: 0.0667115,
                    quantity: 25.11,
                    order_id: 1492210267025414,
                },
                BitPrice {
                    price: 0.06670932,
                    quantity: 0.0030404,
                    order_id: 1491931728293888,
                },
                BitPrice {
                    price: 0.06670344,
                    quantity: 0.00304039,
                    order_id: 1491894409580544,
                },
                BitPrice {
                    price: 0.06669919,
                    quantity: 0.00493385,
                    order_id: 1491833383346178,
                },
                BitPrice {
                    price: 0.06669756,
                    quantity: 0.00304038,
                    order_id: 1491699618426891,
                },
                BitPrice {
                    price: 0.06669168,
                    quantity: 0.00304037,
                    order_id: 1491699501473794,
                },
                BitPrice {
                    price: 0.0666858,
                    quantity: 0.00304036,
                    order_id: 1491689583345664,
                },
                BitPrice {
                    price: 0.06667992,
                    quantity: 0.00304035,
                    order_id: 1491699616432131,
                },
                BitPrice {
                    price: 0.06667404,
                    quantity: 0.00304034,
                    order_id: 1491699499556869,
                },
                BitPrice {
                    price: 0.06666816,
                    quantity: 0.00304033,
                    order_id: 1491663742431235,
                },
                BitPrice {
                    price: 0.06666301,
                    quantity: 35.41,
                    order_id: 1492206009044999,
                },
                BitPrice {
                    price: 0.06666228,
                    quantity: 0.00304032,
                    order_id: 1491699614507010,
                },
                BitPrice {
                    price: 0.06665725,
                    quantity: 1.0,
                    order_id: 1492112682393600,
                },
                BitPrice {
                    price: 0.0666564,
                    quantity: 0.00304031,
                    order_id: 1491698746904576,
                },
                BitPrice {
                    price: 0.06665052,
                    quantity: 0.0030403,
                    order_id: 1491663740481536,
                },
                BitPrice {
                    price: 0.06664464,
                    quantity: 0.00304029,
                    order_id: 1491699612622851,
                },
                BitPrice {
                    price: 0.06663876,
                    quantity: 0.00304028,
                    order_id: 1491698744938496,
                },
                BitPrice {
                    price: 0.06663659,
                    quantity: 1.0,
                    order_id: 1491697722548225,
                },
                BitPrice {
                    price: 0.06663288,
                    quantity: 0.00304027,
                    order_id: 1491663738560515,
                },
                BitPrice {
                    price: 0.066627,
                    quantity: 0.00304026,
                    order_id: 1491699609239552,
                },
                BitPrice {
                    price: 0.06662112,
                    quantity: 0.00304025,
                    order_id: 1491697759674370,
                },
                BitPrice {
                    price: 0.06661524,
                    quantity: 0.00304024,
                    order_id: 1491663638667264,
                },
                BitPrice {
                    price: 0.06660936,
                    quantity: 0.00304023,
                    order_id: 1491699607363585,
                },
                BitPrice {
                    price: 0.0666044,
                    quantity: 76.65,
                    order_id: 1492210267054084,
                },
                BitPrice {
                    price: 0.06660348,
                    quantity: 0.00304022,
                    order_id: 1491689666686976,
                },
                BitPrice {
                    price: 0.0665976,
                    quantity: 0.00304021,
                    order_id: 1491663636791297,
                },
                BitPrice {
                    price: 0.06659172,
                    quantity: 0.0030402,
                    order_id: 1491699605377025,
                },
                BitPrice {
                    price: 0.06658584,
                    quantity: 0.00304019,
                    order_id: 1491663736725504,
                },
                BitPrice {
                    price: 0.06658317,
                    quantity: 1.0,
                    order_id: 1491776603643905,
                },
                BitPrice {
                    price: 0.06658,
                    quantity: 0.99390005,
                    order_id: 1491662723809280,
                },
                BitPrice {
                    price: 0.06658,
                    quantity: 0.00609995,
                    order_id: 1491662741102592,
                },
                BitPrice {
                    price: 0.06657996,
                    quantity: 0.00304018,
                    order_id: 1491663634796545,
                },
                BitPrice {
                    price: 0.06657408,
                    quantity: 0.00304017,
                    order_id: 1491699603410944,
                },
                BitPrice {
                    price: 0.06657,
                    quantity: 0.99780004,
                    order_id: 1491662776889345,
                },
                BitPrice {
                    price: 0.06657,
                    quantity: 0.00219996,
                    order_id: 1491663283941381,
                },
                BitPrice {
                    price: 0.0665682,
                    quantity: 0.00304016,
                    order_id: 1491663734939648,
                },
            ],
            asks: vec![
                BitPrice {
                    price: 0.06713393,
                    quantity: 7.1439133,
                    order_id: 1492210364788736,
                },
                BitPrice {
                    price: 0.06714474,
                    quantity: 2.5408718,
                    order_id: 1492210382635009,
                },
                BitPrice {
                    price: 0.06714588,
                    quantity: 5.08145029,
                    order_id: 1492210373115905,
                },
                BitPrice {
                    price: 0.06715689,
                    quantity: 8.89352905,
                    order_id: 1492210382274562,
                },
                BitPrice {
                    price: 0.06715965,
                    quantity: 0.05,
                    order_id: 1492210357067776,
                },
                BitPrice {
                    price: 0.06716962,
                    quantity: 6.3,
                    order_id: 1492210381512704,
                },
                BitPrice {
                    price: 0.06717535,
                    quantity: 15.25482601,
                    order_id: 1492210229501955,
                },
                BitPrice {
                    price: 0.0671784,
                    quantity: 1.46,
                    order_id: 1492210326503425,
                },
                BitPrice {
                    price: 0.06718384,
                    quantity: 0.02430177,
                    order_id: 1492210356256768,
                },
                BitPrice {
                    price: 0.06718693,
                    quantity: 9.78841483,
                    order_id: 1492210240815104,
                },
                BitPrice {
                    price: 0.06719566,
                    quantity: 15.2492958,
                    order_id: 1492210237636609,
                },
                BitPrice {
                    price: 0.06719891,
                    quantity: 15.25067262,
                    order_id: 1492210310406146,
                },
                BitPrice {
                    price: 0.06721067,
                    quantity: 0.4421161,
                    order_id: 1492210360217601,
                },
                BitPrice {
                    price: 0.067212,
                    quantity: 3.61,
                    order_id: 1492210326515712,
                },
                BitPrice {
                    price: 0.0672157,
                    quantity: 15.24604979,
                    order_id: 1492210382057474,
                },
                BitPrice {
                    price: 0.06721913,
                    quantity: 12.6,
                    order_id: 1492210330345474,
                },
                BitPrice {
                    price: 0.06722843,
                    quantity: 15.24363077,
                    order_id: 1492210383020035,
                },
                BitPrice {
                    price: 0.0672354,
                    quantity: 7.23,
                    order_id: 1492210196623360,
                },
                BitPrice {
                    price: 0.06726183,
                    quantity: 27.95947224,
                    order_id: 1492210237329408,
                },
                BitPrice {
                    price: 0.06727176,
                    quantity: 0.00303078,
                    order_id: 1492206186278916,
                },
                BitPrice {
                    price: 0.0672894,
                    quantity: 0.00303081,
                    order_id: 1492206185787398,
                },
                BitPrice {
                    price: 0.06729431,
                    quantity: 50.5,
                    order_id: 1492210232233984,
                },
                BitPrice {
                    price: 0.0672945,
                    quantity: 27.94901812,
                    order_id: 1492210382635008,
                },
                BitPrice {
                    price: 0.06730704,
                    quantity: 0.00303084,
                    order_id: 1492206118662144,
                },
                BitPrice {
                    price: 0.06732467,
                    quantity: 0.53658541,
                    order_id: 1492210373668864,
                },
                BitPrice {
                    price: 0.06732468,
                    quantity: 0.00303087,
                    order_id: 1492206076321792,
                },
                BitPrice {
                    price: 0.06733702,
                    quantity: 1.34150349,
                    order_id: 1492210365878272,
                },
                BitPrice {
                    price: 0.06733703,
                    quantity: 66.0812488,
                    order_id: 1492210237603841,
                },
                BitPrice {
                    price: 0.06734232,
                    quantity: 0.0030209,
                    order_id: 1492206023348227,
                },
                BitPrice {
                    price: 0.06735995,
                    quantity: 3.48784521,
                    order_id: 1492210365784064,
                },
                BitPrice {
                    price: 0.06735996,
                    quantity: 0.00302093,
                    order_id: 1492205341057024,
                },
                BitPrice {
                    price: 0.0673776,
                    quantity: 0.00302096,
                    order_id: 1492204917878784,
                },
                BitPrice {
                    price: 0.06737764,
                    quantity: 0.00303079,
                    order_id: 1492206186070016,
                },
                BitPrice {
                    price: 0.06739524,
                    quantity: 0.00302099,
                    order_id: 1492204913156101,
                },
                BitPrice {
                    price: 0.06739528,
                    quantity: 0.00303082,
                    order_id: 1492206185582597,
                },
                BitPrice {
                    price: 0.06741288,
                    quantity: 0.00302102,
                    order_id: 1492165241192452,
                },
                BitPrice {
                    price: 0.06741292,
                    quantity: 0.00303085,
                    order_id: 1492206100049920,
                },
                BitPrice {
                    price: 0.06741868,
                    quantity: 256.6845462,
                    order_id: 1492210237833216,
                },
                BitPrice {
                    price: 0.06743052,
                    quantity: 0.00302105,
                    order_id: 1492164409020417,
                },
                BitPrice {
                    price: 0.06743056,
                    quantity: 0.00303088,
                    order_id: 1492206023606275,
                },
                BitPrice {
                    price: 0.06744626,
                    quantity: 37.48,
                    order_id: 1492202466779138,
                },
                BitPrice {
                    price: 0.0674465,
                    quantity: 23.71,
                    order_id: 1492210266959881,
                },
                BitPrice {
                    price: 0.06744816,
                    quantity: 0.00302108,
                    order_id: 1492164408778752,
                },
                BitPrice {
                    price: 0.0674482,
                    quantity: 0.00053207,
                    order_id: 1492205341265920,
                },
                BitPrice {
                    price: 0.0674482,
                    quantity: 0.00248884,
                    order_id: 1492206010265604,
                },
                BitPrice {
                    price: 0.0674658,
                    quantity: 0.00159986,
                    order_id: 1492162766815233,
                },
                BitPrice {
                    price: 0.0674658,
                    quantity: 0.00142125,
                    order_id: 1492164381061120,
                },
                BitPrice {
                    price: 0.06746584,
                    quantity: 0.00302094,
                    order_id: 1492205239832576,
                },
                BitPrice {
                    price: 0.06748344,
                    quantity: 0.00302114,
                    order_id: 1492162766172161,
                },
                BitPrice {
                    price: 0.06748348,
                    quantity: 0.00302097,
                    order_id: 1492204917673984,
                },
                BitPrice {
                    price: 0.06749532,
                    quantity: 0.00303065,
                    order_id: 1491862059839488,
                },
                BitPrice {
                    price: 0.06750108,
                    quantity: 0.00302117,
                    order_id: 1491861941764098,
                },
                BitPrice {
                    price: 0.06750112,
                    quantity: 0.003021,
                    order_id: 1491861972586500,
                },
                BitPrice {
                    price: 0.06751296,
                    quantity: 0.00303068,
                    order_id: 1491862049656833,
                },
                BitPrice {
                    price: 0.06751872,
                    quantity: 0.0030212,
                    order_id: 1491851847143426,
                },
                BitPrice {
                    price: 0.06751876,
                    quantity: 0.00302103,
                    order_id: 1491861971234816,
                },
                BitPrice {
                    price: 0.06752311,
                    quantity: 1.0,
                    order_id: 1491859023663106,
                },
                BitPrice {
                    price: 0.0675306,
                    quantity: 0.00303071,
                    order_id: 1491862037516288,
                },
                BitPrice {
                    price: 0.06753636,
                    quantity: 0.00302123,
                    order_id: 1491851685060611,
                },
                BitPrice {
                    price: 0.0675364,
                    quantity: 0.00302106,
                    order_id: 1491861965447168,
                },
                BitPrice {
                    price: 0.06754824,
                    quantity: 0.00303074,
                    order_id: 1491862036320261,
                },
                BitPrice {
                    price: 0.0675536,
                    quantity: 76.62,
                    order_id: 1492210266976261,
                },
                BitPrice {
                    price: 0.067554,
                    quantity: 0.00302126,
                    order_id: 1491851063762944,
                },
                BitPrice {
                    price: 0.06755404,
                    quantity: 0.00302109,
                    order_id: 1491861961129987,
                },
                BitPrice {
                    price: 0.06756588,
                    quantity: 0.00303077,
                    order_id: 1491862031593472,
                },
                BitPrice {
                    price: 0.06757164,
                    quantity: 0.00301129,
                    order_id: 1491851063558147,
                },
                BitPrice {
                    price: 0.06757168,
                    quantity: 0.00302112,
                    order_id: 1491861949280261,
                },
                BitPrice {
                    price: 0.06758352,
                    quantity: 0.0030308,
                    order_id: 1491862021210120,
                },
                BitPrice {
                    price: 0.06758636,
                    quantity: 1.0,
                    order_id: 1492087951314945,
                },
                BitPrice {
                    price: 0.06758928,
                    quantity: 0.00301132,
                    order_id: 1491851058843655,
                },
                BitPrice {
                    price: 0.06758932,
                    quantity: 0.00250907,
                    order_id: 1491252066885634,
                },
                BitPrice {
                    price: 0.06760116,
                    quantity: 0.00303083,
                    order_id: 1491277405323266,
                },
                BitPrice {
                    price: 0.06760116,
                    quantity: 0.00303083,
                    order_id: 1491277411885060,
                },
                BitPrice {
                    price: 0.06760692,
                    quantity: 0.00301135,
                    order_id: 1491232484462595,
                },
                BitPrice {
                    price: 0.06760696,
                    quantity: 0.00302118,
                    order_id: 1491235224731648,
                },
                BitPrice {
                    price: 0.0676188,
                    quantity: 0.00303086,
                    order_id: 1491277404602372,
                },
                BitPrice {
                    price: 0.06762456,
                    quantity: 0.00301138,
                    order_id: 1491232379027456,
                },
                BitPrice {
                    price: 0.0676246,
                    quantity: 0.00302121,
                    order_id: 1491234605850625,
                },
                BitPrice {
                    price: 0.06763644,
                    quantity: 0.00303089,
                    order_id: 1491277398564864,
                },
                BitPrice {
                    price: 0.0676422,
                    quantity: 0.00301141,
                    order_id: 1491232345210882,
                },
                BitPrice {
                    price: 0.06764224,
                    quantity: 0.00302124,
                    order_id: 1491234605039617,
                },
                BitPrice {
                    price: 0.06765408,
                    quantity: 0.00302092,
                    order_id: 1491277397913602,
                },
                BitPrice {
                    price: 0.06765984,
                    quantity: 0.00301144,
                    order_id: 1491232319770626,
                },
                BitPrice {
                    price: 0.06765988,
                    quantity: 0.00302127,
                    order_id: 1491234002092033,
                },
                BitPrice {
                    price: 0.06766437,
                    quantity: 1.0,
                    order_id: 1491232574193665,
                },
                BitPrice {
                    price: 0.06767172,
                    quantity: 0.00302095,
                    order_id: 1491277397159937,
                },
                BitPrice {
                    price: 0.06767748,
                    quantity: 0.00301147,
                    order_id: 1491231782932484,
                },
                BitPrice {
                    price: 0.06767752,
                    quantity: 0.0030113,
                    order_id: 1491234000748544,
                },
                BitPrice {
                    price: 0.06768936,
                    quantity: 0.00302098,
                    order_id: 1491277392191489,
                },
                BitPrice {
                    price: 0.06769512,
                    quantity: 0.0030115,
                    order_id: 1491230943768583,
                },
                BitPrice {
                    price: 0.06769516,
                    quantity: 0.00301133,
                    order_id: 1491232552075264,
                },
                BitPrice {
                    price: 0.0676992,
                    quantity: 92.06,
                    order_id: 1492210266959873,
                },
                BitPrice {
                    price: 0.067707,
                    quantity: 0.00302101,
                    order_id: 1491277369573386,
                },
                BitPrice {
                    price: 0.06771276,
                    quantity: 0.00301153,
                    order_id: 1491229700182016,
                },
                BitPrice {
                    price: 0.0677128,
                    quantity: 0.00301136,
                    order_id: 1491232479510528,
                },
                BitPrice {
                    price: 0.06772464,
                    quantity: 0.00302104,
                    order_id: 1491277307379721,
                },
                BitPrice {
                    price: 0.0677304,
                    quantity: 0.00301156,
                    order_id: 1491208548585472,
                },
                BitPrice {
                    price: 0.06773044,
                    quantity: 0.00301139,
                    order_id: 1491232365617153,
                },
                BitPrice {
                    price: 0.06774228,
                    quantity: 0.00302107,
                    order_id: 1491257295532032,
                },
                BitPrice {
                    price: 0.06774804,
                    quantity: 0.00301159,
                    order_id: 1491157651738624,
                },
            ],
        };

        let got = super::make_merged_market_depth(binance, bitstamp);

        let expected = Summary {
            spread: 4.200000000000037e-5,
            bids: vec![
                Level {
                    exchange: "bitstamp".to_string(),
                    price: 0.06712,
                    amount: 0.00521,
                },
                Level {
                    exchange: "bitstamp".to_string(),
                    price: 0.06709444,
                    amount: 0.495,
                },
                Level {
                    exchange: "bitstamp".to_string(),
                    price: 0.06709423,
                    amount: 2.54078178,
                },
                Level {
                    exchange: "bitstamp".to_string(),
                    price: 0.06709148,
                    amount: 5.08133812,
                },
                Level {
                    exchange: "bitstamp".to_string(),
                    price: 0.06708276,
                    amount: 0.495,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067077,
                    amount: 26.5401,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067076,
                    amount: 29.3521,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067075,
                    amount: 2.5403,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067074,
                    amount: 0.2542,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067072,
                    amount: 14.3656,
                },
            ],
            asks: vec![
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067078,
                    amount: 0.3728,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067088,
                    amount: 0.075,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.06709,
                    amount: 1.3444,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067091,
                    amount: 3.1781,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067094,
                    amount: 0.02,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067097,
                    amount: 2.757,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067098,
                    amount: 0.174,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067099,
                    amount: 4.0187,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.0671,
                    amount: 5.6645,
                },
                Level {
                    exchange: "binance".to_string(),
                    price: 0.067101,
                    amount: 0.0971,
                },
            ],
        };
        assert_eq!(got, expected);

        // Also make sure expected is sorted
        // Asks should have the smallest value first
        let mut sorted = expected.asks.clone();
        sorted.sort_by(|a, b| OrderedFloat(a.price).cmp(&OrderedFloat(b.price)));
        assert_eq!(sorted, expected.asks);

        // Bids should be sorted with the largest value first
        let mut sorted = expected.bids.clone();
        sorted.sort_by(|a, b| OrderedFloat(a.price).cmp(&OrderedFloat(b.price)));
        sorted.reverse();
        assert_eq!(sorted, expected.bids);

        // The Spread should be the highest (first) bid - the lowest (first) ask
        let lowest_ask = expected
            .asks
            .iter()
            .map(|level| OrderedFloat(level.price))
            .min()
            .unwrap();
        let highest_bid = expected
            .bids
            .iter()
            .map(|level| OrderedFloat(level.price))
            .max()
            .unwrap();
        let expected_spread = highest_bid - lowest_ask;
        assert_eq!(got.spread, expected_spread.into_inner());
    }
}
