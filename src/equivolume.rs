use crate::oanda::Candle;
use anyhow::{Result, Context};

#[derive(Debug, Clone)]
pub struct EquivolumeBox {
    pub time: String,
    pub volume: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

impl EquivolumeBox {
    pub fn from_candle(candle: &Candle) -> Result<Self> {
        let mid = candle.mid.as_ref().context("Candle missing mid prices")?;
        
        let open: f64 = mid.o.parse().context("Failed to parse open price")?;
        let high: f64 = mid.h.parse().context("Failed to parse high price")?;
        let low: f64 = mid.l.parse().context("Failed to parse low price")?;
        let close: f64 = mid.c.parse().context("Failed to parse close price")?;

        Ok(Self {
            time: candle.time.clone(),
            volume: candle.volume,
            open,
            high,
            low,
            close,
        })
    }
}

pub fn calculate(candles: &[Candle]) -> Result<Vec<EquivolumeBox>> {
    let mut boxes = Vec::with_capacity(candles.len());
    for candle in candles {
        if !candle.complete {
            continue; // Skip incomplete candles if preferred, or include them
        }
        boxes.push(EquivolumeBox::from_candle(candle)?);
    }
    Ok(boxes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oanda::{Candle, Ohlc};

    #[test]
    fn test_calculate_equivolume() {
        let candles = vec![
            Candle {
                time: "2023-01-01T00:00:00Z".to_string(),
                volume: 100,
                complete: true,
                mid: Some(Ohlc {
                    o: "1.1000".to_string(),
                    h: "1.1050".to_string(),
                    l: "1.0950".to_string(),
                    c: "1.1020".to_string(),
                }),
            },
            Candle {
                time: "2023-01-02T00:00:00Z".to_string(),
                volume: 200,
                complete: true,
                mid: Some(Ohlc {
                    o: "1.1020".to_string(),
                    h: "1.1100".to_string(),
                    l: "1.1000".to_string(),
                    c: "1.1080".to_string(),
                }),
            },
        ];

        let result = calculate(&candles).expect("Calculation failed");

        assert_eq!(result.len(), 2);
        
        let box1 = &result[0];
        assert_eq!(box1.volume, 100);
        assert!((box1.high - 1.1050).abs() < 1e-6);
        assert!((box1.low - 1.0950).abs() < 1e-6);

        let box2 = &result[1];
        assert_eq!(box2.volume, 200);
        assert!((box2.high - 1.1100).abs() < 1e-6);
        assert!((box2.low - 1.1000).abs() < 1e-6);
    }
}
