use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 22,
        name: "Titanium",
        symbol: "Ti",
        mass: 47.867,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(-3.370_f64, 0.013_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(1.485_f64, 0.002_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(2.87_f64, 0.03_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(4.35_f64, 0.03_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(6.09_f64, 0.13_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 38,
                mass: UncertainFloat::new(38.009_77_f64, 0.000_27_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 39,
                mass: UncertainFloat::new(39.001_32_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 40,
                mass: UncertainFloat::new(39.990_50_f64, 0.000_17_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 41,
                mass: UncertainFloat::new(40.983_130_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 42,
                mass: UncertainFloat::new(41.973_032_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 43,
                mass: UncertainFloat::new(42.968_523_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 44,
                mass: UncertainFloat::new(43.959_690_2_f64, 0.000_000_8_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 45,
                mass: UncertainFloat::new(44.958_124_3_f64, 0.000_001_3_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 46,
                mass: UncertainFloat::new(45.952_629_5_f64, 0.000_001_2_f64),
                abundance: UncertainFloat::new(8.25_f64, 0.03_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.72_f64, 0.05_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(3.05_f64, 0.07_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.05_f64, 0.07_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.59_f64, 0.18_f64)),
                }),
            },
            Isotope { 
                mass_number: 47,
                mass: UncertainFloat::new(46.951_763_8_f64, 0.000_001_0_f64),
                abundance: UncertainFloat::new(7.44_f64, 0.02_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(3.53_f64, 0.07_f64),
                    b_p: Some(UncertainFloat::new(0.46_f64, 0.23_f64)),
                    b_m: Some(UncertainFloat::new(7.64_f64, 0.13_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(1.66_f64, 0.11_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(1.5_f64, 0.2_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.2_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.7_f64, 0.2_f64)),
                }),
            },
            Isotope { 
                mass_number: 48,
                mass: UncertainFloat::new(47.947_947_1_f64, 0.000_001_0_f64),
                abundance: UncertainFloat::new(73.72_f64, 0.03_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-5.86_f64, 0.02_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(4.65_f64, 0.03_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(4.65_f64, 0.03_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(7.84_f64, 0.25_f64)),
                }),
            },
            Isotope { 
                mass_number: 49,
                mass: UncertainFloat::new(48.947_870_8_f64, 0.000_001_0_f64),
                abundance: UncertainFloat::new(5.41_f64, 0.02_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(0.98_f64, 0.05_f64),
                    b_p: Some(UncertainFloat::new(2.6_f64, 0.3_f64)),
                    b_m: Some(UncertainFloat::new(-1.2_f64, 0.4_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(0.14_f64, 0.01_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(3.3_f64, 0.3_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.4_f64, 0.3_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.2_f64, 0.3_f64)),
                }),
            },
            Isotope { 
                mass_number: 50,
                mass: UncertainFloat::new(49.944_792_1_f64, 0.000_001_1_f64),
                abundance: UncertainFloat::new(5.18_f64, 0.02_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.88_f64, 0.10_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(4.80_f64, 0.12_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(4.80_f64, 0.12_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.179_f64, 0.003_f64)),
                }),
            },
            Isotope { 
                mass_number: 51,
                mass: UncertainFloat::new(50.946_616_0_f64, 0.000_001_4_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 52,
                mass: UncertainFloat::new(51.946_898_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 53,
                mass: UncertainFloat::new(52.949_73_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 54,
                mass: UncertainFloat::new(53.950_87_f64, 0.000_25_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 55,
                mass: UncertainFloat::new(54.955_12_f64, 0.000_26_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 56,
                mass: UncertainFloat::new(55.957_99_f64, 0.000_30_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 57,
                mass: UncertainFloat::new(56.962_90_f64, 0.001_00_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 58,
                mass: UncertainFloat::new(57.966_11_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 59,
                mass: UncertainFloat::new(58.971_96_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 60,
                mass: UncertainFloat::new(59.975_64_f64, 0.000_86_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 61,
                mass: UncertainFloat::new(60.982_02_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
