// (c) 2017 Joost Yervante Damad <joost@damad.be>

pub struct Plato(pub f64);
pub struct Sg(pub f64);

// formula from https://www.brewersfriend.com/plato-to-sg-conversion-chart/
impl From<Sg> for Plato {
    fn from(p: Sg) -> Plato {
        let sg = p.0;
        let p = (-1.0 * 616.868) + (1111.14 * sg) - (630.272 * sg * sg) + (135.997 * sg * sg * sg);
        Plato(p)
    }
}

// formula from https://www.brewersfriend.com/plato-to-sg-conversion-chart/
impl From<Plato> for Sg {
    fn from(p: Plato) -> Sg {
        let plato = p.0;
        let sg = 1.0 + (plato / (258.6 - ((plato / 258.2) * 227.1)));
        Sg(sg)
    }
}

// from https://www.brewersfriend.com/2011/06/16/alcohol-by-volume-calculator-updated/

impl Sg {
    pub fn abv(self, og: &Sg) -> f64 {
        let og = og.0;
        let fg = self.0;
        (76.08 * (og - fg) / (1.775 - og)) * (fg / 0.794)
    }
}

// assume bigger then 1.2 is Plato
impl From<f64> for Sg {
    fn from(x: f64) -> Sg {
        if x > 1.2 {
            Plato(x).into()
        } else {
            Sg(x)
        }
    }
}
