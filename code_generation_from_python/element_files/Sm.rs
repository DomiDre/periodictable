use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 62,
        name: "Samarium",
        symbol: "Sm",
        mass: 150.36,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(0.00_f64, 0.05_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(0.422_f64, 0.009_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(39.0_f64, 3.0_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(39.4_f64, 3.0_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(5922.0_f64, 56.0_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 130,
                mass: UncertainFloat::new(129.948_63_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 131,
                mass: UncertainFloat::new(130.945_89_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 132,
                mass: UncertainFloat::new(131.940_82_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 133,
                mass: UncertainFloat::new(132.938_73_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 134,
                mass: UncertainFloat::new(133.934_02_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 135,
                mass: UncertainFloat::new(134.932_35_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 136,
                mass: UncertainFloat::new(135.928_30_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 137,
                mass: UncertainFloat::new(136.927_05_f64, 0.000_12_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 138,
                mass: UncertainFloat::new(137.923_54_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 139,
                mass: UncertainFloat::new(138.922_302_f64, 0.000_016_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 140,
                mass: UncertainFloat::new(139.918_991_f64, 0.000_016_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 141,
                mass: UncertainFloat::new(140.918_469_f64, 0.000_013_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 142,
                mass: UncertainFloat::new(141.915_193_f64, 0.000_011_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 143,
                mass: UncertainFloat::new(142.914_624_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 144,
                mass: UncertainFloat::new(143.911_995_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(3.07_f64, 0.07_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-3.0_f64, 4.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(1.0_f64, 3.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.0_f64, 3.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.7_f64, 0.3_f64)),
                }),
            },
            Isotope { 
                mass_number: 145,
                mass: UncertainFloat::new(144.913_406_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 146,
                mass: UncertainFloat::new(145.913_037_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 147,
                mass: UncertainFloat::new(146.914_893_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(14.99_f64, 0.18_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(14.0_f64, 3.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(25.0_f64, 11.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(14.0_f64, 19.0._f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(39.0_f64, 16.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(57.0_f64, 3.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 148,
                mass: UncertainFloat::new(147.914_818_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(11.24_f64, 0.10_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-3.0_f64, 4.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(1.0_f64, 3.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.0_f64, 3.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.4_f64, 0.6_f64)),
                }),
            },
            Isotope { 
                mass_number: 149,
                mass: UncertainFloat::new(148.917_180_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(13.82_f64, 0.07_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(18.7_f64, 0.28_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(63.5_f64, 0.6_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(137.0_f64, 5.0_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(200.0_f64, 5.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(42080.0_f64, 400.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 150,
                mass: UncertainFloat::new(149.917_271_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(7.38_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(14.0_f64, 3.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(25.0_f64, 11.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(25.0_f64, 11.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(104.0_f64, 4.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 151,
                mass: UncertainFloat::new(150.919_928_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 152,
                mass: UncertainFloat::new(151.919_728_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(26.75_f64, 0.16_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-5.0_f64, 0.6_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(3.1_f64, 0.8_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.1_f64, 0.8_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(206.0_f64, 6.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 153,
                mass: UncertainFloat::new(152.922_094_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 154,
                mass: UncertainFloat::new(153.922_205_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(22.75_f64, 0.29_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(8.0_f64, 1.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(11.0_f64, 2.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(11.0_f64, 2.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(8.4_f64, 0.5_f64)),
                }),
            },
            Isotope { 
                mass_number: 155,
                mass: UncertainFloat::new(154.924_636_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 156,
                mass: UncertainFloat::new(155.925_526_f64, 0.000_010_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 157,
                mass: UncertainFloat::new(156.928_350_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 158,
                mass: UncertainFloat::new(157.929_990_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 159,
                mass: UncertainFloat::new(158.933_20_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 160,
                mass: UncertainFloat::new(159.935_14_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 161,
                mass: UncertainFloat::new(160.938_83_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 162,
                mass: UncertainFloat::new(161.941_22_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 163,
                mass: UncertainFloat::new(162.945_36_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 164,
                mass: UncertainFloat::new(163.948_28_f64, 0.000_86_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 165,
                mass: UncertainFloat::new(164.952_98_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
