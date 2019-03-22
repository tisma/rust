/// The Computer Language Benchmarks Game
/// https://salsa.debian.org/benchmarksgame-team/benchmarksgame/
///
/// contributed by Mark C. Lewis
/// modified slightly by Chad Whipkey
/// converted from java to c++,added sse support, by Branimir Maksimovic
/// converted from c++ to c, by Alexey Medvedchikov
/// converted from c to Rust by Frank Rehberger

const PI: f64 = 3.141592653589793;
const SOLAR_MASS: f64 = 4.0 * PI * PI;
const YEAR: f64 = 365.24;
const N_BODIES: usize = 5;

use std::arch::x86_64::*;

///
type F64x3 = [f64; 3];

///
#[repr(C)]
#[derive(Clone, Copy)]
struct Planet {
    x: F64x3,
    fill: f64,
    v: F64x3,
    mass: f64,
}

static BODIES: [Planet; N_BODIES] = [
    // Sun
    Planet {
        x: [0.0, 0.0, 0.0],
        fill: 0.0,
        v: [0.0, 0.0, 0.0],
        mass: SOLAR_MASS,
    },
    // Jupiter
    Planet {
        x: [4.84143144246472090e+00,
            -1.16032004402742839e+00,
            -1.03622044471123109e-01],
        fill: 0.0,
        v: [1.66007664274403694e-03 * YEAR,
            7.69901118419740425e-03 * YEAR,
            -6.90460016972063023e-05 * YEAR],
        mass: 9.54791938424326609e-04 * SOLAR_MASS,
    },
    // Saturn
    Planet {
        x: [8.34336671824457987e+00,
            4.12479856412430479e+00,
            -4.03523417114321381e-01],
        fill: 0.0,
        v: [-2.76742510726862411e-03 * YEAR,
            4.99852801234917238e-03 * YEAR,
            2.30417297573763929e-05 * YEAR],
        mass: 2.85885980666130812e-04 * SOLAR_MASS,
    },
    // Uranus
    Planet {
        x: [1.28943695621391310e+01,
            -1.51111514016986312e+01,
            -2.23307578892655734e-01],
        fill: 0.0,
        v: [2.96460137564761618e-03 * YEAR,
            2.37847173959480950e-03 * YEAR,
            -2.96589568540237556e-05 * YEAR],
        mass: 4.36624404335156298e-05 * SOLAR_MASS,
    },
    // Neptune
    Planet {
        x: [1.53796971148509165e+01,
            -2.59193146099879641e+01,
            1.79258772950371181e-01],
        fill: 0.0,
        v: [2.68067772490389322e-03 * YEAR,
            1.62824170038242295e-03 * YEAR,
            -9.51592254519715870e-05 * YEAR],
        mass: 5.15138902046611451e-05 * SOLAR_MASS,
    },
];

///
///
fn offset_momentum(bodies: &mut [Planet; N_BODIES]) {
    for i in 0..bodies.len() {
        for k in 0..3 {
            bodies[0].v[k] -= bodies[i].v[k] * bodies[i].mass / SOLAR_MASS;
        }
    }
}


///
///
fn bodies_energy(bodies: &[Planet; N_BODIES]) -> f64 {
    let mut dx: [f64; 3] = [0.0; 3];
    let mut e = 0.0;

    for i in 0..bodies.len() {
        e += bodies[i].mass * ((bodies[i].v[0] * bodies[i].v[0])
            + (bodies[i].v[1] * bodies[i].v[1])
            + (bodies[i].v[2] * bodies[i].v[2])) / 2.0;

        for j in (i + 1)..bodies.len() {
            for k in 0..3 {
                dx[k] = bodies[i].x[k] - bodies[j].x[k];
            }
            let distance = ((dx[0] * dx[0]) + (dx[1] * dx[1])
                + (dx[2] * dx[2])).sqrt();
            e -= (bodies[i].mass * bodies[j].mass) / distance;
        }
    }

    e
}

///
///
#[repr(C)]
#[derive(Clone, Copy)]
struct Delta {
    dx: F64x3,
    fill: f64,
}

///
///
impl Default for Delta {
    fn default() -> Delta {
        let fill = 0.0;

        Delta {
            dx: [fill, fill, fill],
            fill: fill,
        }
    }
}

