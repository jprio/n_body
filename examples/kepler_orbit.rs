/*
Ref :  https://srenevey.github.io/ode-solvers/examples/kepler_orbit.html
 */

use ode_solvers::dopri5::*;
use ode_solvers::*;

type State = Vector6<f64>;
type Time = f64;

struct KeplerOrbit {
    mu: f64,
}

impl ode_solvers::System<State> for KeplerOrbit {
    // Equations of motion of the system
    fn system(&self, _t: Time, y: &State, dy: &mut State) {
        let r = (y[0] * y[0] + y[1] * y[1] + y[2] * y[2]).sqrt();

        dy[0] = y[3];
        dy[1] = y[4];
        dy[2] = y[5];
        dy[3] = (-self.mu * y[0]) / r.powi(3);
        dy[4] = (-self.mu * y[1]) / r.powi(3);
        dy[5] = (-self.mu * y[2]) / r.powi(3);
    }
}

fn main() {
    let system = KeplerOrbit { mu: 398600.435436 };

    let a: f64 = 20000.0;
    let period = 2.0 * PI * (a.powi(3) / system.mu).sqrt();
    let y0 = State::new(
        -5007.248417988539,
        -1444.918140151374,
        3628.534606178356,
        0.717716656891,
        -10.224093784269,
        0.748229399696
    );

    let mut stepper = Dopri5::new(system, 0.0, 5.0 * period, 60.0, y0, 1.0e-10, 1.0e-10);
    let res = stepper.integrate();

    // Handle result
    match res {
        Ok(stats) => {
            stats.print();

            // Do something with the output...
            // let path = Path::new("./outputs/kepler_orbit_dopri5.dat");
            // save(stepper.x_out(), stepper.y_out(), path);
            // println!("Results saved in: {:?}", path);
        }
        Err(_) => println!("An error occured."),
    }
}