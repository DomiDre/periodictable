use crate::Element;
use crate::{UncertainFloat, Isotope, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 96,
        name: "Curium",
        symbol: "Cm",
        mass: 247.0_f64,
        common_ions: vec![3],
        uncommon_ions: vec![4, 6],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(9.5_f64, 0.3_f64),
            b_p: None,
            b_m: None,
            bound_coherent_scattering_xs: Some(UncertainFloat::new(11.3_f64, 0.7_f64)),
            bound_incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
            total_bound_scattering_xs: Some(UncertainFloat::new(11.3_f64, 0.7_f64)),
            absorption_xs: Some(UncertainFloat::new(16.2_f64, 1.2_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 233,
                mass: UncertainFloat::new(233.050_8_f64, 0.004_3_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 234,
                mass: UncertainFloat::new(234.050_24_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 235,
                mass: UncertainFloat::new(235.051_59_f64, 0.000_24_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 236,
                mass: UncertainFloat::new(236.051_41_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 237,
                mass: UncertainFloat::new(237.052_89_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 238,
                mass: UncertainFloat::new(238.053_02_f64, 0.000_40_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 239,
                mass: UncertainFloat::new(239.054_95_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 240,
                mass: UncertainFloat::new(240.055_519_f64, 0.000_029_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 241,
                mass: UncertainFloat::new(241.057_646_7_f64, 0.000_002_4_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 242,
                mass: UncertainFloat::new(242.058_829_3_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 243,
                mass: UncertainFloat::new(243.061_382_2_f64, 0.000_002_4_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 244,
                mass: UncertainFloat::new(244.062_746_3_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 245,
                mass: UncertainFloat::new(245.065_485_6_f64, 0.000_002_9_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 246,
                mass: UncertainFloat::new(246.067_217_6_f64, 0.000_002_4_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(9.3_f64, 0.2_f64),
                    b_p: None,
                    b_m: None,
                    bound_coherent_scattering_xs: Some(UncertainFloat::new(10.9_f64, 0.5_f64)),
                    bound_incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    total_bound_scattering_xs: Some(UncertainFloat::new(10.9_f64, 0.5_f64)),
                    absorption_xs: Some(UncertainFloat::new(1.36_f64, 0.17_f64)),
                }),
            },
            Isotope { 
                mass_number: 247,
                mass: UncertainFloat::new(247.070_347_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 248,
                mass: UncertainFloat::new(248.072_342_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.7_f64, 0.2_f64),
                    b_p: None,
                    b_m: None,
                    bound_coherent_scattering_xs: Some(UncertainFloat::new(7.5_f64, 0.4_f64)),
                    bound_incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    total_bound_scattering_xs: Some(UncertainFloat::new(7.5_f64, 0.4_f64)),
                    absorption_xs: Some(UncertainFloat::new(3.0_f64, 0.26_f64)),
                }),
            },
            Isotope { 
                mass_number: 249,
                mass: UncertainFloat::new(249.075_947_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 250,
                mass: UncertainFloat::new(250.078_351_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 251,
                mass: UncertainFloat::new(251.082_278_f64, 0.000_024_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 252,
                mass: UncertainFloat::new(252.084_87_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
