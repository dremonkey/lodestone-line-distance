
// Third party crates
extern crate lodestone_distance;
extern crate lodestone_linestring;
extern crate lodestone_point;

use lodestone_distance::distance;
use lodestone_linestring::FeatureLineString;
use lodestone_point::FeaturePoint;

pub trait LineDistance {
  fn distance(&self, units: &str) -> f64;
}

impl LineDistance for FeatureLineString {
  fn distance(&self, units: &str) -> f64 {
    
    let mut traveled = 0.0;
    let mut coords = self.coordinates();
    let mut prev = coords.remove(0);

    for coord in coords {
      let pt1 = FeaturePoint::new(prev);
      let pt2 = FeaturePoint::new(coord.clone());
      traveled += distance(&pt1, &pt2, &units);

      // prepare for next iteration
      prev = coord;
    }
    
    traveled
  }
}

#[cfg(test)]
mod tests {
  use lodestone_linestring::FeatureLineString;
  use super::LineDistance;

  #[test]
  fn test_la_sf_ny() {
    let la = vec![-118.2500,34.0500];
    let ny = vec![-74.0059,40.7127];
    let sf = vec![-122.4167,37.7833];
    let line = FeatureLineString::new(vec![la, sf, ny]);

    assert_eq!(line.distance("km"), 4693.2446126025625);
    assert_eq!(line.distance("mi"), 2916.2460981774666);
  }
}