///
///
#[repr(C)]
#[repr(align(16))]
#[derive(Clone, Copy)]
struct F64Array([f64; N_BODIES * N_BODIES]);

///
struct NBodySim {
    r: [Delta; N_BODIES * N_BODIES],
    mag: F64Array,
    // intermediate buffer, so all variables might fit into cache together
    dx: [__m128d; 3],
}

///
impl NBodySim {
    pub fn new() -> NBodySim {
        unsafe {
            NBodySim {
                r: [Delta::default(); N_BODIES * N_BODIES],
                mag: F64Array([0.0; N_BODIES * N_BODIES]),
                dx: [_mm_setzero_pd(); 3],
            }
        }
    }


    ///
    ///
    #[inline]
    pub fn bodies_advance(&mut self,
                          bodies: &mut [Planet; N_BODIES],
                          dt: f64, ncycles: usize) {
        let nslots = ((bodies.len() - 1) * bodies.len()) / 2;

        for _ in 0..ncycles {
            // init
            self.dx = unsafe { [_mm_setzero_pd(); 3] };

            let mut k = 0;
            for i in 0..(bodies.len() - 1) {
                for j in (i + 1)..bodies.len() {
                    for m in 0..3 {
                        self.r[k].dx[m] = bodies[i].x[m] - bodies[j].x[m];
                    }
                    k += 1;
                }
            }

            // enumerate in +2 steps
            for i_2 in 0..(nslots / 2) {
                let i = i_2 * 2;

                for m in 0..3 {
                    self.dx[m] = unsafe {
                        _mm_loadl_pd(self.dx[m], &self.r[i].dx[m])
                    };
                    self.dx[m] = unsafe {
                        _mm_loadh_pd(self.dx[m], &self.r[i + 1].dx[m])
                    };
                }

                let dsquared: __m128d = unsafe {
                    _mm_add_pd(_mm_add_pd(_mm_mul_pd(self.dx[0], self.dx[0]),
                                          _mm_mul_pd(self.dx[1], self.dx[1])),
                               _mm_mul_pd(self.dx[2], self.dx[2]))
                };
                let mut distance = unsafe {
                    _mm_cvtps_pd(_mm_rsqrt_ps(_mm_cvtpd_ps(dsquared))) };


                // repeat 2 times
                for _ in 0..2 {
                    let sub = unsafe {
                        _mm_mul_pd(
                            _mm_mul_pd(
                                _mm_mul_pd(_mm_set1_pd(0.5), dsquared),
                                distance),
                            _mm_mul_pd(distance, distance))
                    };

                    distance = unsafe {
                        _mm_sub_pd(
                            _mm_mul_pd(distance, _mm_set1_pd(1.5)),
                            sub)
                    };
                }

                let dmag: __m128d =
                    unsafe { _mm_mul_pd(
                        _mm_div_pd(_mm_set1_pd(dt), dsquared),
                        distance) };

                unsafe { _mm_store_pd(&mut (self.mag.0)[i], dmag); }
            }


            let mut k = 0;
            for i in 0..(bodies.len() - 1) {
                for j in (i + 1)..bodies.len() {
                    for m in 0..3 {
                        bodies[i].v[m] -= (self.r[k].dx[m] * bodies[j].mass)
                            * (self.mag.0)[k];

                        bodies[j].v[m] += (self.r[k].dx[m] * bodies[i].mass)
                            * (self.mag.0)[k];
                    }
                    k += 1;
                }
            }

            for i in 0..bodies.len() {
                for m in 0..3 {
                    bodies[i].x[m] += dt * bodies[i].v[m];
                }
            }
        }
    }
}


fn main() {
    let ncycles = std::env::args_os().nth(1)
        .and_then(|s| s.into_string().ok())
        .and_then(|n| n.parse().ok())
        .unwrap_or(1000);

    let mut bodies = BODIES;
    let mut sim = NBodySim::new();

    offset_momentum(&mut bodies);

    println!("{:.9}", bodies_energy(&bodies));

    sim.bodies_advance(&mut bodies, 0.01, ncycles);

    println!("{:.9}", bodies_energy(&bodies));
}

