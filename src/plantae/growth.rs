#[derive(Debug)]
pub struct GrowthModel {
    pub tick_mid: f64,
    pub tick_end: f64,
    pub max_weight: f64,
    pub max_growth_rate: f64,
}

impl GrowthModel {
    pub fn new(tick_mid: f64, tick_end: f64, max_weight: f64) -> GrowthModel {
        let max_growth_rate = GrowthModel::max_growth_rate(tick_mid, tick_end, max_weight);

        GrowthModel {
            tick_mid,
            tick_end,
            max_weight,
            max_growth_rate,
        }
    }

    pub fn max_growth_rate(tick_mid: f64, tick_end: f64, max_weight: f64) -> f64 {
        let a = (2. * tick_end) - tick_mid;
        let b = tick_end * (tick_end - tick_mid);
        let c = tick_mid / tick_end;
        let d = tick_mid / (tick_end - tick_mid);

        return (a / b) * c.powf(d) * max_weight;
    }

    pub fn weight(&self, tick: f64) -> f64 {
        let a = 1. + ((self.tick_end - tick) / (self.tick_end - self.tick_mid));
        let b = tick / self.tick_end;
        let c = self.tick_end / (self.tick_end - self.tick_mid);

        return self.max_weight * a * b.powf(c);
    }

    pub fn relative_growth_rate(&self, tick: f64) -> f64 {
        let weight = self.weight(tick);
        let a = 2. * self.tick_end - self.tick_mid;
        let b = self.tick_end - tick;
        let c = self.tick_end - self.tick_mid;
        let d = 2. * self.tick_end - self.tick_mid - tick;

        return weight * ((a * b) / (c * d * tick));
    }
}
