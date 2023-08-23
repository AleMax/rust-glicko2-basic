const SYSTEM_CONSTANT: f64 = 0.9;

const CONVERGENCE_TOLERANCE: f64 = 0.00000001;

const DEFAULT_RATING: f64 = 0.0;
const DEFAULT_RATING_DEVIATION: f64 = 350.0 / 173.7178;
const DEFAULT_VOLATILITY: f64 = 0.06;

pub struct Glicko2Competitor {
    rating: f64, // Rating
    rating_deviation: f64, // Rating Deviation
    volatility: f64, // Votality
}

impl Glicko2Competitor {
    pub fn new() -> Self {
        Glicko2Competitor {
            rating: DEFAULT_RATING,
            rating_deviation: DEFAULT_RATING_DEVIATION,
            volatility: DEFAULT_VOLATILITY,
        }
    }

    pub fn from_values(rating: f64, rating_deviation: f64, volatility: f64) -> Self {
        Glicko2Competitor {
            rating: rating,
            rating_deviation: rating_deviation,
            volatility: volatility,
        }
    }

    pub fn from_values_elo(rating: f64, rating_deviation: f64, volatility: f64) -> Self {
        Glicko2Competitor {
            rating: (rating - 1500.0) / 173.7178,
            rating_deviation: rating_deviation / 173.7178,
            volatility: volatility,
        }
    }

}


pub struct Glicko2Instance {
    competitor: Glicko2Competitor,
    
    opponents: Vec<(Glicko2Competitor, f64)> // Opponent, and Score: 1.0 = win, 0.5 = draw, 0.0 = loss
}

impl Glicko2Instance {

    pub fn new(competitor: Glicko2Competitor) -> Self {
        Glicko2Instance {
            competitor: competitor,
            opponents: Vec::new()
        }
    }

    pub fn add_opponent(&mut self, opponent: (Glicko2Competitor, f64)) {
        self.opponents.push(opponent);
    }

    pub fn compute_rating(&mut self) {
        //println!("Glicko-2 Scale: {:?}", (self.rating, self.rating_deviation, self.volatility));
        let quantity_v = self.compute_quantity_v();
        //println!("Quantity V: {}", quantity_v);
        let quantity_delta = self.compute_quantity_delta(quantity_v);
        //println!("Quantity Delta: {}", quantity_delta);
        self.compute_new_volatility(quantity_v, quantity_delta);
        //println!("New Volatility: {}", self.volatility);
        let new_pre_period_rating_deviation = self.compute_new_pre_period_rating_deviation();
        //println!("New Pre Period Rating Deviation: {}", new_pre_period_rating_deviation);
        self.compute_new_rating_deviation(new_pre_period_rating_deviation, quantity_v);
        //println!("New Rating Deviation: {}", self.rating_deviation);
        self.compute_new_rating();
        //println!("New Rating: {}", self.rating);
    }

    fn compute_quantity_v(&self) -> f64 {
        let mut v: f64 = 0.0;
        for opponent in &self.opponents {
            v += Glicko2Instance::fun_g(opponent.0.rating_deviation).powi(2) * Glicko2Instance::fun_e(self.competitor.rating, opponent.0.rating, opponent.0.rating_deviation) * (1.0 - Glicko2Instance::fun_e(self.competitor.rating, opponent.0.rating, opponent.0.rating_deviation))
        }
        v.recip()
    }

    fn compute_quantity_delta(&self, quantity_v: f64) -> f64 {
        let mut delta: f64 = 0.0;
        for opponent in &self.opponents {
            delta += Glicko2Instance::fun_g(opponent.0.rating_deviation) * (opponent.1 - Glicko2Instance::fun_e(self.competitor.rating, opponent.0.rating, opponent.0.rating_deviation))
        }
        quantity_v * delta
    }

