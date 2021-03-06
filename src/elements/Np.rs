use crate::Element;
use crate::{UncertainFloat, Isotope, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 93,
        name: "Neptunium",
        symbol: "Np",
        mass: 237.0_f64,
        common_ions: vec![5],
        uncommon_ions: vec![2, 3, 4, 6, 7],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(10.55_f64, 0.10_f64),
            b_p: None,
            b_m: None,
            bound_coherent_scattering_xs: Some(UncertainFloat::new(14.0_f64, 0.3_f64)),
            bound_incoherent_scattering_xs: Some(UncertainFloat::new(0.5_f64, 0.5_f64)),
            total_bound_scattering_xs: Some(UncertainFloat::new(14.5_f64, 0.6_f64)),
            absorption_xs: Some(UncertainFloat::new(175.9_f64, 2.9_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 225,
                mass: UncertainFloat::new(225.033_9_f64, 0.008_0_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 226,
                mass: UncertainFloat::new(226.035_13_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 227,
                mass: UncertainFloat::new(227.034_96_f64, 0.000_80_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 228,
                mass: UncertainFloat::new(228.036_18_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 229,
                mass: UncertainFloat::new(229.036_25_f64, 0.000_90_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 230,
                mass: UncertainFloat::new(230.037_81_f64, 0.000_60_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 231,
                mass: UncertainFloat::new(231.038_23_f64, 0.000_50_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 232,
                mass: UncertainFloat::new(232.040_1_f64, 0.001_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 233,
                mass: UncertainFloat::new(233.040_73_f64, 0.000_60_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 234,
                mass: UncertainFloat::new(234.042_889_f64, 0.000_009_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 235,
                mass: UncertainFloat::new(235.044_055_9_f64, 0.000_002_3_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 236,
                mass: UncertainFloat::new(236.046_56_f64, 0.000_50_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 237,
                mass: UncertainFloat::new(237.048_167_3_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 238,
                mass: UncertainFloat::new(238.050_940_5_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 239,
                mass: UncertainFloat::new(239.052_931_4_f64, 0.000_002_3_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 240,
                mass: UncertainFloat::new(240.056_169_f64, 0.000_016_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 241,
                mass: UncertainFloat::new(241.058_25_f64, 0.000_80_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 242,
                mass: UncertainFloat::new(242.061_64_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 243,
                mass: UncertainFloat::new(243.064_27_f64, 0.000_30_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 244,
                mass: UncertainFloat::new(244.067_85_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
