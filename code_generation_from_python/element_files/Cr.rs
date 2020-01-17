use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 24,
        name: "Chromium",
        symbol: "Cr",
        mass: 51.9961,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(3.635_f64, 0.007_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(1.660_f64, 0.006_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(1.83_f64, 0.02_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(3.49_f64, 0.02_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(3.05_f64, 0.06_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 42,
                mass: UncertainFloat::new(42.006_43_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 43,
                mass: UncertainFloat::new(42.997_710_f64, 0.000_090_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 44,
                mass: UncertainFloat::new(43.985_47_f64, 0.000_14_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 45,
                mass: UncertainFloat::new(44.979_16_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 46,
                mass: UncertainFloat::new(45.968_362_f64, 0.000_022_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 47,
                mass: UncertainFloat::new(46.962_907_f64, 0.000_015_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 48,
                mass: UncertainFloat::new(47.954_036_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 49,
                mass: UncertainFloat::new(48.951_341_1_f64, 0.000_002_8_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 50,
                mass: UncertainFloat::new(49.946_049_6_f64, 0.000_001_4_f64),
                abundance: UncertainFloat::new(4.345_f64, 0.013_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-4.50_f64, 0.05_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(2.54_f64, 0.06_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(2.54_f64, 0.06_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(15.8_f64, 0.2_f64)),
                }),
            },
            Isotope { 
                mass_number: 51,
                mass: UncertainFloat::new(50.944_771_8_f64, 0.000_001_4_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 52,
                mass: UncertainFloat::new(51.940_511_9_f64, 0.000_001_5_f64),
                abundance: UncertainFloat::new(83.789_f64, 0.018_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.914_f64, 0.015_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(3.042_f64, 0.012_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.042_f64, 0.012_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.76_f64, 0.06_f64)),
                }),
            },
            Isotope { 
                mass_number: 53,
                mass: UncertainFloat::new(52.940_653_8_f64, 0.000_001_5_f64),
                abundance: UncertainFloat::new(9.501_f64, 0.017_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-4.20_f64, 0.03_f64),
                    b_p: Some(UncertainFloat::new(1.16_f64, 0.10_f64)),
                    b_m: Some(UncertainFloat::new(-13.0_f64, 0.2_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(2.22_f64, 0.03_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(5.93_f64, 0.17_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(8.15_f64, 0.17_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(18.1_f64, 1.5_f64)),
                }),
            },
            Isotope { 
                mass_number: 54,
                mass: UncertainFloat::new(53.938_884_9_f64, 0.000_001_5_f64),
                abundance: UncertainFloat::new(2.365_f64, 0.007_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.55_f64, 0.10_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(2.60_f64, 0.11_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(2.60_f64, 0.11_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.36_f64, 0.04_f64)),
                }),
            },
            Isotope { 
                mass_number: 55,
                mass: UncertainFloat::new(54.940_844_2_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 56,
                mass: UncertainFloat::new(55.940_645_f64, 0.000_010_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 57,
                mass: UncertainFloat::new(56.943_75_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 58,
                mass: UncertainFloat::new(57.944_25_f64, 0.000_26_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 59,
                mass: UncertainFloat::new(58.948_63_f64, 0.000_27_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 60,
                mass: UncertainFloat::new(59.949_73_f64, 0.000_28_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 61,
                mass: UncertainFloat::new(60.954_09_f64, 0.000_30_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 62,
                mass: UncertainFloat::new(61.955_80_f64, 0.000_40_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 63,
                mass: UncertainFloat::new(62.961_86_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 64,
                mass: UncertainFloat::new(63.964_20_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 65,
                mass: UncertainFloat::new(64.970_37_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
