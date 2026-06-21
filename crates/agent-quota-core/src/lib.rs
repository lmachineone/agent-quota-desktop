#[derive(Debug, Clone, PartialEq)]
pub struct RateWindow {
    pub used_percent: f64,
    pub window_minutes: Option<u32>,
    pub reset_description: Option<String>,
}

impl RateWindow {
    pub fn remaining_percent(&self) -> f64 {
        (100.0 - self.used_percent).clamp(0.0, 100.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct UsageSnapshot {
    pub provider: ProviderId,
    pub primary: Option<RateWindow>,
    pub secondary: Option<RateWindow>,
    pub source_label: String,
}

pub fn product_positioning() -> &'static str {
    "Every agent quota. Every desktop. One control plane."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remaining_percent_is_clamped() {
        let depleted = RateWindow {
            used_percent: 125.0,
            window_minutes: Some(300),
            reset_description: None,
        };
        assert_eq!(depleted.remaining_percent(), 0.0);

        let impossible_credit = RateWindow {
            used_percent: -10.0,
            window_minutes: None,
            reset_description: None,
        };
        assert_eq!(impossible_credit.remaining_percent(), 100.0);
    }
}