    fn compute_new_volatility(&mut self, quantity_v: f64, quantity_delta: f64) {
        let a: f64 = self.competitor.volatility.powi(2).ln();

        let mut var_a: f64 = a;
        let mut var_b: f64;

        if quantity_delta.powi(2) > (self.competitor.rating_deviation.powi(2) + quantity_v) {

            var_b = (quantity_delta.powi(2) - self.competitor.rating_deviation.powi(2) - quantity_v).ln();

        } else {

            let mut k: u32 = 1;
            if Glicko2Instance::fun_f(a - (k as f64) * SYSTEM_CONSTANT, quantity_delta, self.competitor.rating_deviation, quantity_v, a) < 0.0 {
                k += 1;
            }
            var_b = a - (k as f64) * SYSTEM_CONSTANT;

        }

        let mut f_a: f64 = Glicko2Instance::fun_f(var_a, quantity_delta, self.competitor.rating_deviation, quantity_v, a);
        let mut f_b: f64 = Glicko2Instance::fun_f(var_b, quantity_delta, self.competitor.rating_deviation, quantity_v, a);

        while (var_b - var_a).abs() > CONVERGENCE_TOLERANCE {
            let var_c: f64 = var_a + (var_a - var_b) * f_a / (f_b - f_a);
            let f_c: f64 = Glicko2Instance::fun_f(var_c, quantity_delta, self.competitor.rating_deviation, quantity_v, a);

            if f_c * f_b < 0.0 {
                var_a = var_b;
                f_a = f_b;
            } else {
                f_a = f_a / 2.0;
            }
            var_b = var_c;
            f_b = f_c;
        }
        
        self.competitor.volatility = (var_a / 2.0).exp();
    }

    fn compute_new_pre_period_rating_deviation(&self) -> f64 {
        (self.competitor.rating_deviation.powi(2) + self.competitor.volatility.powi(2)).sqrt()
    }

    fn compute_new_rating_deviation(&mut self, new_pre_period_rating_deviation: f64, quantity_v: f64) {
        self.competitor.rating_deviation = 1.0 / ( 1.0 / new_pre_period_rating_deviation.powi(2) + 1.0 / quantity_v).sqrt();
    }

    fn compute_new_rating(&mut self) {
        let mut sum: f64 = 0.0;
        for opponent in &self.opponents {
            sum += Glicko2Instance::fun_g(opponent.0.rating_deviation) * (opponent.1 - Glicko2Instance::fun_e(self.competitor.rating, opponent.0.rating, opponent.0.rating_deviation))
        }
        self.competitor.rating = self.competitor.rating + self.competitor.rating_deviation.powi(2) * sum;
    }

    fn fun_g(rating_deviation: f64) -> f64 {
        1.0 / (1.0 + 3.0 * rating_deviation * rating_deviation / std::f64::consts::PI / std::f64::consts::PI).sqrt()
    }

    fn fun_e(rating: f64, rating_opponent: f64, rating_deviation_opponent: f64) -> f64 {
        1.0 / (1.0 + ( -Glicko2Instance::fun_g(rating_deviation_opponent) * (rating - rating_opponent) ).exp() )
    }

    fn fun_f(x: f64, delta: f64, rating_deviation: f64, v: f64, a: f64) -> f64 {
        x.exp() * (delta.powi(2) - rating_deviation.powi(2) - v - x.exp()) / 2.0 / (rating_deviation.powi(2) + v + x.exp()).powi(2) - (x - a) / SYSTEM_CONSTANT.powi(2)
    }

    pub fn get_rating(&self) -> (f64, f64, f64) {
        (173.7178 * self.competitor.rating + 1500.0, 173.7178 * self.competitor.rating_deviation, self.competitor.volatility)
    }

    pub fn get_winning_probability(&self, opponent: Glicko2Instance) -> f64 {
        1.0 / (1.0 + ( -Glicko2Instance::fun_g( ( self.competitor.rating_deviation.powi(2) + opponent.competitor.rating_deviation.powi(2) ).sqrt() ) * (self.competitor.rating - opponent.competitor.rating) ).exp() )
    }

}